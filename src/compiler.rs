use std::iter::Peekable;

use crate::{
    token::{Kind, Token, Tokens},
    value::{Function, Value},
};

#[derive(Debug)]
pub enum Op {
    Call(u16),
    Constant(u16),
    Return,
    False,
    Jump(u16),
    JumpIfFalse(u16),
    Nil,
    Pop,
    Squash,
    True,
    Unreachable,
    Var(u16),
}

#[derive(Debug)]
pub struct Chunk<'src> {
    pub code: Vec<Op>,
    pub constants: Vec<Value<'src>>,
    // TODO this is a huge waste of space
    pub debug_info: Vec<Option<Token<'src>>>,
}

impl<'src> Chunk<'src> {
    fn push(&mut self, op: Op, token: Option<Token<'src>>) -> usize {
        let i = self.code.len();
        self.code.push(op);
        self.debug_info.push(token);
        i
    }

    fn push_constant(&mut self, constant: Value<'src>, token: Option<Token<'src>>) -> usize {
        let i = self.constants.len().try_into().unwrap();
        self.constants.push(constant);
        self.push(Op::Constant(i), token)
    }
}

#[derive(Debug)]
struct State<'src> {
    vars: Vec<&'src str>,
    function: Function<'src>,
}

impl<'src> State<'src> {
    fn new() -> Self {
        Self {
            vars: Vec::new(),
            function: Function {
                arity: 0,
                chunk: Chunk {
                    code: Vec::new(),
                    constants: Vec::new(),
                    debug_info: Vec::new(),
                },
            },
        }
    }
}

#[derive(Debug)]
struct Compiler<'src> {
    tokens: Peekable<Tokens<'src>>,
    state: Vec<State<'src>>,
}

impl<'src> Compiler<'src> {
    fn vars(&mut self) -> &mut Vec<&'src str> {
        &mut self.state.last_mut().unwrap().vars
    }

    fn chunk(&mut self) -> &mut Chunk<'src> {
        &mut self.state.last_mut().unwrap().function.chunk
    }

    fn arity(&mut self) -> &mut u8 {
        &mut self.state.last_mut().unwrap().function.arity
    }

    fn resolve_var(&mut self, name: &'src str) -> u16 {
        self.vars()
            .iter()
            .rposition(|var| *var == name)
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn var(&mut self) -> &'src str {
        if let Kind::Var(name) = self.tokens.next().unwrap().kind {
            name
        } else {
            todo!()
        }
    }

    fn expression(&mut self) {
        if let Some(token) = self.tokens.next() {
            match token.kind {
                Kind::Nil => {
                    self.chunk().push(Op::Nil, Some(token));
                }
                Kind::True => {
                    self.chunk().push(Op::True, Some(token));
                }
                Kind::False => {
                    self.chunk().push(Op::False, Some(token));
                }
                Kind::Int(s) => {
                    self.chunk()
                        .push_constant(Value::Int(s.parse().unwrap()), Some(token));
                }
                Kind::String(s) => {
                    self.chunk()
                        .push_constant(Value::string(s[1..(s.len() - 1)].to_string()), Some(token));
                }
                Kind::If => {
                    self.block(); // test
                    let jump_over_then = self.chunk().push(Op::Unreachable, Some(token));
                    self.chunk().push(Op::Pop, Some(token)); // pop result of test
                    self.block(); // then
                    let jump_over_else = self.chunk().push(Op::Unreachable, Some(token));
                    self.chunk().code[jump_over_then] = Op::JumpIfFalse(
                        (self.chunk().code.len() - jump_over_then)
                            .try_into()
                            .unwrap(),
                    );
                    self.chunk().push(Op::Pop, Some(token)); // pop result of test
                    self.block(); // else
                    self.chunk().code[jump_over_else] = Op::Jump(
                        (self.chunk().code.len() - jump_over_else)
                            .try_into()
                            .unwrap(),
                    );
                }
                Kind::Let => {
                    let name = self.var();
                    self.vars().push(name);
                    self.block();
                    self.block();
                    self.vars().pop();
                    self.chunk().push(Op::Squash, Some(token));
                }
                Kind::Pipe => {
                    self.state.push(State::new());
                    loop {
                        if let Some(token) = self.tokens.next() {
                            match token.kind {
                                Kind::Pipe => {
                                    break;
                                }
                                Kind::Var(name) => {
                                    *self.arity() += 1;
                                    self.vars().push(name);
                                }
                                _ => todo!(),
                            }
                        } else {
                            todo!();
                        }
                    }
                    self.block();
                    self.chunk().push(Op::Return, Some(token));
                    let function = self.state.pop().unwrap().function;
                    self.chunk()
                        .push_constant(Value::function(function), Some(token));
                }
                Kind::OpenParen => {
                    self.expression(); // function
                    let mut args = 0;
                    loop {
                        if let Some(token) = self.tokens.peek() {
                            if let Kind::CloseParen = token.kind {
                                self.tokens.next();
                                break;
                            }
                            self.expression();
                            args += 1;
                        } else {
                            todo!();
                        }
                    }
                    self.chunk().push(Op::Call(args), Some(token));
                }
                Kind::Var(name) => {
                    let i = self.resolve_var(name);
                    self.chunk().push(Op::Var(i), Some(token));
                }
                _ => todo!(),
            }
        }
    }

    fn block(&mut self) {
        let token = self.tokens.peek().unwrap();
        let col = token.col;
        self.expression();
        while let Some(token) = self.tokens.peek() {
            if token.col != col {
                break;
            }
            self.chunk().push(Op::Pop, None);
            self.expression();
        }
    }
}

pub fn compile(tokens: Tokens) -> Value {
    let mut compiler = Compiler {
        tokens: tokens.peekable(),
        state: vec![State::new()],
    };
    compiler.block();
    if !compiler.tokens.next().is_none() {
        todo!()
    }
    assert_eq!(compiler.state.len(), 1);
    compiler.chunk().push(Op::Return, None);
    Value::function(compiler.state.pop().unwrap().function)
}
