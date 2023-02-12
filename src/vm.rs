use std::{
    io::{self, BufRead},
    rc::Rc,
};

use crate::{
    compiler::Op,
    value::{Function, Value},
};

pub struct VM<'src> {
    stack: Vec<Value<'src>>,
    frames: Vec<Frame<'src>>,
}

struct Frame<'src> {
    function: Rc<Function<'src>>,
    ip: u16,
    offset: u16,
}

impl<'src> VM<'src> {
    pub fn new(function: Rc<Function<'src>>) -> Self {
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
        let stdin = io::stdin();
        let mut frame = self.frames.last_mut().unwrap();
        loop {
            println!("{:?}", self.stack);
            println!("{:?}", frame.function.chunk.code[frame.ip as usize]);
            // stdin.lock().lines().next().unwrap().unwrap();
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
                    frame.ip += i;
                    continue;
                }
                Op::JumpIfFalse(i) => {
                    if self.stack.last().unwrap().is_falsey() {
                        frame.ip += i;
                        continue;
                    }
                }
                Op::Call(args) => {
                    match self
                        .stack
                        .get(self.stack.len() - 1 - args as usize)
                        .unwrap()
                    {
                        // TODO check arity
                        Value::Function(function) => {
                            if args != function.arity {
                                todo!();
                            }
                            self.frames.push(Frame {
                                function: function.clone(),
                                ip: 0,
                                offset: self.stack.len() as u16 - args,
                            });
                            frame = self.frames.last_mut().unwrap();
                            continue;
                        }
                        Value::Native(native) => {
                            if args != native.arity {
                                todo!();
                            }
                            let value =
                                (native.f)(&self.stack[(self.stack.len() - args as usize)..]);
                            self.stack.truncate(self.stack.len() - args as usize - 1);
                            self.stack.push(value);
                        }
                        _ => todo!(),
                    }
                }
                Op::Return => {
                    {
                        let value = self.stack.pop().unwrap();
                        let frame = self.frames.pop().unwrap();
                        if self.frames.is_empty() {
                            return value;
                        }
                        self.stack.truncate(frame.offset as usize - 1);
                        self.stack.push(value);
                    }
                    frame = self.frames.last_mut().unwrap();
                }
                Op::Pop => {
                    self.stack.pop().unwrap();
                }
                Op::Squash => {
                    let value = self.stack.pop().unwrap();
                    self.stack.pop().unwrap();
                    self.stack.push(value);
                }
                Op::Unreachable => panic!("unreachable"),
            }
            frame.ip += 1;
        }
    }
}
