#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,
    Import,
    Print,
    If,
    Elif,
    Else,
    Global,
    While,
    Break,
    Continue,
    End,
    Action,
    Return,
    App,
    State,
    Page,
    Column,
    Row,
    Heading,
    Text,
    Button,
    Input,
    Container,
    Card,
    Icon,
    Grid,
    Run,
    Route,
    Get,
    Post,
    Put,
    Delete,
    Respond,
    Model,
    True,
    False,
    Null,
    StringLiteral(String),
    Identifier(String),
    Integer(i64),
    Float(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    LessThan,
    GreaterThan,
    DoubleEqual,
    NotEqual,
    LessEqual,
    GreaterEqual,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Equal,
    Comma,
    Colon,
    Dot,
    As,
    Newline,
    Invalid(String),
    Eof,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.pos < self.input.len() {
            let ch = self.current_char();

            // 1. Skip comments: # or //
            if ch == '#' || (ch == '/' && self.peek_char() == '/') {
                while self.pos < self.input.len() && self.current_char() != '\n' {
                    self.pos += self.current_char().len_utf8();
                }
                continue;
            }

            // 2. Strings
            if ch == '"' {
                // Check for triple-quote: """..."""
                if self.pos + 2 < self.input.len() {
                    let bytes = self.input.as_bytes();
                    if bytes[self.pos + 1] == b'"' && bytes[self.pos + 2] == b'"' {
                        self.pos += 3; // skip opening """
                        let mut s = String::new();
                        while self.pos < self.input.len() {
                            if self.pos + 2 < self.input.len()
                                && self.input.as_bytes()[self.pos] == b'"'
                                && self.input.as_bytes()[self.pos + 1] == b'"'
                                && self.input.as_bytes()[self.pos + 2] == b'"'
                            {
                                break;
                            }
                            let c = self.current_char();
                            s.push(c);
                            self.pos += c.len_utf8();
                        }
                        if self.pos + 2 < self.input.len() {
                            self.pos += 3; // skip closing """
                        } else {
                            tokens.push(Token::Invalid("unterminated triple-quote string".to_string()));
                            continue;
                        }
                        tokens.push(Token::StringLiteral(s));
                        continue;
                    }
                }

                // Regular single double-quote string
                self.pos += 1;
                let mut s = String::new();
                while self.pos < self.input.len() {
                    let c = self.current_char();
                    if c == '\\' && self.pos + 1 < self.input.len() {
                        let next = self.peek_char();
                        if next == '"' {
                            s.push('"');
                            self.pos += 2;
                            continue;
                        } else if next == 'n' {
                            s.push('\n');
                            self.pos += 2;
                            continue;
                        } else if next == 'r' {
                            s.push('\r');
                            self.pos += 2;
                            continue;
                        } else if next == 't' {
                            s.push('\t');
                            self.pos += 2;
                            continue;
                        } else if next == '\\' {
                            s.push('\\');
                            self.pos += 2;
                            continue;
                        }
                    }
                    if c == '"' {
                        break;
                    }
                    s.push(c);
                    self.pos += c.len_utf8();
                }
                if self.pos < self.input.len() && self.current_char() == '"' {
                    self.pos += 1; // skip closing quote
                } else {
                    tokens.push(Token::Invalid("unterminated string literal".to_string()));
                    continue;
                }
                tokens.push(Token::StringLiteral(s));
                continue;
            }

            // 3. Whitespace
            if ch.is_whitespace() {
                if ch == '\n' {
                    tokens.push(Token::Newline);
                }
                self.pos += ch.len_utf8();
            } else if ch.is_ascii_alphabetic() || ch == '_' {
                let start = self.pos;
                while self.pos < self.input.len() && (self.current_char().is_ascii_alphanumeric() || self.current_char() == '_') {
                    self.pos += 1;
                }
                let word = &self.input[start..self.pos];
                match word {
                    "let" => tokens.push(Token::Let),
                    "import" => tokens.push(Token::Import),
                    "print" => tokens.push(Token::Print),
                    "if" => tokens.push(Token::If),
                    "elif" => tokens.push(Token::Elif),
                    "else" => tokens.push(Token::Else),
                    "global" => tokens.push(Token::Global),
                    "while" => tokens.push(Token::While),
                    "break" => tokens.push(Token::Break),
                    "continue" => tokens.push(Token::Continue),
                    "action" => tokens.push(Token::Action),
                    "return" => tokens.push(Token::Return),
                    "app" => tokens.push(Token::App),
                    "state" => tokens.push(Token::State),
                    "page" | "Page" => tokens.push(Token::Page),
                    "Column" => tokens.push(Token::Column),
                    "Container" => tokens.push(Token::Container),
                    "Card" => tokens.push(Token::Card),
                    "Icon" => tokens.push(Token::Icon),
                    "Grid" => tokens.push(Token::Grid),
                    "Row" => tokens.push(Token::Row),
                    "Heading" => tokens.push(Token::Heading),
                    "Text" => tokens.push(Token::Text),
                    "Button" => tokens.push(Token::Button),
                    "Input" => tokens.push(Token::Input),
                    "run" | "Run" => tokens.push(Token::Run),
                    "end" => tokens.push(Token::End),
                    "route" => tokens.push(Token::Route),
                    "get" | "GET" => tokens.push(Token::Get),
                    "post" | "POST" => tokens.push(Token::Post),
                    "put" | "PUT" => tokens.push(Token::Put),
                    "delete" | "DELETE" => tokens.push(Token::Delete),
                    "respond" => tokens.push(Token::Respond),
                    "model" => tokens.push(Token::Model),
                    "true" => tokens.push(Token::True),
                    "false" => tokens.push(Token::False),
                    "null" => tokens.push(Token::Null),
                    "as" => tokens.push(Token::As),
                    _ => tokens.push(Token::Identifier(word.to_string())),
                }
            } else if ch.is_ascii_digit() {
                let start = self.pos;
                let mut is_float = false;
                while self.pos < self.input.len() {
                    let c = self.current_char();
                    if c.is_ascii_digit() {
                        self.pos += 1;
                    } else if c == '.' && !is_float && self.peek_char().is_ascii_digit() {
                        is_float = true;
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                let num_str = &self.input[start..self.pos];
                if is_float {
                    match num_str.parse::<f64>() {
                        Ok(value) if value.is_finite() => tokens.push(Token::Float(value)),
                        _ => tokens.push(Token::Invalid(format!("invalid or out-of-range float literal {}", num_str))),
                    }
                } else {
                    match num_str.parse::<i64>() {
                        Ok(value) => tokens.push(Token::Integer(value)),
                        _ => tokens.push(Token::Invalid(format!("integer literal {} exceeds the parser's 64-bit range", num_str))),
                    }
                }
            } else {
                match ch {
                    '+' => tokens.push(Token::Plus),
                    '-' => tokens.push(Token::Minus),
                    '*' => tokens.push(Token::Star),
                    '/' => tokens.push(Token::Slash),
                    '%' => tokens.push(Token::Percent),
                    '(' => tokens.push(Token::LeftParen),
                    ')' => tokens.push(Token::RightParen),
                    '[' => tokens.push(Token::LeftBracket),
                    ']' => tokens.push(Token::RightBracket),
                    '{' => tokens.push(Token::LeftBrace),
                    '}' => tokens.push(Token::RightBrace),
                    ',' => tokens.push(Token::Comma),
                    ':' => tokens.push(Token::Colon),
                    '.' => tokens.push(Token::Dot),
                    '=' => {
                        if self.peek_char() == '=' {
                            self.pos += 1;
                            tokens.push(Token::DoubleEqual);
                        } else {
                            tokens.push(Token::Equal);
                        }
                    }
                    '!' => {
                        if self.peek_char() == '=' {
                            self.pos += 1;
                            tokens.push(Token::NotEqual);
                        } else {
                            tokens.push(Token::Invalid("unexpected character '!'".to_string()));
                        }
                    }
                    '<' => {
                        if self.peek_char() == '=' {
                            self.pos += 1;
                            tokens.push(Token::LessEqual);
                        } else {
                            tokens.push(Token::LessThan);
                        }
                    }
                    '>' => {
                        if self.peek_char() == '=' {
                            self.pos += 1;
                            tokens.push(Token::GreaterEqual);
                        } else {
                            tokens.push(Token::GreaterThan);
                        }
                    }
                    _ => tokens.push(Token::Invalid(format!("unexpected character {:?}", ch))),
                }
                self.pos += ch.len_utf8();
            }
        }
        tokens.push(Token::Eof);
        tokens
    }

    fn current_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap_or('\0')
    }

    fn peek_char(&self) -> char {
        let mut it = self.input[self.pos..].chars();
        it.next();
        it.next().unwrap_or('\0')
    }
}
