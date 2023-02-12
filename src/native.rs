use crate::value::{Native, Value};

fn add<'src>(args: &[Value<'src>]) -> Value<'src> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
        _ => todo!(),
    }
}
pub static ADD: Native = Native { arity: 2, f: add };

fn sub<'src>(args: &[Value<'src>]) -> Value<'src> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
        _ => todo!(),
    }
}
pub static SUB: Native = Native { arity: 2, f: sub };

fn eq<'src>(args: &[Value<'src>]) -> Value<'src> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a == b),
        _ => todo!(),
    }
}
pub static EQ: Native = Native { arity: 2, f: eq };

fn not_eq<'src>(args: &[Value<'src>]) -> Value<'src> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a != b),
        _ => todo!(),
    }
}
pub static NOT_EQ: Native = Native {
    arity: 2,
    f: not_eq,
};

fn lt<'src>(args: &[Value<'src>]) -> Value<'src> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a < b),
        _ => todo!(),
    }
}
pub static LT: Native = Native { arity: 2, f: lt };

fn lt_eq<'src>(args: &[Value<'src>]) -> Value<'src> {
    match (&args[0], &args[1]) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a <= b),
        _ => todo!(),
    }
}
pub static LT_EQ: Native = Native { arity: 2, f: lt_eq };

fn not<'src>(args: &[Value<'src>]) -> Value<'src> {
    Value::Bool(args[0].is_falsey())
}
pub static NOT: Native = Native { arity: 1, f: not };
