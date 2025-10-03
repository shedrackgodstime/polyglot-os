//! Terminal emulator with text rendering

use limine::framebuffer::Framebuffer;
use super::font::{self, FONT_WIDTH, FONT_HEIGHT};

/// Terminal colors
pub const BLACK: u32 = 0x000000;
pub const WHITE: u32 = 0xFFFFFF;
pub const GREEN: u32 = 0x00FF00;
pub const BLUE: u32 = 0x0000FF;
pub const RED: u32 = 0xFF0000;

/// Terminal state
pub struct Terminal {
    framebuffer: &'static Framebuffer<'static>,
    width_chars: usize,
    height_chars: usize,
    cursor_x: usize,
    cursor_y: usize,
    fg_color: u32,
    bg_color: u32,
}

impl Terminal {
    /// Create a new terminal
    pub fn new(framebuffer: &'static Framebuffer<'static>) -> Self {
        let width_chars = framebuffer.width() as usize / FONT_WIDTH;
        let height_chars = framebuffer.height() as usize / FONT_HEIGHT;
        
        Self {
            framebuffer,
            width_chars,
            height_chars,
            cursor_x: 0,
            cursor_y: 0,
            fg_color: WHITE,
            bg_color: BLACK,
        }
    }
    
    /// Get terminal dimensions in characters
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width_chars, self.height_chars)
    }
    
    /// Set cursor position
    pub fn set_cursor(&mut self, x: usize, y: usize) {
        if x < self.width_chars && y < self.height_chars {
            self.cursor_x = x;
            self.cursor_y = y;
        }
    }
    
    /// Clear the screen
    pub fn clear(&mut self) {
        let fb_ptr = self.framebuffer.addr() as *mut u32;
        let width = self.framebuffer.width() as usize;
        let height = self.framebuffer.height() as usize;
        let pitch = self.framebuffer.pitch() as usize / 4; // Convert to u32 pitch
        
        unsafe {
            for y in 0..height {
                for x in 0..width {
                    let offset = y * pitch + x;
                    *fb_ptr.add(offset) = self.bg_color;
                }
            }
        }
        
        self.cursor_x = 0;
        self.cursor_y = 0;
    }
    
    /// Write a string to the terminal
    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_char(byte);
        }
    }
    
    /// Write a single character
    pub fn write_char(&mut self, c: u8) {
        match c {
            b'\n' => {
                self.cursor_x = 0;
                self.cursor_y += 1;
                if self.cursor_y >= self.height_chars {
                    self.scroll_up();
                    self.cursor_y = self.height_chars - 1;
                }
            }
            b'\r' => {
                self.cursor_x = 0;
            }
            b'\t' => {
                // Tab to next 4-character boundary
                self.cursor_x = (self.cursor_x + 4) & !3;
                if self.cursor_x >= self.width_chars {
                    self.cursor_x = 0;
                    self.cursor_y += 1;
                    if self.cursor_y >= self.height_chars {
                        self.scroll_up();
                        self.cursor_y = self.height_chars - 1;
                    }
                }
            }
            b'\x08' => {
                // Backspace
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                    self.draw_char_at(self.cursor_x, self.cursor_y, b' ');
                }
            }
            c if c >= 32 && c <= 126 => {
                // Printable character
                self.draw_char_at(self.cursor_x, self.cursor_y, c);
                self.cursor_x += 1;
                if self.cursor_x >= self.width_chars {
                    self.cursor_x = 0;
                    self.cursor_y += 1;
                    if self.cursor_y >= self.height_chars {
                        self.scroll_up();
                        self.cursor_y = self.height_chars - 1;
                    }
                }
            }
            _ => {
                // Ignore other control characters
            }
        }
    }
    
    /// Draw a character at specific position
    fn draw_char_at(&mut self, x: usize, y: usize, c: u8) {
        if x >= self.width_chars || y >= self.height_chars {
            return;
        }
        
        let char_data = font::get_char_data(c);
        let fb_ptr = self.framebuffer.addr() as *mut u32;
        let pitch = self.framebuffer.pitch() as usize / 4; // Convert to u32 pitch
        
        let start_x = x * FONT_WIDTH;
        let start_y = y * FONT_HEIGHT;
        
        unsafe {
            for row in 0..FONT_HEIGHT {
                let font_row = char_data[row];
                for col in 0..FONT_WIDTH {
                    let pixel_x = start_x + col;
                    let pixel_y = start_y + row;
                    let offset = pixel_y * pitch + pixel_x;
                    
                    let color = if (font_row & (0x80 >> col)) != 0 {
                        self.fg_color
                    } else {
                        self.bg_color
                    };
                    
                    *fb_ptr.add(offset) = color;
                }
            }
        }
    }
    
    /// Scroll the screen up by one line
    fn scroll_up(&mut self) {
        let fb_ptr = self.framebuffer.addr() as *mut u32;
        let pitch = self.framebuffer.pitch() as usize / 4;
        let width = self.framebuffer.width() as usize;
        
        unsafe {
            // Move all lines up by FONT_HEIGHT pixels
            for y in FONT_HEIGHT..(self.height_chars * FONT_HEIGHT) {
                for x in 0..width {
                    let src_offset = y * pitch + x;
                    let dst_offset = (y - FONT_HEIGHT) * pitch + x;
                    *fb_ptr.add(dst_offset) = *fb_ptr.add(src_offset);
                }
            }
            
            // Clear the last line
            let last_line_start = (self.height_chars - 1) * FONT_HEIGHT;
            for y in last_line_start..(self.height_chars * FONT_HEIGHT) {
                for x in 0..width {
                    let offset = y * pitch + x;
                    *fb_ptr.add(offset) = self.bg_color;
                }
            }
        }
    }
    
    /// Set foreground color
    pub fn set_fg_color(&mut self, color: u32) {
        self.fg_color = color;
    }
    
    /// Set background color
    pub fn set_bg_color(&mut self, color: u32) {
        self.bg_color = color;
    }
}
