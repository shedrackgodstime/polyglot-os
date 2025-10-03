//! Console and terminal emulation

pub mod font;
pub mod terminal;

use limine::framebuffer::Framebuffer;

/// Global console instance
static mut CONSOLE: Option<terminal::Terminal> = None;

/// Initialize the console system
pub fn init(framebuffer: &'static Framebuffer<'static>) {
    unsafe {
        CONSOLE = Some(terminal::Terminal::new(framebuffer));
    }
    
    // Clear screen and show welcome message
    clear();
    // TODO: Temporarily disable text output to avoid double fault
    // println("Polyglot OS Console");
    // println("==================");
    // println("");
    // println("Welcome to Polyglot OS!");
    // println("Type 'help' for available commands.");
    // println("");
}

/// Print a string to the console
pub fn print(s: &str) {
    unsafe {
        if let Some(ref mut console) = CONSOLE {
            console.write_str(s);
        }
    }
}

/// Print a string with newline
pub fn println(s: &str) {
    print(s);
    print("\n");
}

/// Clear the console screen
pub fn clear() {
    unsafe {
        if let Some(ref mut console) = CONSOLE {
            console.clear();
        }
    }
}

/// Get console dimensions (width, height in characters)
pub fn dimensions() -> (usize, usize) {
    unsafe {
        if let Some(ref console) = CONSOLE {
            console.dimensions()
        } else {
            (80, 25) // Default
        }
    }
}

/// Set cursor position
pub fn set_cursor(x: usize, y: usize) {
    unsafe {
        if let Some(ref mut console) = CONSOLE {
            console.set_cursor(x, y);
        }
    }
}
