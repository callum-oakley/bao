use std::iter::Peekable;

use crate::{
    token::{Kind, Token, Tokens},
    value::Value,
};

#[derive(Debug)]
pub enum Op {
    Constant(u16),
    False,
    Nil,
    True,
}

#[derive(Debug)]
pub struct Chunk<'src> {
    code: Vec<Op>,
    constants: Vec<Value>,
    // TODO this is a huge waste of space
    debug_info: Vec<Token<'src>>,
}

impl<'src> Chunk<'src> {
    pub fn new(tokens: Tokens<'src>) -> Self {
        let mut tokens = tokens.peekable();
        let mut chunk = Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            debug_info: Vec::new(),
        };
        while tokens.peek().is_some() {
            chunk.expression(&mut tokens);
        }
        chunk
    }

    fn push(&mut self, op: Op, token: Token<'src>) {
        self.code.push(op);
        self.debug_info.push(token);
    }

    fn push_constant(&mut self, constant: Value, token: Token<'src>) {
        let i = self.constants.len().try_into().unwrap();
        self.constants.push(constant);
        self.push(Op::Constant(i), token);
    }

    fn expression(&mut self, tokens: &mut Peekable<Tokens<'src>>) {
        if let Some(token) = tokens.next() {
            match token.kind {
                Kind::Nil => self.push(Op::Nil, token),
                Kind::True => self.push(Op::True, token),
                Kind::False => self.push(Op::False, token),
                Kind::Int(s) => self.push_constant(Value::Int(s.parse().unwrap()), token),
                _ => todo!(),
            }
        }
    }
}
