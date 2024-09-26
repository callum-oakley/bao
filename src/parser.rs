use std::{iter::Peekable, path::Path};

use anyhow::{anyhow, bail, ensure, Result};

use crate::tokenizer::{Location, Token, TokenKind, Tokens};

#[derive(Debug)]
pub enum Exp<'a> {
    Call(Box<Exp<'a>>, Vec<Exp<'a>>),
    Fn(Option<&'a str>, Vec<&'a str>, Vec<Stmt<'a>>, Box<Exp<'a>>),
    Int(String),
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
            TokenKind::Var => self.parse_var(),
            TokenKind::Int => self.parse_int(),
            TokenKind::Char => self.parse_char(),
            TokenKind::String => self.parse_string(),
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

    fn parse_var(&mut self) -> Result<Exp<'a>> {
        let token = self.consume(TokenKind::Var)?;
        Ok(Exp::Var(token.as_str()))
    }

    fn parse_int(&mut self) -> Result<Exp<'a>> {
        let token = self.consume(TokenKind::Int)?;
        Ok(Exp::Int(token.as_str().to_owned()))
    }

    fn parse_char(&mut self) -> Result<Exp<'a>> {
        let token = self.consume(TokenKind::Char)?;
        let bytes = token.as_bytes();
        ensure!(
            bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\'',
            token.location.error(anyhow!("malformed char")),
        );
        let bytes = &bytes[1..bytes.len() - 1];
        let bytes = unescape(token.location, bytes)?;
        ensure!(
            bytes.len() == 1,
            token.location.error(anyhow!("malformed char")),
        );
        Ok(Exp::Int(bytes[0].to_string()))
    }

    fn parse_string(&mut self) -> Result<Exp<'a>> {
        let token = self.consume(TokenKind::String)?;
        let bytes = token.as_bytes();
        ensure!(
            bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"',
            token.location.error(anyhow!("malformed string"))
        );
        let bytes = &bytes[1..bytes.len() - 1];
        let bytes = unescape(token.location, bytes)?;
        let mut exp = Exp::Var("nil");
        for byte in bytes.iter().rev() {
            exp = Exp::Call(
                Box::new(Exp::Var("cons")),
                vec![Exp::Int(byte.to_string()), exp],
            );
        }
        Ok(exp)
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

fn unescape(location: Location, bytes: &[u8]) -> Result<Vec<u8>> {
    let mut bytes = bytes.iter();
    let mut res = Vec::new();
    while let Some(byte) = bytes.next() {
        res.push(match byte {
            b'\\' => match bytes.next() {
                Some(b't') => b'\t',
                Some(b'n') => b'\n',
                Some(c) => *c,
                _ => bail!(location.error(anyhow!("malformed string"))),
            },
            c => *c,
        })
    }
    Ok(res)
}
