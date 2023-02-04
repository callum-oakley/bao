use std::{iter::Peekable, rc::Rc};

use crate::{
    token::{Kind, Token, Tokens},
    value::Value,
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
    pub constants: Vec<Value>,
    // TODO this is a huge waste of space
    pub debug_info: Vec<Option<Token<'src>>>,
}

#[derive(Debug)]
struct State<'src> {
    tokens: Peekable<Tokens<'src>>,
    vars: Vec<&'src str>,
}

impl<'src> Chunk<'src> {
    pub fn new(tokens: Tokens<'src>) -> Self {
        let mut chunk = Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            debug_info: Vec::new(),
        };
        let mut state = State {
            tokens: tokens.peekable(),
            vars: Vec::new(),
        };
        chunk.block(&mut state);
        if !state.tokens.next().is_none() {
            todo!()
        }
        chunk.push(Op::Done, None);
        chunk
    }

    fn push(&mut self, op: Op, token: Option<Token<'src>>) -> usize {
        let i = self.code.len();
        self.code.push(op);
        self.debug_info.push(token);
        i
    }

    fn push_constant(&mut self, constant: Value, token: Option<Token<'src>>) -> usize {
        let i = self.constants.len().try_into().unwrap();
        self.constants.push(constant);
        self.push(Op::Constant(i), token)
    }

    fn resolve_var(&mut self, state: &mut State<'src>, name: &'src str) -> u16 {
        state
            .vars
            .iter()
            .rposition(|var| *var == name)
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn identifier(&mut self, state: &mut State<'src>) -> &'src str {
        if let Kind::Var(name) = state.tokens.next().unwrap().kind {
            name
        } else {
            todo!()
        }
    }

    fn expression(&mut self, state: &mut State<'src>) {
        if let Some(token) = state.tokens.next() {
            match token.kind {
                Kind::Nil => {
                    self.push(Op::Nil, Some(token));
                }
                Kind::True => {
                    self.push(Op::True, Some(token));
                }
                Kind::False => {
                    self.push(Op::False, Some(token));
                }
                Kind::Int(s) => {
                    self.push_constant(Value::Int(s.parse().unwrap()), Some(token));
                }
                Kind::String(s) => {
                    self.push_constant(
                        Value::String(Rc::new(s[1..(s.len() - 1)].to_string())),
                        Some(token),
                    );
                }
                Kind::If => {
                    self.block(state); // test
                    let jump_over_then = self.push(Op::Unreachable, Some(token));
                    self.push(Op::Pop, Some(token)); // pop result of test
                    self.block(state); // then
                    let jump_over_else = self.push(Op::Unreachable, Some(token));
                    self.code[jump_over_then] =
                        Op::JumpIfFalse((self.code.len() - jump_over_then).try_into().unwrap());
                    self.push(Op::Pop, Some(token)); // pop result of test
                    self.block(state); // else
                    self.code[jump_over_else] =
                        Op::Jump((self.code.len() - jump_over_else).try_into().unwrap());
                }
                Kind::Let => {
                    let name = self.identifier(state);
                    state.vars.push(name);
                    self.block(state);
                    self.block(state);
                    state.vars.pop();
                    self.push(Op::Squash, Some(token));
                }
                Kind::Var(name) => {
                    let i = self.resolve_var(state, name);
                    self.push(Op::Var(i), Some(token));
                }
                _ => todo!(),
            }
        }
    }

    fn block(&mut self, state: &mut State<'src>) {
        let token = state.tokens.peek().unwrap();
        let col = token.col;
        self.expression(state);
        while let Some(token) = state.tokens.peek() {
            if token.col != col {
                break;
            }
            self.push(Op::Pop, None);
            self.expression(state);
        }
    }
}
