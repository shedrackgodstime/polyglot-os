use limine::response::Framebuffer;

/// Draws a pattern to the screen using the provided framebuffer.
pub fn draw(framebuffer: &Framebuffer) {
    let fb_ptr = framebuffer.addr() as *mut u32;
    let width = framebuffer.width() as usize;
    let height = framebuffer.height() as usize;
    let pitch = framebuffer.pitch() as usize;

    unsafe {
        // Fill screen with a dark blue background
        fill_screen(fb_ptr, width, height, pitch, 0x001a1a2e);

        // Draw a large white rectangle in the center
        let rect_width = 600;
        let rect_height = 400;
        let rect_x = (width - rect_width) / 2;
        let rect_y = (height - rect_height) / 2;
        draw_rect(
            fb_ptr, pitch, rect_x, rect_y, rect_width, rect_height, 0xFFFFFF, // White
        );

        // Draw a green border around the rectangle
        let border = 10;
        draw_rect(
            fb_ptr,
            pitch,
            rect_x - border,
            rect_y - border,
            rect_width + 2 * border,
            border,
            0x00FF00, // Top border
        );
        draw_rect(
            fb_ptr,
            pitch,
            rect_x - border,
            rect_y + rect_height,
            rect_width + 2 * border,
            border,
            0x00FF00, // Bottom border
        );
        draw_rect(
            fb_ptr,
            pitch,
            rect_x - border,
            rect_y,
            border,
            rect_height,
            0x00FF00, // Left border
        );
        draw_rect(
            fb_ptr,
            pitch,
            rect_x + rect_width,
            rect_y,
            border,
            rect_height,
            0x00FF00, // Right border
        );

        // Draw "POLYGLOT OS" text pattern (simple pixel art)
        let text_y = rect_y + 180;
        let text_x = rect_x + 200;
        draw_rect(fb_ptr, pitch, text_x, text_y, 200, 10, 0xFF0000); // Red line
        draw_rect(fb_ptr, pitch, text_x, text_y + 20, 200, 10, 0x0000FF); // Blue line
    }
}

/// Fills the entire screen with a single color.
unsafe fn fill_screen(
    fb_ptr: *mut u32,
    width: usize,
    height: usize,
    pitch: usize,
    color: u32,
) {
    for y in 0..height {
        for x in 0..width {
            let offset = y * (pitch / 4) + x;
            *fb_ptr.add(offset) = color;
        }
    }
}

/// Draws a filled rectangle.
unsafe fn draw_rect(
    fb_ptr: *mut u32,
    pitch: usize,
    x_pos: usize,
    y_pos: usize,
    width: usize,
    height: usize,
    color: u32,
) {
    for y in y_pos..(y_pos + height) {
        for x in x_pos..(x_pos + width) {
            let offset = y * (pitch / 4) + x;
            *fb_ptr.add(offset) = color;
        }
    }
}
