//! Interactive shell and command processor

pub mod parser;
pub mod builtins;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Shell state
pub struct Shell {
    input_buffer: String,
    history: Vec<String>,
    prompt: &'static str,
}

impl Shell {
    /// Create a new shell
    pub fn new() -> Self {
        Self {
            input_buffer: String::new(),
            history: Vec::new(),
            prompt: "polyglot> ",
        }
    }
    
    /// Start the shell
    pub fn run(&mut self) {
        // TODO: Temporarily disable shell output to avoid double fault
        // crate::console::println("");
        // crate::console::println("Starting Polyglot OS Shell...");
        // crate::console::println("Type 'help' for available commands.");
        // crate::console::println("");
        
        // self.show_prompt();
    }
    
    /// Handle a character input
    pub fn handle_char(&mut self, c: u8) {
        match c {
            b'\n' | b'\r' => {
                // Execute command
                crate::console::print("\n");
                let command = self.input_buffer.trim().to_string();
                
                if !command.is_empty() {
                    self.history.push(command.clone());
                    self.execute_command(&command);
                }
                
                self.input_buffer.clear();
                self.show_prompt();
            }
            b'\x08' => {
                // Backspace
                if !self.input_buffer.is_empty() {
                    self.input_buffer.pop();
                    crate::console::print("\x08 \x08"); // Backspace, space, backspace
                }
            }
            c if c >= 32 && c <= 126 => {
                // Printable character
                self.input_buffer.push(c as char);
                let byte_array = [c];
                let s = core::str::from_utf8(&byte_array).unwrap_or("?");
                crate::console::print(s);
            }
            _ => {
                // Ignore other characters
            }
        }
    }
    
    /// Show the command prompt
    fn show_prompt(&self) {
        crate::console::print(self.prompt);
    }
    
    /// Execute a command
    fn execute_command(&mut self, command: &str) {
        let args = parser::parse_command(command);
        
        if args.is_empty() {
            return;
        }
        
        let cmd = &args[0];
        let cmd_args = &args[1..];
        
        match cmd.as_str() {
            "help" => builtins::cmd_help(cmd_args),
            "clear" => builtins::cmd_clear(cmd_args),
            "echo" => builtins::cmd_echo(cmd_args),
            "mem" => builtins::cmd_mem(cmd_args),
            "tasks" => builtins::cmd_tasks(cmd_args),
            "uptime" => builtins::cmd_uptime(cmd_args),
            "history" => builtins::cmd_history(cmd_args, &self.history),
            "reboot" => builtins::cmd_reboot(cmd_args),
            "panic" => builtins::cmd_panic(cmd_args),
            _ => {
                crate::console::print("Unknown command: ");
                crate::console::print(cmd);
                crate::console::println("");
                crate::console::println("Type 'help' for available commands.");
            }
        }
    }
}

/// Global shell instance
static mut SHELL: Option<Shell> = None;

/// Initialize the shell
pub fn init() {
    unsafe {
        SHELL = Some(Shell::new());
        if let Some(ref mut shell) = SHELL {
            shell.run();
        }
    }
}

/// Handle keyboard input for the shell
pub fn handle_keyboard_input(c: u8) {
    unsafe {
        if let Some(ref mut shell) = SHELL {
            shell.handle_char(c);
        }
    }
}
