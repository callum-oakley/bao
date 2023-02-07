use std::{rc::Rc, io::{self, BufRead}};

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
    pub fn new(function: Value<'src>) -> Self {
        let mut vm = VM {
            stack: Vec::new(),
            frames: Vec::new(),
        };
        vm.stack.push(function);
        vm.call(0);
        vm
    }

    pub fn run(&mut self) -> Value {
        let stdin = io::stdin();
        let mut frame = self.frames.last_mut().unwrap();
        loop {
            println!("{:#?}", self.stack);
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
                    self.call(args);
                    frame = self.frames.last_mut().unwrap();
                    continue;
                }
                Op::Return => {
                    let value = self.stack.pop().unwrap();
                    {
                        let frame = self.frames.pop().unwrap();
                        if self.frames.is_empty() {
                            self.stack.pop().unwrap();
                            assert!(self.stack.is_empty());
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

    // TODO check arity
    fn call(&mut self, args: u16) {
        match self.stack.get(self.stack.len() - 1 - args as usize).unwrap() {
            Value::Function(function) => self.frames.push(Frame {
                function: function.clone(),
                ip: 0,
                offset: self.stack.len() as u16 - args,
            }),
            _ => todo!(),
        }
    }
}
