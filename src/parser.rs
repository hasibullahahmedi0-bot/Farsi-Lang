use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum AstNode {
    Program(Vec<AstNode>),
    Function {
        name: String,
        params: Vec<String>,
        body: Box<AstNode>,
    },
    Block(Vec<AstNode>),
    If {
        condition: Box<AstNode>,
        then_body: Box<AstNode>,
        else_body: Option<Box<AstNode>>,
    },
    While {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    For {
        init: Option<Box<AstNode>>,
        condition: Option<Box<AstNode>>,
        update: Option<Box<AstNode>>,
        body: Box<AstNode>,
    },
    Return(Option<Box<AstNode>>),
    Call {
        name: String,
        args: Vec<AstNode>,
    },
    BinaryOp {
        left: Box<AstNode>,
        op: String,
        right: Box<AstNode>,
    },
    UnaryOp {
        op: String,
        operand: Box<AstNode>,
    },
    Assignment {
        name: String,
        value: Box<AstNode>,
    },
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<AstNode>),
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }
    
    fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }
    
    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }
    
    pub fn parse(&mut self) -> Result<AstNode, String> {
        let mut statements = Vec::new();
        
        while self.current_token() != &Token::Eof {
            statements.push(self.parse_statement()?);
        }
        
        Ok(AstNode::Program(statements))
    }
    
    fn parse_statement(&mut self) -> Result<AstNode, String> {
        match self.current_token() {
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::LeftBrace => self.parse_block(),
            Token::Return => self.parse_return(),
            _ => self.parse_expression_statement(),
        }
    }
    
    fn parse_if(&mut self) -> Result<AstNode, String> {
        self.advance(); // Skip 'اگر'
        self.expect_token(&Token::LeftParen)?;
        let condition = Box::new(self.parse_expression()?);
        self.expect_token(&Token::RightParen)?;
        let then_body = Box::new(self.parse_statement()?);
        
        let else_body = if self.current_token() == &Token::Else {
            self.advance();
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };
        
        Ok(AstNode::If {
            condition,
            then_body,
            else_body,
        })
    }
    
    fn parse_while(&mut self) -> Result<AstNode, String> {
        self.advance(); // Skip 'تا‌زمانی_که'
        self.expect_token(&Token::LeftParen)?;
        let condition = Box::new(self.parse_expression()?);
        self.expect_token(&Token::RightParen)?;
        let body = Box::new(self.parse_statement()?);
        
        Ok(AstNode::While { condition, body })
    }
    
    fn parse_block(&mut self) -> Result<AstNode, String> {
        self.expect_token(&Token::LeftBrace)?;
        let mut statements = Vec::new();
        
        while self.current_token() != &Token::RightBrace && self.current_token() != &Token::Eof {
            statements.push(self.parse_statement()?);
        }
        
        self.expect_token(&Token::RightBrace)?;
        Ok(AstNode::Block(statements))
    }
    
    fn parse_return(&mut self) -> Result<AstNode, String> {
        self.advance(); // Skip 'برگردان'
        
        if self.current_token() == &Token::Semicolon {
            self.advance();
            Ok(AstNode::Return(None))
        } else {
            let expr = self.parse_expression()?;
            if self.current_token() == &Token::Semicolon {
                self.advance();
            }
            Ok(AstNode::Return(Some(Box::new(expr))))
        }
    }
    
    fn parse_expression_statement(&mut self) -> Result<AstNode, String> {
        let expr = self.parse_expression()?;
        if self.current_token() == &Token::Semicolon {
            self.advance();
        }
        Ok(expr)
    }
    
    fn parse_expression(&mut self) -> Result<AstNode, String> {
        self.parse_assignment()
    }
    
    fn parse_assignment(&mut self) -> Result<AstNode, String> {
        let expr = self.parse_or()?;
        
        if self.current_token() == &Token::Assign {
            if let AstNode::Identifier(name) = expr {
                self.advance();
                let value = Box::new(self.parse_assignment()?);
                return Ok(AstNode::Assignment { name, value });
            }
        }
        
        Ok(expr)
    }
    
    fn parse_or(&mut self) -> Result<AstNode, String> {
        let mut expr = self.parse_and()?;
        
        while self.current_token() == &Token::Or {
            self.advance();
            let right = Box::new(self.parse_and()?);
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                op: "یا".to_string(),
                right,
            };
        }
        
        Ok(expr)
    }
    
    fn parse_and(&mut self) -> Result<AstNode, String> {
        let mut expr = self.parse_equality()?;
        
        while self.current_token() == &Token::And {
            self.advance();
            let right = Box::new(self.parse_equality()?);
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                op: "و".to_string(),
                right,
            };
        }
        
        Ok(expr)
    }
    
    fn parse_equality(&mut self) -> Result<AstNode, String> {
        let mut expr = self.parse_comparison()?;
        
        while matches!(self.current_token(), Token::Equal | Token::NotEqual) {
            let op = match self.current_token() {
                Token::Equal => "==",
                Token::NotEqual => "!=",
                _ => unreachable!(),
            };
            self.advance();
            let right = Box::new(self.parse_comparison()?);
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                op: op.to_string(),
                right,
            };
        }
        
        Ok(expr)
    }
    
    fn parse_comparison(&mut self) -> Result<AstNode, String> {
        let mut expr = self.parse_addition()?;
        
        while matches!(
            self.current_token(),
            Token::Less | Token::Greater | Token::LessEqual | Token::GreaterEqual
        ) {
            let op = match self.current_token() {
                Token::Less => "<",
                Token::Greater => ">",
                Token::LessEqual => "<=",
                Token::GreaterEqual => ">=",
                _ => unreachable!(),
            };
            self.advance();
            let right = Box::new(self.parse_addition()?);
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                op: op.to_string(),
                right,
            };
        }
        
        Ok(expr)
    }
    
    fn parse_addition(&mut self) -> Result<AstNode, String> {
        let mut expr = self.parse_multiplication()?;
        
        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let op = match self.current_token() {
                Token::Plus => "+",
                Token::Minus => "-",
                _ => unreachable!(),
            };
            self.advance();
            let right = Box::new(self.parse_multiplication()?);
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                op: op.to_string(),
                right,
            };
        }
        
        Ok(expr)
    }
    
    fn parse_multiplication(&mut self) -> Result<AstNode, String> {
        let mut expr = self.parse_power()?;
        
        while matches!(self.current_token(), Token::Star | Token::Slash | Token::Modulo) {
            let op = match self.current_token() {
                Token::Star => "*",
                Token::Slash => "/",
                Token::Modulo => "%",
                _ => unreachable!(),
            };
            self.advance();
            let right = Box::new(self.parse_power()?);
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                op: op.to_string(),
                right,
            };
        }
        
        Ok(expr)
    }
    
    fn parse_power(&mut self) -> Result<AstNode, String> {
        let mut expr = self.parse_unary()?;
        
        if self.current_token() == &Token::Power {
            self.advance();
            let right = Box::new(self.parse_power()?);
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                op: "**".to_string(),
                right,
            };
        }
        
        Ok(expr)
    }
    
    fn parse_unary(&mut self) -> Result<AstNode, String> {
        match self.current_token() {
            Token::Minus | Token::Not => {
                let op = match self.current_token() {
                    Token::Minus => "-",
                    Token::Not => "!",
                    _ => unreachable!(),
                };
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(AstNode::UnaryOp {
                    op: op.to_string(),
                    operand,
                })
            }
            _ => self.parse_call(),
        }
    }
    
    fn parse_call(&mut self) -> Result<AstNode, String> {
        let mut expr = self.parse_primary()?;
        
        while self.current_token() == &Token::LeftParen {
            if let AstNode::Identifier(name) = expr {
                self.advance();
                let mut args = Vec::new();
                
                while self.current_token() != &Token::RightParen {
                    args.push(self.parse_expression()?);
                    if self.current_token() == &Token::Comma {
                        self.advance();
                    }
                }
                
                self.expect_token(&Token::RightParen)?;
                expr = AstNode::Call { name, args };
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn parse_primary(&mut self) -> Result<AstNode, String> {
        match self.current_token().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(AstNode::Number(n))
            }
            Token::String(s) => {
                self.advance();
                Ok(AstNode::String(s))
            }
            Token::True => {
                self.advance();
                Ok(AstNode::Boolean(true))
            }
            Token::False => {
                self.advance();
                Ok(AstNode::Boolean(false))
            }
            Token::Null => {
                self.advance();
                Ok(AstNode::Null)
            }
            Token::Identifier(name) => {
                self.advance();
                Ok(AstNode::Identifier(name))
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect_token(&Token::RightParen)?;
                Ok(expr)
            }
            _ => Err(format!("توکن غیر منتظره: {:?}", self.current_token())),
        }
    }
    
    fn expect_token(&mut self, expected: &Token) -> Result<(), String> {
        if std::mem::discriminant(self.current_token()) == std::mem::discriminant(expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("توکن مورد انتظار: {:?}", expected))
        }
    }
}
