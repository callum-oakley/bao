use std::iter::Peekable;

use anyhow::{anyhow, bail, Context, Result};

use crate::tokenizer::{Location, Token, TokenKind, Tokens};

#[derive(Debug)]
pub struct Exp<'a> {
    kind: ExpKind<'a>,
    location: Location<'a>,
}

#[derive(Debug)]
pub enum ExpKind<'a> {
    Apply(Box<Exp<'a>>, Vec<Exp<'a>>),
    Block(Vec<Exp<'a>>),
    Fn(Option<&'a str>, Vec<&'a str>, Box<Exp<'a>>),
    Int(i64),
    Let(&'a str, Box<Exp<'a>>),
    String(String),
    Use(String),
    Var(&'a str),
}

impl<'a> Exp<'a> {
    fn new(kind: ExpKind<'a>, location: Location<'a>) -> Self {
        Exp { kind, location }
    }
}

pub fn parse<'a>(path: &'a str, src: &'a str) -> Result<Vec<Exp<'a>>> {
    let tokens = Tokens::new(path, src).peekable();
    let mut parser = Parser { path, src, tokens };
    let routine = parser.parse_routine()?;
    if !parser.is_eof() {
        bail!(parser.peek()?.unexpected());
    }
    Ok(routine)
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

    fn parse_routine(&mut self) -> Result<Vec<Exp<'a>>> {
        let mut routine = Vec::new();
        while !self.is_eof() && self.peek()?.kind != TokenKind::RBrace {
            routine.push(self.parse_exp()?);
        }
        Ok(routine)
    }

    fn parse_exp(&mut self) -> Result<Exp<'a>> {
        let token = self.peek()?;
        let exp = match token.kind {
            TokenKind::Char => self.parse_char(),
            TokenKind::Fn => self.parse_fn(),
            TokenKind::Int => self.parse_int(),
            TokenKind::LBrace => self.parse_block(),
            TokenKind::Let => self.parse_let(),
            TokenKind::String => self.parse_string(),
            TokenKind::Use => self.parse_use(),
            TokenKind::Var => self.parse_var(),
            TokenKind::LParen | TokenKind::RParen | TokenKind::RBrace => bail!(token.unexpected()),
        }?;
        if self.peek()?.kind == TokenKind::LParen {
            self.parse_apply(exp)
        } else {
            Ok(exp)
        }
    }

    fn parse_char(&mut self) -> Result<Exp<'a>> {
        let token = self.consume(TokenKind::Char)?;
        let c = match token.as_bytes() {
            b"'\\n'" => b'\n',
            b"'\\t'" => b'\t',
            [b'\'', c, b'\''] | [b'\'', b'\\', c, b'\''] => *c,
            _ => bail!(token.location.error(anyhow!("malformed char"))),
        };
        Ok(Exp::new(ExpKind::Int(c.into()), token.location))
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

        let body = self.parse_exp()?;

        Ok(Exp::new(
            ExpKind::Fn(name, params, Box::new(body)),
            location,
        ))
    }

    fn parse_int(&mut self) -> Result<Exp<'a>> {
        let token = self.consume(TokenKind::Int)?;
        Ok(Exp::new(
            ExpKind::Int(
                token
                    .as_str()
                    .parse()
                    .with_context(|| token.location.error(anyhow!("malformed int")))?,
            ),
            token.location,
        ))
    }

    fn parse_block(&mut self) -> Result<Exp<'a>> {
        let location = self.consume(TokenKind::LBrace)?.location;
        let routine = self.parse_routine()?;
        self.consume(TokenKind::RBrace)?;
        Ok(Exp::new(ExpKind::Block(routine), location))
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

    fn parse_string(&mut self) -> Result<Exp<'a>> {
        let location = self.peek()?.location;
        Ok(Exp::new(
            ExpKind::String(self.parse_string_inner()?),
            location,
        ))
    }

    fn parse_use(&mut self) -> Result<Exp<'a>> {
        let location = self.consume(TokenKind::Use)?.location;
        let s = self.parse_string_inner()?;
        Ok(Exp::new(ExpKind::Use(s), location))
    }

    fn parse_var(&mut self) -> Result<Exp<'a>> {
        let token = self.consume(TokenKind::Var)?;
        Ok(Exp::new(ExpKind::Var(token.as_str()), token.location))
    }

    fn parse_apply(&mut self, function: Exp<'a>) -> Result<Exp<'a>> {
        let location = self.consume(TokenKind::LParen)?.location;
        let mut args = Vec::new();
        while self.peek()?.kind != TokenKind::RParen {
            args.push(self.parse_exp()?);
        }
        self.consume(TokenKind::RParen)?;

        let exp = Exp::new(ExpKind::Apply(Box::new(function), args), location);

        if !self.is_eof() && self.peek()?.kind == TokenKind::LParen {
            self.parse_apply(exp)
        } else {
            Ok(exp)
        }
    }

    fn parse_string_inner(&mut self) -> Result<String> {
        let token = self.consume(TokenKind::String)?;
        match token.as_bytes() {
            [b'"', cs @ .., b'"'] => {
                let mut s = Vec::new();
                let mut escape = false;
                for c in cs {
                    if escape {
                        match c {
                            b'n' => {
                                s.push(b'\n');
                            }
                            b't' => {
                                s.push(b'\t');
                            }
                            _ => {
                                s.push(*c);
                            }
                        }
                        escape = false;
                    } else {
                        match c {
                            b'\\' => {
                                escape = true;
                            }
                            _ => {
                                s.push(*c);
                            }
                        }
                    }
                }
                Ok(String::from_utf8(s)
                    .with_context(|| token.location.error(anyhow!("malformed string")))?)
            }
            _ => Err(token.location.error(anyhow!("malformed string"))),
        }
    }
}
