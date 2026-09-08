use crate::parser::AstNode;

#[derive(Debug, Clone)]
pub enum Bytecode {
    Push(Value),
    Pop,
    Store(String),
    Load(String),
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    Negate,
    Print,
    Input,
    JumpIfFalse(usize),
    Jump(usize),
    Call(String, usize),
    Return,
    Function(String, Vec<String>, Vec<Bytecode>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<Value>),
}

pub struct Compiler {
    functions: std::collections::HashMap<String, (Vec<String>, Vec<Bytecode>)>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            functions: std::collections::HashMap::new(),
        }
    }
    
    pub fn compile(&mut self, node: &AstNode) -> Result<Vec<Bytecode>, String> {
        self.compile_node(node)
    }
    
    fn compile_node(&mut self, node: &AstNode) -> Result<Vec<Bytecode>, String> {
        match node {
            AstNode::Program(statements) => {
                let mut bytecode = Vec::new();
                for stmt in statements {
                    bytecode.extend(self.compile_node(stmt)?);
                }
                Ok(bytecode)
            }
            AstNode::Number(n) => Ok(vec![Bytecode::Push(Value::Number(*n))]),
            AstNode::String(s) => Ok(vec![Bytecode::Push(Value::String(s.clone()))]),
            AstNode::Boolean(b) => Ok(vec![Bytecode::Push(Value::Boolean(*b))]),
            AstNode::Null => Ok(vec![Bytecode::Push(Value::Null)]),
            AstNode::Identifier(name) => Ok(vec![Bytecode::Load(name.clone())]),
            AstNode::BinaryOp { left, op, right } => {
                let mut bytecode = self.compile_node(left)?;
                bytecode.extend(self.compile_node(right)?);
                bytecode.push(match op.as_str() {
                    "+" => Bytecode::Add,
                    "-" => Bytecode::Subtract,
                    "*" => Bytecode::Multiply,
                    "/" => Bytecode::Divide,
                    "%" => Bytecode::Modulo,
                    "**" => Bytecode::Power,
                    "==" => Bytecode::Equal,
                    "!=" => Bytecode::NotEqual,
                    "<" => Bytecode::Less,
                    ">" => Bytecode::Greater,
                    "<=" => Bytecode::LessEqual,
                    ">=" => Bytecode::GreaterEqual,
                    "و" => Bytecode::And,
                    "یا" => Bytecode::Or,
                    _ => return Err(format!("عملگر نامشخص: {}", op)),
                });
                Ok(bytecode)
            }
            AstNode::UnaryOp { op, operand } => {
                let mut bytecode = self.compile_node(operand)?;
                bytecode.push(match op.as_str() {
                    "!" => Bytecode::Not,
                    "-" => Bytecode::Negate,
                    _ => return Err(format!("عملگر یکانی نامشخص: {}", op)),
                });
                Ok(bytecode)
            }
            AstNode::Assignment { name, value } => {
                let mut bytecode = self.compile_node(value)?;
                bytecode.push(Bytecode::Store(name.clone()));
                Ok(bytecode)
            }
            AstNode::Call { name, args } => {
                let mut bytecode = Vec::new();
                for arg in args {
                    bytecode.extend(self.compile_node(arg)?);
                }
                bytecode.push(Bytecode::Call(name.clone(), args.len()));
                Ok(bytecode)
            }
            AstNode::Block(statements) => {
                let mut bytecode = Vec::new();
                for stmt in statements {
                    bytecode.extend(self.compile_node(stmt)?);
                }
                Ok(bytecode)
            }
            AstNode::If { condition, then_body, else_body } => {
                let condition_code = self.compile_node(condition)?;
                let then_code = self.compile_node(then_body)?;
                let else_code = if let Some(else_b) = else_body {
                    self.compile_node(else_b)?
                } else {
                    Vec::new()
                };
                
                let mut bytecode = condition_code;
                let else_jump = bytecode.len() + then_code.len() + 1;
                bytecode.push(Bytecode::JumpIfFalse(else_jump));
                bytecode.extend(then_code);
                if !else_code.is_empty() {
                    bytecode.push(Bytecode::Jump(else_jump + else_code.len()));
                    bytecode.extend(else_code);
                }
                Ok(bytecode)
            }
            AstNode::Function { name, params, body } => {
                let body_code = self.compile_node(body)?;
                self.functions.insert(name.clone(), (params.clone(), body_code));
                Ok(vec![])
            }
            _ => Err("گره AST پشتیبانی نشده".to_string()),
        }
    }
}
