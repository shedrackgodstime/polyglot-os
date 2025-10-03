//! Command line parser

use alloc::string::String;
use alloc::vec::Vec;

/// Parse a command line into command and arguments
pub fn parse_command(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
            }
            ' ' | '\t' if !in_quotes => {
                if !current_arg.is_empty() {
                    args.push(current_arg);
                    current_arg = String::new();
                }
                // Skip multiple spaces
                while let Some(&next_c) = chars.peek() {
                    if next_c == ' ' || next_c == '\t' {
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
            '\\' if chars.peek().is_some() => {
                // Escape sequence
                if let Some(escaped) = chars.next() {
                    match escaped {
                        'n' => current_arg.push('\n'),
                        't' => current_arg.push('\t'),
                        'r' => current_arg.push('\r'),
                        '\\' => current_arg.push('\\'),
                        '"' => current_arg.push('"'),
                        c => {
                            current_arg.push('\\');
                            current_arg.push(c);
                        }
                    }
                }
            }
            c => {
                current_arg.push(c);
            }
        }
    }
    
    if !current_arg.is_empty() {
        args.push(current_arg);
    }
    
    args
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_command() {
        let result = parse_command("echo hello world");
        assert_eq!(result, vec!["echo", "hello", "world"]);
    }
    
    #[test]
    fn test_quoted_args() {
        let result = parse_command("echo \"hello world\" test");
        assert_eq!(result, vec!["echo", "hello world", "test"]);
    }
    
    #[test]
    fn test_multiple_spaces() {
        let result = parse_command("echo   hello    world");
        assert_eq!(result, vec!["echo", "hello", "world"]);
    }
    
    #[test]
    fn test_escape_sequences() {
        let result = parse_command("echo \"hello\\nworld\"");
        assert_eq!(result, vec!["echo", "hello\nworld"]);
    }
}
