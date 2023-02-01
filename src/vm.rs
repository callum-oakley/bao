use crate::{
    code::{Chunk, Op},
    value::Value,
};

pub struct VM<'src> {
    ip: usize,
    chunk: Chunk<'src>,
    stack: Vec<Value>,
}

impl<'src> VM<'src> {
    pub fn new(chunk: Chunk<'src>) -> Self {
        VM {
            ip: 0,
            chunk,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Value {
        loop {
            println!("{:?}", self.stack);
            println!("{:?}", self.chunk.code[self.ip]);
            match self.chunk.code[self.ip] {
                Op::Nil => self.stack.push(Value::Nil),
                Op::False => self.stack.push(Value::Bool(false)),
                Op::True => self.stack.push(Value::Bool(true)),
                Op::Constant(i) => self.stack.push(self.chunk.constants[i as usize]),
                Op::Var(i) => self.stack.push(self.stack[i as usize]),
                Op::Jump(i) => {
                    self.ip += i as usize - 1;
                }
                Op::JumpIfFalse(i) => {
                    if self.stack.last().unwrap().is_falsey() {
                        self.ip += i as usize - 1;
                    }
                }
                Op::Pop => {
                    self.stack.pop().unwrap();
                }
                Op::Squash => {
                    let value = self.stack.pop().unwrap();
                    self.stack.pop().unwrap();
                    self.stack.push(value);
                }
                Op::Done => {
                    assert_eq!(self.stack.len(), 1);
                    return self.stack.pop().unwrap();
                }
                Op::Unreachable => panic!("unreachable"),
            }
            self.ip += 1;
        }
    }
}
