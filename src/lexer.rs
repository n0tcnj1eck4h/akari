use std::str::FromStr;

use crate::token::{Keyword, Operator, Token};

pub struct Lexer<T> {
    stream: T,
    ch: Option<char>,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    pub fn new(mut stream: T) -> Self {
        let ch = stream.next();
        Lexer { stream, ch }
    }

    fn advance(&mut self) -> bool {
        self.ch = self.stream.next();
        self.ch.is_some()
    }

    fn check<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(char) -> bool,
    {
        self.ch.map_or(false, f)
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.check(char::is_whitespace) {
            self.advance();
        }

        let ch = match self.ch {
            Some(character) => character,
            None => return None,
        };

        if ch == '#' {
            while let Some(ch) = self.ch {
                self.advance();
                if ch == '\n' {
                    break;
                }
            }
            return self.next();
        } else if ch.is_alphabetic() {
            let mut buf = String::new();
            while let Some(ch) = self.ch {
                if ch.is_alphanumeric() || ch == '_' {
                    buf.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }

            if let Ok(keyword) = Keyword::from_str(&buf) {
                return Some(Token::Keyword(keyword));
            }

            return Some(match buf.as_str() {
                "false" => Token::BooleanLiteral(false),
                "true" => Token::BooleanLiteral(true),
                _ => Token::Identifier(buf),
            });
        } else if let '0'..='9' = ch {
            let mut n = 0i128;
            while let Some(ch) = self.ch {
                if let Some(d) = ch.to_digit(10) {
                    n *= 10;
                    n += d as i128;
                    self.advance();
                } else if ch == '.' {
                    self.advance();
                    let mut denominator = 1f64;
                    let mut decimals = 0f64;
                    while let Some(ch) = self.ch {
                        if let Some(d) = ch.to_digit(10) {
                            self.advance();
                            denominator *= 10.0;
                            decimals += d as f64 / denominator;
                        } else {
                            return Some(Token::FloatingPointLiteral(n as f64 + decimals));
                        }
                    }
                } else {
                    break;
                }
            }
            return Some(Token::IntegerLiteral(n));
        } else if ch == '"' {
            self.advance();
            let mut buf = String::new();
            let mut escaped = false;
            while let Some(ch) = self.ch {
                match (ch, escaped) {
                    ('"', false) => break,
                    ('\\', false) => {
                        escaped = true;
                        continue;
                    }
                    ('\\', true) => buf.push('\\'),
                    ('n', true) => buf.push('\n'),
                    ('"', true) => buf.push('"'),
                    (_, false) => buf.push(ch),
                    (_, true) => {
                        buf.push('\\');
                        buf.push(ch);
                    }
                }
                escaped = false;
                self.advance();
            }
            return Some(Token::StringLiteral(buf));
        } else {
            self.advance();
            let ch2 = self.ch;
            #[rustfmt::skip]
            let op = match (ch, ch2) {
                ('=', Some('=')) => { self.advance(); Some(Operator::Equal) }
                ('<', Some('=')) => { self.advance(); Some(Operator::LessOrEqual) }
                ('>', Some('=')) => { self.advance(); Some(Operator::GreaterOrEqual) }
                ('>', Some('>')) => { self.advance(); Some(Operator::BinaryRight) }
                ('<', Some('<')) => { self.advance(); Some(Operator::BinaryLeft) }
                ('!', Some('=')) => { self.advance(); Some(Operator::NotEqual) }
                ('&', Some('&')) => { self.advance(); Some(Operator::LogicAnd) }
                ('|', Some('|')) => { self.advance(); Some(Operator::LogicOr) }
                (':', Some(':')) => { self.advance(); Some(Operator::ScopeResolution) }
                ('-', Some('>')) => { self.advance(); Some(Operator::RightArrow) }
                ('+', _) => Some(Operator::Add),
                ('-', _) => Some(Operator::Subtract),
                ('*', _) => Some(Operator::Multiply),
                ('/', _) => Some(Operator::Divide),
                ('%', _) => Some(Operator::Modulo),
                ('&', _) => Some(Operator::BinaryAnd),
                ('|', _) => Some(Operator::BinaryOr),
                ('^', _) => Some(Operator::BinaryXor),
                ('~', _) => Some(Operator::BinaryNot),
                ('=', _) => Some(Operator::Assign),
                ('<', _) => Some(Operator::Less),
                ('>', _) => Some(Operator::Greater),
                ('!', _) => Some(Operator::LogicNot),
                _ => None,
            };
            if let Some(op) = op {
                return Some(Token::Operator(op));
            } else {
                return Some(Token::Atom(ch));
            }
        }
    }
}
