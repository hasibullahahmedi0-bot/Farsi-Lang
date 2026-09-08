use crate::compiler::{Bytecode, Value};
use std::collections::HashMap;
use std::io::{self, Write};

pub struct Runtime {
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
    functions: HashMap<String, Vec<Bytecode>>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            stack: Vec::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
    
    pub fn execute(&mut self, bytecode: &[Bytecode]) -> Result<(), String> {
        self.execute_chunk(bytecode)
    }
    
    fn execute_chunk(&mut self, bytecode: &[Bytecode]) -> Result<(), String> {
        let mut pc = 0;
        
        while pc < bytecode.len() {
            match &bytecode[pc] {
                Bytecode::Push(value) => {
                    self.stack.push(value.clone());
                    pc += 1;
                }
                Bytecode::Pop => {
                    self.stack.pop();
                    pc += 1;
                }
                Bytecode::Store(name) => {
                    if let Some(value) = self.stack.pop() {
                        self.variables.insert(name.clone(), value);
                    }
                    pc += 1;
                }
                Bytecode::Load(name) => {
                    if let Some(value) = self.variables.get(name) {
                        self.stack.push(value.clone());
                    } else {
                        return Err(format!("متغیر نامعلوم: {}", name));
                    }
                    pc += 1;
                }
                Bytecode::Add => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
                        (Value::String(x), Value::String(y)) => Value::String(x + &y),
                        _ => Value::Null,
                    })?;
                    pc += 1;
                }
                Bytecode::Subtract => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => Value::Number(x - y),
                        _ => Value::Null,
                    })?;
                    pc += 1;
                }
                Bytecode::Multiply => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => Value::Number(x * y),
                        _ => Value::Null,
                    })?;
                    pc += 1;
                }
                Bytecode::Divide => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => {
                            if y == 0.0 {
                                Value::Null
                            } else {
                                Value::Number(x / y)
                            }
                        }
                        _ => Value::Null,
                    })?;
                    pc += 1;
                }
                Bytecode::Modulo => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => Value::Number(x % y),
                        _ => Value::Null,
                    })?;
                    pc += 1;
                }
                Bytecode::Power => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => Value::Number(x.powf(y)),
                        _ => Value::Null,
                    })?;
                    pc += 1;
                }
                Bytecode::Equal => {
                    self.binary_op(|a, b| Value::Boolean(a == b))?;
                    pc += 1;
                }
                Bytecode::NotEqual => {
                    self.binary_op(|a, b| Value::Boolean(a != b))?;
                    pc += 1;
                }
                Bytecode::Less => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => Value::Boolean(x < y),
                        _ => Value::Boolean(false),
                    })?;
                    pc += 1;
                }
                Bytecode::Greater => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => Value::Boolean(x > y),
                        _ => Value::Boolean(false),
                    })?;
                    pc += 1;
                }
                Bytecode::LessEqual => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => Value::Boolean(x <= y),
                        _ => Value::Boolean(false),
                    })?;
                    pc += 1;
                }
                Bytecode::GreaterEqual => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => Value::Boolean(x >= y),
                        _ => Value::Boolean(false),
                    })?;
                    pc += 1;
                }
                Bytecode::And => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Boolean(x), Value::Boolean(y)) => Value::Boolean(x && y),
                        _ => Value::Boolean(false),
                    })?;
                    pc += 1;
                }
                Bytecode::Or => {
                    self.binary_op(|a, b| match (a, b) {
                        (Value::Boolean(x), Value::Boolean(y)) => Value::Boolean(x || y),
                        _ => Value::Boolean(false),
                    })?;
                    pc += 1;
                }
                Bytecode::Not => {
                    if let Some(value) = self.stack.pop() {
                        let result = match value {
                            Value::Boolean(b) => Value::Boolean(!b),
                            _ => Value::Boolean(false),
                        };
                        self.stack.push(result);
                    }
                    pc += 1;
                }
                Bytecode::Negate => {
                    if let Some(value) = self.stack.pop() {
                        let result = match value {
                            Value::Number(n) => Value::Number(-n),
                            _ => Value::Null,
                        };
                        self.stack.push(result);
                    }
                    pc += 1;
                }
                Bytecode::Print => {
                    if let Some(value) = self.stack.pop() {
                        println!("{}", self.value_to_string(&value));
                    }
                    pc += 1;
                }
                Bytecode::Input => {
                    print!("> ");
                    io::stdout().flush().ok();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).ok();
                    self.stack.push(Value::String(input.trim().to_string()));
                    pc += 1;
                }
                Bytecode::Call(name, _args_count) => {
                    match name.as_str() {
                        "نوشتن" => {
                            if let Some(value) = self.stack.pop() {
                                println!("{}", self.value_to_string(&value));
                            }
                        }
                        _ => {
                            return Err(format!("تابع نامعلوم: {}", name));
                        }
                    }
                    pc += 1;
                }
                _ => {
                    return Err("دستور پشتیبانی نشده".to_string());
                }
            }
        }
        
        Ok(())
    }
    
    fn binary_op<F>(&mut self, op: F) -> Result<(), String>
    where
        F: Fn(Value, Value) -> Value,
    {
        if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
            self.stack.push(op(a, b));
            Ok(())
        } else {
            Err("Stack خالی است".to_string())
        }
    }
    
    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::String(s) => s.clone(),
            Value::Boolean(b) => if *b { "درست" } else { "غلط" }.to_string(),
            Value::Null => "تهی".to_string(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.value_to_string(v)).collect();
                format!("[{}]", items.join(", "))
            }
        }
    }
}
