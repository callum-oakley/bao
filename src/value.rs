use std::rc::Rc;

use crate::compiler::Chunk;

#[derive(Debug, Clone)]
pub enum Value<'src> {
    Nil,
    Bool(bool),
    Int(i32),
    String(Rc<String>),
    Function(Rc<Function<'src>>),
}

#[derive(Debug)]
pub struct Function<'src> {
    pub arity: u8,
    pub chunk: Chunk<'src>,
    // TODO name?
}

impl<'src> Value<'src> {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Bool(false) | Value::Nil => true,
            _ => false,
        }
    }

    pub fn string(s: String) -> Value<'src> {
        Value::String(Rc::new(s))
    }

    pub fn function(f: Function) -> Value {
        Value::Function(Rc::new(f))
    }
}
