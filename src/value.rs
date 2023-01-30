#[derive(Debug, Clone, Copy)]
pub enum Value {
    Bool(bool),
    Int(i32),
    Nil,
}
impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Bool(false) | Value::Nil => true,
            _ => false,
        }
    }
}
