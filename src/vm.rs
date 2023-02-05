use crate::{
    compiler::Op,
    value::{Function, Value},
};

pub struct VM<'src> {
    stack: Vec<Value<'src>>,
    frames: Vec<Frame<'src>>,
}

struct Frame<'src> {
    function: Function<'src>,
    ip: u16,
    offset: u16,
}

impl<'src> VM<'src> {
    pub fn new(function: Function<'src>) -> Self {
        // TODO do we need to push the root function on to the stack?
        VM {
            stack: Vec::new(),
            frames: vec![Frame {
                function,
                ip: 0,
                offset: 0,
            }],
        }
    }

    pub fn run(&mut self) -> Value {
        loop {
            let frame = self.frames.last_mut().unwrap();
            println!("{:?}", self.stack);
            println!("{:?}", frame.function.chunk.code[frame.ip as usize]);
            match frame.function.chunk.code[frame.ip as usize] {
                Op::Nil => self.stack.push(Value::Nil),
                Op::False => self.stack.push(Value::Bool(false)),
                Op::True => self.stack.push(Value::Bool(true)),
                Op::Constant(i) => self
                    .stack
                    .push(frame.function.chunk.constants[i as usize].clone()),
                Op::Var(i) => self
                    .stack
                    .push(self.stack[(i + frame.offset) as usize].clone()),
                Op::Jump(i) => {
                    frame.ip += i - 1;
                }
                Op::JumpIfFalse(i) => {
                    if self.stack.last().unwrap().is_falsey() {
                        frame.ip += i - 1;
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
            frame.ip += 1;
        }
    }
}
