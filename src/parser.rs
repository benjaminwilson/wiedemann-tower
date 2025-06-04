use std::cmp::max;

use crate::arithmetic::{add, inv, mul};

/// A simple parser for bitstring expressions with AND (&) and OR (|),
/// parentheses, and “_” as the symbol for the previous result.
pub struct Parser {
    pub chars: Vec<char>,
    pub pos: usize,
    pub prev: Option<Vec<bool>>,
}

impl Parser {
    /// Create a new parser for the given input string, carrying the previous result.
    pub fn new(input: &str, prev: Option<Vec<bool>>) -> Self {
        Parser {
            chars: input.chars().collect(),
            pos: 0,
            prev,
        }
    }

    /// Parse an expression: term { '+' term }
    pub fn parse_expression(&mut self) -> Result<Vec<bool>, String> {
        let mut value = self.parse_term()?;
        loop {
            self.skip_whitespace();
            if self.peek_char() == Some('+') {
                self.consume_char(); // consume '+'
                let rhs = self.parse_term()?;
                let (lhs, rhs) = Parser::check_arguments(&value, &rhs)?;
                value = add(&lhs, &rhs);
            } else {
                break;
            }
        }
        Ok(value)
    }

    /// Parse a term: factor { '*|/' factor }
    fn parse_term(&mut self) -> Result<Vec<bool>, String> {
        let mut value = self.parse_factor()?;
        loop {
            self.skip_whitespace();
            if self.peek_char() == Some('*') {
                self.consume_char(); // consume '*'
                let rhs = self.parse_factor()?;
                let (lhs, rhs) = Parser::check_arguments(&value, &rhs)?;
                value = mul(&lhs, &rhs);
            } else if self.peek_char() == Some('/') {
                self.consume_char(); // consume '/'
                let rhs = self.parse_factor()?;
                let (lhs, rhs) = Parser::check_arguments(&value, &rhs)?;
                value = mul(&lhs, &inv(&rhs));
            } else {
                break;
            }
        }
        Ok(value)
    }

    /// Parse a factor: bitstring | '_' | '(' expression ')'
    fn parse_factor(&mut self) -> Result<Vec<bool>, String> {
        self.skip_whitespace();
        match self.peek_char() {
            Some('(') => {
                self.consume_char(); // consume '('
                let inner = self.parse_expression()?;
                self.skip_whitespace();
                if self.peek_char() == Some(')') {
                    self.consume_char(); // consume ')'
                    Ok(inner)
                } else {
                    Err("Expected ')'".to_string())
                }
            }
            Some('_') => {
                self.consume_char(); // consume '_'
                if let Some(prev_val) = &self.prev {
                    Ok(prev_val.clone())
                } else {
                    Err("No previous result available".to_string())
                }
            }
            Some(c) if c == '0' || c == '1' => {
                let mut bits: Vec<bool> = vec![];
                while let Some('0') | Some('1') = self.peek_char() {
                    let bit = self.consume_char().unwrap() == '1';
                    bits.push(bit);
                }
                if bits.is_empty() {
                    Err("Expected bitstring".to_string())
                } else {
                    Ok(bits)
                }
            }
            Some(other) => Err(format!("Unexpected character '{}'", other)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    /// Skip over any whitespace characters.
    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.consume_char();
            } else {
                break;
            }
        }
    }

    /// Peek at the next character without consuming it.
    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos).cloned()
    }

    /// Consume and return the next character.
    fn consume_char(&mut self) -> Option<char> {
        if self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            self.pos += 1;
            Some(c)
        } else {
            None
        }
    }

    /// Check that the two bitstrings are of length a power of two and return them padded to equal
    /// length with trailing false bits.
    fn check_arguments(lhs: &[bool], rhs: &[bool]) -> Result<(Vec<bool>, Vec<bool>), String> {
        if !lhs.len().is_power_of_two() {
            return Err(format!(
                "Bitstrings must be of length 2^i, but LHS has length {}",
                lhs.len(),
            ));
        }
        if !rhs.len().is_power_of_two() {
            return Err(format!(
                "Bitstrings must be of length 2^i, but RHS has length {}",
                rhs.len(),
            ));
        }
        let max_log = max(lhs.len().ilog2(), rhs.len().ilog2());
        let target_length = 1 << max_log; // 2^max_log
        let mut lhs_result = vec![false; target_length];
        let mut rhs_result = vec![false; target_length];
        lhs_result[..lhs.len()].copy_from_slice(lhs);
        rhs_result[..rhs.len()].copy_from_slice(rhs);

        Ok((lhs_result, rhs_result))
    }
}
