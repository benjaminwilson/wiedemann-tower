/// Underlying arithmetic operations
mod arithmetic;
/// Parser (and evaluator) for expressions
mod parser;

use parser::Parser;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut prev: Option<Vec<bool>> = None;

    println!("Bitstrings represent elements of the Wiedemann tower and must be length a power of 2.");
    println!("Examples:");
    println!("T1: 00 = 0, 10 = 1, 01 = X0, 11 = 1 + X0");
    println!("T2: 0000 = 0, 1000 = 1, .., 1010 = 1 + X1, .., 1001 = 1 + X0X1");
    println!("Enter expressions using 0/1, '*', '/', '+', '()', and '_' for the previous result.");
    println!("Type 'exit' or press Ctrl+D to quit.");
    println!("");

    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed.eq_ignore_ascii_case("exit") {
                    break;
                }
                let mut parser = Parser::new(trimmed, prev.clone());
                match parser.parse_expression() {
                    Ok(result) => {
                        // Check if there are any leftover non-whitespace characters
                        parser.skip_whitespace();
                        if parser.pos < parser.chars.len() {
                            eprintln!("Error: Unexpected character at position {}", parser.pos + 1);
                            continue;
                        }

                        print!("=");
                        result.iter().for_each(|bit| {
                            if *bit {
                                print!("1");
                            } else {
                                print!("0");
                            }
                        });
                        println!();
                        prev = Some(result);
                    }
                    Err(err_msg) => {
                        eprintln!("Error: {}", err_msg);
                    }
                }
            }
            Err(_) => break, // EOF or read error
        }
    }

    println!("Goodbye!");
}