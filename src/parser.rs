use std::{iter::Peekable, path::Path};

use anyhow::{anyhow, bail, Result};

use crate::tokenizer::{Token, TokenKind, Tokens};

#[derive(Debug)]
pub enum Exp<'a> {
    Call(Box<Exp<'a>>, Vec<Exp<'a>>),
    Fn(Option<&'a str>, Vec<&'a str>, Vec<Stmt<'a>>, Box<Exp<'a>>),
    Int(&'a str),
    Var(&'a str),
}

#[derive(Debug)]
pub enum Stmt<'a> {
    Let(&'a str, Box<Exp<'a>>),
    Exp(Exp<'a>),
}

pub fn parse<'a>(path: &'a Path, src: &'a str) -> Result<Vec<Stmt<'a>>> {
    let tokens = Tokens::new(path, src).peekable();
    let mut parser = Parser { path, tokens };
    let block = parser.parse_block()?;

    if !parser.is_eof() {
        bail!(parser.peek()?.unexpected());
    }

    Ok(block)
}

struct Parser<'a> {
    path: &'a Path,
    tokens: Peekable<Tokens<'a>>,
}

impl<'a> Parser<'a> {
    fn is_eof(&mut self) -> bool {
        self.tokens.peek().is_none()
    }

    fn peek(&mut self) -> Result<&Token<'a>> {
        match self.tokens.peek() {
            Some(token) => token
                .as_ref()
                // We need to return an owned error but peeking only gives us a reference so flatten
                // the error in to a new anyhow::Error.
                .map_err(|err| anyhow!("{err}")),
            None => bail!("EOF parsing {}", self.path.display()),
        }
    }

    fn next(&mut self) -> Result<Token<'a>> {
        match self.tokens.next() {
            Some(token) => token,
            None => bail!("EOF parsing {}", self.path.display()),
        }
    }

    fn consume(&mut self, expected: TokenKind) -> Result<Token<'a>> {
        let token = self.next()?;
        if token.kind != expected {
            bail!(token.unexpected());
        }
        Ok(token)
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt<'a>>> {
        let mut block = Vec::new();
        while !self.is_eof() && self.peek()?.kind != TokenKind::RBrace {
            block.push(self.parse_stmt()?);
        }
        Ok(block)
    }

    fn parse_stmt(&mut self) -> Result<Stmt<'a>> {
        if self.peek()?.kind == TokenKind::Let {
            self.parse_let()
        } else {
            Ok(Stmt::Exp(self.parse_exp()?))
        }
    }

    fn parse_let(&mut self) -> Result<Stmt<'a>> {
        self.consume(TokenKind::Let)?;
        let var = self.consume(TokenKind::Var)?;
        let exp = self.parse_exp()?;
        Ok(Stmt::Let(var.as_str(), Box::new(exp)))
    }

    fn parse_exp(&mut self) -> Result<Exp<'a>> {
        let token = self.peek()?;
        let exp = match token.kind {
            TokenKind::Fn => self.parse_fn(),
            TokenKind::Int => self.parse_int(),
            TokenKind::Var => self.parse_var(),
            TokenKind::LParen
            | TokenKind::RParen
            | TokenKind::LBrace
            | TokenKind::RBrace
            | TokenKind::Let => {
                bail!(token.unexpected())
            }
        }?;
        if !self.is_eof() && self.peek()?.kind == TokenKind::LParen {
            self.parse_call(exp)
        } else {
            Ok(exp)
        }
    }

    fn parse_fn(&mut self) -> Result<Exp<'a>> {
        self.consume(TokenKind::Fn)?;

        let mut name = None;
        let maybe_var = self.peek()?;
        if maybe_var.kind == TokenKind::Var {
            name = Some(self.next()?.as_str());
        }

        let mut params = Vec::new();
        self.consume(TokenKind::LParen)?;
        while self.peek()?.kind == TokenKind::Var {
            params.push(self.next()?.as_str());
        }
        self.consume(TokenKind::RParen)?;

        if self.peek()?.kind == TokenKind::LBrace {
            self.consume(TokenKind::LBrace)?;
            let mut body = self.parse_block()?;
            self.consume(TokenKind::RBrace)?;

            if let Some(Stmt::Exp(_)) = body.last() {
                let Some(Stmt::Exp(res)) = body.pop() else {
                    unreachable!()
                };
                Ok(Exp::Fn(name, params, body, Box::new(res)))
            } else {
                Ok(Exp::Fn(name, params, body, Box::new(Exp::Var("nil"))))
            }
        } else {
            let res = self.parse_exp()?;
            Ok(Exp::Fn(name, params, Vec::new(), Box::new(res)))
        }
    }

    fn parse_int(&mut self) -> Result<Exp<'a>> {
        let token = self.consume(TokenKind::Int)?;
        Ok(Exp::Int(token.as_str()))
    }

    fn parse_var(&mut self) -> Result<Exp<'a>> {
        let token = self.consume(TokenKind::Var)?;
        Ok(Exp::Var(token.as_str()))
    }

    fn parse_call(&mut self, function: Exp<'a>) -> Result<Exp<'a>> {
        self.consume(TokenKind::LParen)?;
        let mut args = Vec::new();
        while self.peek()?.kind != TokenKind::RParen {
            args.push(self.parse_exp()?);
        }
        self.consume(TokenKind::RParen)?;

        let exp = Exp::Call(Box::new(function), args);

        if !self.is_eof() && self.peek()?.kind == TokenKind::LParen {
            self.parse_call(exp)
        } else {
            Ok(exp)
        }
    }
}
