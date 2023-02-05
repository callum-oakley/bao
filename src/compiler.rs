use std::iter::Peekable;

use crate::{
    token::{Kind, Token, Tokens},
    value::{Function, Value},
};

#[derive(Debug)]
pub enum Op {
    Constant(u16),
    Done,
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
struct Compiler<'src> {
    tokens: Peekable<Tokens<'src>>,
    vars: Vec<&'src str>,
    function: Function<'src>,
}

impl<'src> Compiler<'src> {
    fn resolve_var(&mut self, name: &'src str) -> u16 {
        self.vars
            .iter()
            .rposition(|var| *var == name)
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn identifier(&mut self) -> &'src str {
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
                    self.function.chunk.push(Op::Nil, Some(token));
                }
                Kind::True => {
                    self.function.chunk.push(Op::True, Some(token));
                }
                Kind::False => {
                    self.function.chunk.push(Op::False, Some(token));
                }
                Kind::Int(s) => {
                    self.function
                        .chunk
                        .push_constant(Value::Int(s.parse().unwrap()), Some(token));
                }
                Kind::String(s) => {
                    self.function
                        .chunk
                        .push_constant(Value::string(s[1..(s.len() - 1)].to_string()), Some(token));
                }
                Kind::If => {
                    self.block(); // test
                    let jump_over_then = self.function.chunk.push(Op::Unreachable, Some(token));
                    self.function.chunk.push(Op::Pop, Some(token)); // pop result of test
                    self.block(); // then
                    let jump_over_else = self.function.chunk.push(Op::Unreachable, Some(token));
                    self.function.chunk.code[jump_over_then] = Op::JumpIfFalse(
                        (self.function.chunk.code.len() - jump_over_then)
                            .try_into()
                            .unwrap(),
                    );
                    self.function.chunk.push(Op::Pop, Some(token)); // pop result of test
                    self.block(); // else
                    self.function.chunk.code[jump_over_else] = Op::Jump(
                        (self.function.chunk.code.len() - jump_over_else)
                            .try_into()
                            .unwrap(),
                    );
                }
                Kind::Let => {
                    let name = self.identifier();
                    self.vars.push(name);
                    self.block();
                    self.block();
                    self.vars.pop();
                    self.function.chunk.push(Op::Squash, Some(token));
                }
                Kind::Var(name) => {
                    let i = self.resolve_var(name);
                    self.function.chunk.push(Op::Var(i), Some(token));
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
            self.function.chunk.push(Op::Pop, None);
            self.expression();
        }
    }
}

pub fn compile(tokens: Tokens) -> Function {
    let mut compiler = Compiler {
        tokens: tokens.peekable(),
        vars: Vec::new(),
        function: Function {
            arity: 0,
            name: "",
            chunk: Chunk {
                code: Vec::new(),
                constants: Vec::new(),
                debug_info: Vec::new(),
            },
        },
    };
    compiler.block();
    if !compiler.tokens.next().is_none() {
        todo!()
    }
    compiler.function.chunk.push(Op::Done, None);
    compiler.function
}
