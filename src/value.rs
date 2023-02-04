use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i32),
    String(Rc<String>),
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Bool(false) | Value::Nil => true,
            _ => false,
        }
    }
}
