use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // کلمات کلیدی
    Function,
    If,
    Else,
    While,
    For,
    Return,
    Print,
    Input,
    True,
    False,
    Null,
    Var,
    Const,
    
    // شناسه‌ها و مقادیر
    Identifier(String),
    Number(f64),
    String(String),
    
    // عملگرها
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Power,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    
    // جداکننده‌ها
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    Dot,
    
    // انتهای فایل
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = if chars.is_empty() { None } else { Some(chars[0]) };
        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
        self.current_char = if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        };
    }
    
    fn peek(&self, offset: usize) -> Option<char> {
        let pos = self.position + offset;
        if pos < self.input.len() {
            Some(self.input[pos])
        } else {
            None
        }
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_comment(&mut self) {
        if self.current_char == Some('/') && self.peek(1) == Some('/') {
            while self.current_char.is_some() && self.current_char != Some('\n') {
                self.advance();
            }
        }
    }
    
    fn read_string(&mut self, quote: char) -> Result<String, String> {
        let mut result = String::new();
        self.advance();
        
        while self.current_char.is_some() && self.current_char != Some(quote) {
            if let Some(ch) = self.current_char {
                result.push(ch);
                self.advance();
            }
        }
        
        if self.current_char != Some(quote) {
            return Err("رشته بسته نشده".to_string());
        }
        
        self.advance();
        Ok(result)
    }
    
    fn read_number(&mut self) -> Result<f64, String> {
        let mut result = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_numeric() || ch == '.' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        result.parse::<f64>().map_err(|_| "عدد نامعتبر".to_string())
    }
    
    fn read_identifier(&mut self) -> String {
        let mut result = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        result
    }
    
    fn keyword_or_identifier(&self, word: &str) -> Token {
        match word {
            "تابع" => Token::Function,
            "اگر" => Token::If,
            "وگرنه" => Token::Else,
            "تا‌زمانی_که" => Token::While,
            "برای" => Token::For,
            "برگردان" => Token::Return,
            "نوشتن" => Token::Print,
            "ورودی" => Token::Input,
            "درست" => Token::True,
            "غلط" => Token::False,
            "تهی" => Token::Null,
            "متغیر" => Token::Var,
            "ثابت" => Token::Const,
            "و" => Token::And,
            "یا" => Token::Or,
            "نه" => Token::Not,
            _ => Token::Identifier(word.to_string()),
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while self.current_char.is_some() {
            self.skip_whitespace();
            
            if self.current_char == Some('/') && self.peek(1) == Some('/') {
                self.skip_comment();
                continue;
            }
            
            if self.current_char.is_none() {
                break;
            }
            
            match self.current_char.unwrap() {
                '(' => {
                    tokens.push(Token::LeftParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    self.advance();
                }
                '{' => {
                    tokens.push(Token::LeftBrace);
                    self.advance();
                }
                '}' => {
                    tokens.push(Token::RightBrace);
                    self.advance();
                }
                '[' => {
                    tokens.push(Token::LeftBracket);
                    self.advance();
                }
                ']' => {
                    tokens.push(Token::RightBracket);
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    self.advance();
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.advance();
                }
                '.' => {
                    tokens.push(Token::Dot);
                    self.advance();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }
                '*' => {
                    if self.peek(1) == Some('*') {
                        tokens.push(Token::Power);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Star);
                        self.advance();
                    }
                }
                '/' => {
                    tokens.push(Token::Slash);
                    self.advance();
                }
                '%' => {
                    tokens.push(Token::Percent);
                    self.advance();
                }
                '=' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::EqualEqual);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Equal);
                        self.advance();
                    }
                }
                '!' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::NotEqual);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Not);
                        self.advance();
                    }
                }
                '<' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::LessEqual);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Less);
                        self.advance();
                    }
                }
                '>' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::GreaterEqual);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Greater);
                        self.advance();
                    }
                }
                '"' => {
                    let s = self.read_string('"')?;
                    tokens.push(Token::String(s));
                }
                '\'' => {
                    let s = self.read_string('\'')?;
                    tokens.push(Token::String(s));
                }
                ch if ch.is_numeric() => {
                    let num = self.read_number()?;
                    tokens.push(Token::Number(num));
                }
                ch if ch.is_alphabetic() || ch == '_' => {
                    let ident = self.read_identifier();
                    let token = self.keyword_or_identifier(&ident);
                    tokens.push(token);
                }
                ch => {
                    return Err(format!("کاراکتر نامعلوم: {}", ch));
                }
            }
        }
        
        tokens.push(Token::Eof);
        Ok(tokens)
    }
}
