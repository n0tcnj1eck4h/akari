use crate::operator::Operator;
use crate::token::{Keyword, Token};

pub struct Lexer<T: Iterator<Item = char>> {
    stream: T,
    ch: Option<char>,
}

impl<T: Iterator<Item = char>> Iterator for Lexer<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_token()
    }
}

impl<T: Iterator<Item = char>> Lexer<T> {
    fn advance(&mut self) -> bool {
        self.ch = self.stream.next();
        self.ch.is_some()
    }

    fn check<F>(&mut self, f: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        if let Some(ch) = self.ch {
            if f(ch) {
                return true;
            }
        }

        false
    }

    fn accept(&mut self, c: char) -> bool {
        if Some(c) == self.ch {
            self.advance();
            return true;
        }

        false
    }

    pub fn new(mut stream: T) -> Self {
        let ch = stream.next();
        Lexer { stream, ch }
    }

    pub fn get_token(&mut self) -> Option<Token> {
        while self.check(|c| c.is_whitespace()) {
            self.advance();
        }

        if self.check(|c| c == '#') {
            while self.check(|c| c != '\n') {
                self.advance();
            }

            return self.get_token();
        }

        if self.check(|c| c.is_alphabetic()) {
            let mut buf = String::new();
            while self.check(|c| c.is_alphanumeric() || c == '_') {
                buf.push(self.ch.unwrap());
                self.advance();
            }

            return Some(match buf.as_str() {
                "print" => Token::Keyword(Keyword::PRINT),
                "if"    => Token::Keyword(Keyword::IF),
                "else"  => Token::Keyword(Keyword::ELSE),
                _ => Token::Identifier(buf),
            });
        }

        if self.check(|c| c.is_numeric()) {
            let mut n = 0i32;
            while self.check(|c| c.is_numeric()) {
                n *= 10;
                n += self.ch.unwrap().to_digit(10).unwrap() as i32;
                self.advance();
            }

            return Some(Token::IntegerLiteral(n));
        }

        if let Some(ch) = self.ch {
            self.advance();
            if let Some(op) = match ch {
                '=' => Some(if self.accept('=') {
                    Operator::Equal
                } else {
                    Operator::Assign
                }),
                '*' => Some(if self.accept('*') {
                    Operator::Power
                } else {
                    Operator::Multiply
                }),
                '<' => Some(if self.accept('=') {
                    Operator::LessOrEqual
                } else if self.accept('<') {
                    Operator::BinaryLeft
                } else {
                    Operator::Less
                }),
                '>' => Some(if self.accept('=') {
                    Operator::GreaterOrEqual
                } else if self.accept('>') {
                    Operator::BinaryRight
                } else {
                    Operator::Greater
                }),
                '!' => Some(if self.accept('=') {
                    Operator::NotEqual
                } else {
                    Operator::LogicNot
                }),
                '&' => Some(if self.accept('&') {
                    Operator::LogicAnd
                } else {
                    Operator::BinaryAnd
                }),
                '|' => Some(if self.accept('|') {
                    Operator::LogicOr
                } else {
                    Operator::BinaryOr
                }),
                '+' => Some(Operator::Add),
                '-' => Some(Operator::Subtract),
                '/' => Some(Operator::Divide),
                '^' => Some(Operator::BinaryXor),
                '~' => Some(Operator::BinaryNot),
                '%' => Some(Operator::Modulo),
                _ => None,
            } {
                return Some(Token::Operator(op));
            } else {
                return Some(Token::Atom(ch));
            }
        }

        None
    }
}
