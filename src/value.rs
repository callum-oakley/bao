use core::fmt;
use std::{rc::Rc};

use crate::compiler::Chunk;

#[derive(Debug, Clone)]
pub enum Value<'src> {
    Nil,
    Bool(bool),
    Int(i32),
    String(Rc<String>),
    Function(Rc<Function<'src>>),
    Native(&'static Native),
}

pub struct Function<'src> {
    pub arity: u16,
    pub chunk: Chunk<'src>,
    // TODO name?
}

impl<'src> fmt::Debug for Function<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<function>")
    }
}

pub struct Native {
    pub arity: u16,
    pub f: for<'src> fn(&[Value<'src>]) -> Value<'src>,
}

impl fmt::Debug for Native {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native>")
    }
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
