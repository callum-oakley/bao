use std::iter::Peekable;

use crate::{
    token::{Kind, Token, Tokens},
    value::Value,
};

#[derive(Debug)]
pub enum Op {
    Constant(u16),
    False,
    Jump(u16),
    JumpIfFalse(u16),
    Nil,
    Pop,
    Return,
    True,
    Unreachable,
}

#[derive(Debug)]
pub struct Chunk<'src> {
    pub code: Vec<Op>,
    pub constants: Vec<Value>,
    // TODO this is a huge waste of space
    pub debug_info: Vec<Option<Token<'src>>>,
}

impl<'src> Chunk<'src> {
    pub fn new(tokens: Tokens<'src>) -> Self {
        let mut tokens = tokens.peekable();
        let mut chunk = Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            debug_info: Vec::new(),
        };
        chunk.block(&mut tokens);
        if !tokens.next().is_none() {
            todo!()
        }
        chunk.push(Op::Return, None);
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

    fn expression(&mut self, tokens: &mut Peekable<Tokens<'src>>) {
        if let Some(token) = tokens.next() {
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
                Kind::If => {
                    self.block(tokens); // test
                    let jump_over_then = self.push(Op::Unreachable, Some(token));
                    self.push(Op::Pop, Some(token)); // pop result of test
                    self.block(tokens); // then
                    let jump_over_else = self.push(Op::Unreachable, Some(token));
                    self.code[jump_over_then] =
                        Op::JumpIfFalse((self.code.len() - jump_over_then).try_into().unwrap());
                    self.push(Op::Pop, Some(token)); // pop result of test
                    self.block(tokens); // else
                    self.code[jump_over_else] =
                        Op::Jump((self.code.len() - jump_over_else).try_into().unwrap());
                }
                _ => todo!(),
            }
        }
    }

    fn block(&mut self, tokens: &mut Peekable<Tokens<'src>>) {
        if let Some(token) = tokens.peek() {
            let col = token.col;
            self.expression(tokens);
            while let Some(token) = tokens.peek() {
                if token.col != col {
                    break;
                }
                self.push(Op::Pop, None);
                self.expression(tokens);
            }
        }
    }
}
