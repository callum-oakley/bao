use std::iter::Peekable;

use anyhow::{anyhow, bail, Result};

use crate::tokenizer::{Location, Token, TokenKind, Tokens};

#[derive(Debug)]
pub struct Exp<'a> {
    pub kind: ExpKind<'a>,
    pub location: Location<'a>,
}

#[derive(Debug)]
pub enum ExpKind<'a> {
    Call(Box<Exp<'a>>, Vec<Exp<'a>>),
    Fn(Option<&'a str>, Vec<&'a str>, Vec<Exp<'a>>),
    Num(&'a str),
    Let(&'a str, Box<Exp<'a>>),
    String(&'a str),
    Var(&'a str),
}

impl<'a> Exp<'a> {
    pub fn new(kind: ExpKind<'a>, location: Location<'a>) -> Self {
        Exp { kind, location }
    }
}

pub fn parse<'a>(path: &'a str, src: &'a str) -> Result<Exp<'a>> {
    let tokens = Tokens::new(path, src).peekable();
    let mut parser = Parser { path, src, tokens };
    let block = parser.parse_block()?;

    if !parser.is_eof() {
        bail!(parser.peek()?.unexpected());
    }

    let location = Location {
        path,
        src,
        start: 0,
        end: src.len(),
    };
    Ok(Exp::new(
        ExpKind::Call(
            Box::new(Exp::new(ExpKind::Fn(None, Vec::new(), block), location)),
            Vec::new(),
        ),
        location,
    ))
}

struct Parser<'a> {
    path: &'a str,
    src: &'a str,
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
            None => bail!("EOF parsing {}", self.path),
        }
    }

    fn next(&mut self) -> Result<Token<'a>> {
        match self.tokens.next() {
            Some(token) => token,
            None => bail!("EOF parsing {}", self.path),
        }
    }

    fn consume(&mut self, expected: TokenKind) -> Result<Token<'a>> {
        let token = self.next()?;
        if token.kind != expected {
            bail!(token.unexpected());
        }
        Ok(token)
    }

    fn parse_block(&mut self) -> Result<Vec<Exp<'a>>> {
        let mut block = Vec::new();
        while !self.is_eof() && self.peek()?.kind != TokenKind::RBrace {
            block.push(self.parse_exp()?);
        }
        Ok(block)
    }

    fn parse_exp(&mut self) -> Result<Exp<'a>> {
        let token = self.peek()?;
        let exp = match token.kind {
            TokenKind::Fn => self.parse_fn(),
            TokenKind::Let => self.parse_let(),
            TokenKind::Num => self.parse_num(),
            TokenKind::String => self.parse_string(),
            TokenKind::Var => self.parse_var(),
            TokenKind::LParen | TokenKind::RParen | TokenKind::LBrace | TokenKind::RBrace => {
                bail!(token.unexpected())
            }
        }?;
        if self.peek()?.kind == TokenKind::LParen {
            self.parse_call(exp)
        } else {
            Ok(exp)
        }
    }

    fn parse_fn(&mut self) -> Result<Exp<'a>> {
        let location = self.consume(TokenKind::Fn)?.location;

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

        let mut body = Vec::new();
        if self.peek()?.kind == TokenKind::LBrace {
            self.consume(TokenKind::LBrace)?;
            while self.peek()?.kind != TokenKind::RBrace {
                body.push(self.parse_exp()?);
            }
            self.consume(TokenKind::RBrace)?;
        } else {
            body.push(self.parse_exp()?);
        }

        Ok(Exp::new(ExpKind::Fn(name, params, body), location))
    }

    fn parse_let(&mut self) -> Result<Exp<'a>> {
        let location = self.consume(TokenKind::Let)?.location;
        let var = self.consume(TokenKind::Var)?;
        let exp = self.parse_exp()?;
        Ok(Exp::new(
            ExpKind::Let(var.as_str(), Box::new(exp)),
            location,
        ))
    }

    fn parse_num(&mut self) -> Result<Exp<'a>> {
        let token = self.consume(TokenKind::Num)?;
        Ok(Exp::new(ExpKind::Num(token.as_str()), token.location))
    }

    fn parse_string(&mut self) -> Result<Exp<'a>> {
        let token = self.consume(TokenKind::String)?;
        Ok(Exp::new(ExpKind::String(token.as_str()), token.location))
    }

    fn parse_var(&mut self) -> Result<Exp<'a>> {
        let token = self.consume(TokenKind::Var)?;
        Ok(Exp::new(ExpKind::Var(token.as_str()), token.location))
    }

    fn parse_call(&mut self, function: Exp<'a>) -> Result<Exp<'a>> {
        let location = self.consume(TokenKind::LParen)?.location;
        let mut args = Vec::new();
        while self.peek()?.kind != TokenKind::RParen {
            args.push(self.parse_exp()?);
        }
        self.consume(TokenKind::RParen)?;

        let exp = Exp::new(ExpKind::Call(Box::new(function), args), location);

        if !self.is_eof() && self.peek()?.kind == TokenKind::LParen {
            self.parse_call(exp)
        } else {
            Ok(exp)
        }
    }
}
