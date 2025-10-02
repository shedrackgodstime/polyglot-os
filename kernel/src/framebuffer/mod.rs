/// Framebuffer graphics driver
/// Provides functions to draw to the screen

use limine::response::FramebufferResponse;

/// Draw graphics to the framebuffer
pub fn draw(framebuffer_response: &FramebufferResponse) {
    if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
        crate::serial::write_str("Drawing to framebuffer...\n");
        
        let fb_ptr = framebuffer.addr() as *mut u32;
        let width = framebuffer.width() as usize;
        let height = framebuffer.height() as usize;
        let pitch = framebuffer.pitch() as usize;
        
        unsafe {
            // Fill screen with dark blue background
            for y in 0..height {
                for x in 0..width {
                    let offset = y * (pitch / 4) + x;
                    *fb_ptr.add(offset) = 0x001a1a2e; // Dark blue
                }
            }
            
            // Draw a large white rectangle in the center
            let rect_width = 600;
            let rect_height = 400;
            let rect_x = (width - rect_width) / 2;
            let rect_y = (height - rect_height) / 2;
            
            for y in rect_y..(rect_y + rect_height) {
                for x in rect_x..(rect_x + rect_width) {
                    let offset = y * (pitch / 4) + x;
                    *fb_ptr.add(offset) = 0xFFFFFF; // White
                }
            }
            
            // Draw a green border around the rectangle
            let border = 10;
            for y in (rect_y - border)..(rect_y + rect_height + border) {
                for x in (rect_x - border)..(rect_x + border) {
                    let offset = y * (pitch / 4) + x;
                    *fb_ptr.add(offset) = 0x00FF00; // Green left border
                }
                for x in (rect_x + rect_width)..(rect_x + rect_width + border) {
                    let offset = y * (pitch / 4) + x;
                    *fb_ptr.add(offset) = 0x00FF00; // Green right border
                }
            }
            for x in (rect_x - border)..(rect_x + rect_width + border) {
                for y in (rect_y - border)..(rect_y) {
                    let offset = y * (pitch / 4) + x;
                    *fb_ptr.add(offset) = 0x00FF00; // Green top border
                }
                for y in (rect_y + rect_height)..(rect_y + rect_height + border) {
                    let offset = y * (pitch / 4) + x;
                    *fb_ptr.add(offset) = 0x00FF00; // Green bottom border
                }
            }
            
            // Draw "POLYGLOT OS" text pattern (simple pixel art)
            let text_y = rect_y + 180;
            let text_x = rect_x + 200;
            
            // Draw some colorful pixels to represent text
            for i in 0..200 {
                let offset = text_y * (pitch / 4) + text_x + i;
                *fb_ptr.add(offset) = 0xFF0000; // Red line
            }
            for i in 0..200 {
                let offset = (text_y + 20) * (pitch / 4) + text_x + i;
                *fb_ptr.add(offset) = 0x0000FF; // Blue line
            }
        }
        
        crate::serial::write_str("Framebuffer drawing complete!\n");
    }
}
