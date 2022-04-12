use crate::error::Error;
use crate::token::{Keyword, Kind, Position, Token};

pub struct Scanner {
    source: Vec<char>,
    current_position: usize,
    current_start: usize,
    current_line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            current_position: 0,
            current_start: 0,
            current_line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.finished() {
            self.mark_start();

            match self.scan_token()? {
                Some(token) => tokens.push(token),
                None => (),
            }
        }

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Option<Token>, Error> {
        if let Some(character) = self.get_current_char_and_advance() {
            match character {
                '(' => Ok(Some(self.build_token(Kind::OpenParenthesis))),
                ')' => Ok(Some(self.build_token(Kind::CloseParenthesis))),
                '{' => Ok(Some(self.build_token(Kind::OpenCurlyBracket))),
                '}' => Ok(Some(self.build_token(Kind::CloseCurlyBracket))),
                ',' => Ok(Some(self.build_token(Kind::Comma))),
                '.' => Ok(Some(self.build_token(Kind::Dot))),
                '-' => Ok(Some(self.build_token(Kind::Minus))),
                '+' => Ok(Some(self.build_token(Kind::Plus))),
                ';' => Ok(Some(self.build_token(Kind::Semicolon))),
                '*' => Ok(Some(self.build_token(Kind::Asterisk))),

                '!' if self.get_next_char() == Some('=') => {
                    self.advance();
                    Ok(Some(self.build_token(Kind::ExclamationEqual)))
                }
                '!' => Ok(Some(self.build_token(Kind::Exclamation))),

                '>' if self.get_current_char() == Some('=') => {
                    self.advance();
                    Ok(Some(self.build_token(Kind::GreaterEqual)))
                }
                '>' => Ok(Some(self.build_token(Kind::Greater))),

                '<' if self.get_current_char() == Some('=') => {
                    self.advance();
                    Ok(Some(self.build_token(Kind::LessEqual)))
                }
                '<' => Ok(Some(self.build_token(Kind::Less))),

                '=' if self.get_current_char() == Some('=') => {
                    self.advance();
                    Ok(Some(self.build_token(Kind::EqualEqual)))
                }
                '=' => Ok(Some(self.build_token(Kind::Equal))),

                '/' if self.get_current_char() == Some('/') => {
                    while self.get_current_char() != Some('\n') && !self.finished() {
                        self.advance();
                    }

                    Ok(None)
                }
                '/' => Ok(Some(self.build_token(Kind::Slash))),

                '"' => self.scan_string(),

                '0'..='9' => self.scan_number(),

                'a'..='z' | 'A'..='Z' | '_' => self.scan_identifier(),

                ' ' | '\r' | '\t' => Ok(None),
                '\n' => {
                    self.advance_line();
                    Ok(None)
                }

                _ => Err(self.build_error("Invalid syntax.".to_string())),
            }
        } else {
            Err(self.build_error("Invalid syntax.".to_string()))
        }
    }

    fn scan_string(&mut self) -> Result<Option<Token>, Error> {
        while self.get_current_char() != Some('"') && !self.finished() {
            if self.get_current_char() == Some('\n') {
                self.advance_line();
            }

            self.advance();
        }

        if self.finished() {
            Err(self.build_error("EOF while scanning string literal".to_string()))
        } else {
            self.advance();

            let string: String = self.source[(self.current_start + 1)..(self.current_position - 1)]
                .iter()
                .collect();

            Ok(Some(self.build_token(Kind::String(string))))
        }
    }

    fn scan_number(&mut self) -> Result<Option<Token>, Error> {
        while is_numeric(self.get_current_char()) {
            self.advance();
        }

        if self.get_current_char() == Some('.') && is_numeric(self.get_next_char()) {
            self.advance();
            self.advance();

            while is_numeric(self.get_current_char()) {
                self.advance();
            }
        }

        let number: f64 = self.source[self.current_start..self.current_position]
            .iter()
            .collect::<String>()
            .parse::<f64>()
            .unwrap();

        Ok(Some(self.build_token(Kind::Number(number))))
    }

    fn scan_identifier(&mut self) -> Result<Option<Token>, Error> {
        while is_alphanumeric(self.get_current_char()) {
            self.advance();
        }

        let string: String = self.source[self.current_start..self.current_position]
            .iter()
            .collect();

        match str::parse::<Keyword>(&string) {
            Ok(keyword) => Ok(Some(self.build_token(Kind::Keyword(keyword)))),
            Err(_) => Ok(Some(self.build_token(Kind::Identifier(string)))),
        }
    }

    fn get_current_char_and_advance(&mut self) -> Option<char> {
        let character = self.get_current_char();
        self.advance();

        character
    }

    fn advance(&mut self) -> () {
        self.current_position += 1;
    }

    fn advance_line(&mut self) -> () {
        self.current_line += 1;
    }

    fn mark_start(&mut self) -> () {
        self.current_start = self.current_position;
    }

    fn get_character_at_position(&self, position: usize) -> char {
        *self.source.get(position).unwrap()
    }

    fn finished(&self) -> bool {
        self.current_position >= self.source.len()
    }

    fn is_valid_position(&self, position: usize) -> bool {
        position <= self.source.len()
    }

    fn get_current_char(&self) -> Option<char> {
        match self.finished() {
            true => None,
            false => Some(self.get_character_at_position(self.current_position)),
        }
    }

    fn get_next_char(&self) -> Option<char> {
        match self.is_valid_position(self.current_position + 1) {
            false => None,
            true => Some(self.get_character_at_position(self.current_position + 1)),
        }
    }

    fn build_token(&self, kind: Kind) -> Token {
        Token {
            kind,
            position: Position {
                start: self.current_start,
                current: self.current_position,
                line: self.current_line,
            },
        }
    }

    fn build_error(&self, message: String) -> Error {
        Error {
            message,
            line: self.current_line,
        }
    }
}

fn is_numeric(character: Option<char>) -> bool {
    if let Some(_ch @ '0'..='9') = character {
        true
    } else {
        false
    }
}

fn is_alpha(character: Option<char>) -> bool {
    if let Some(ch) = character {
        match ch {
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false,
        }
    } else {
        false
    }
}

fn is_alphanumeric(character: Option<char>) -> bool {
    is_numeric(character) || is_alpha(character)
}
