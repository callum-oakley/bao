use std::{iter::Peekable, str::CharIndices};

#[derive(Debug, Copy, Clone)]
pub enum Kind<'src> {
    CloseParen,
    False,
    If,
    Int(&'src str),
    Let,
    Nil,
    OpenParen,
    Pipe,
    True,
    Var(&'src str),
    String(&'src str),
}

#[derive(Debug, Copy, Clone)]
pub struct Token<'src> {
    pub kind: Kind<'src>,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Tokens<'src> {
    src: &'src str,
    chars: Peekable<CharIndices<'src>>,
    line: usize,
    col: usize,
}

impl<'src> Tokens<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            chars: src.char_indices().peekable(),
            line: 0,
            col: 0,
        }
    }

    fn next_char(&mut self) -> Option<(usize, char)> {
        let res = self.chars.next();
        if let Some((_, c)) = res {
            if c == '\n' {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += c.len_utf8();
            }
        }
        res
    }
}

impl<'src> Iterator for Tokens<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.chars.peek().map_or(false, |(_, c)| c.is_whitespace()) {
            self.next_char();
        }

        let line = self.line;
        let col = self.col;

        self.next_char().map(|(i, c)| {
            let kind = match c {
                '(' => Kind::OpenParen,
                ')' => Kind::CloseParen,
                '|' => Kind::Pipe,
                '"' => {
                    let kind;
                    loop {
                        if let Some((j, c)) = self.next_char() {
                            if c == '"' {
                                kind = Kind::String(&self.src[i..(j + c.len_utf8())]);
                                break;
                            }
                        } else {
                            todo!();
                        }
                    }
                    kind
                }
                _ => {
                    let (mut j, mut c) = (i, c);
                    while self.chars.peek().map_or(false, |(_, c)| {
                        *c != '(' && *c != ')' && *c != '|' && *c != '"' && !c.is_whitespace()
                   }) {
                        // safe to unwrap because we just peeked
                        (j, c) = self.next_char().unwrap();
                    }
                    match &self.src[i..(j + c.len_utf8())] {
                        "false" => Kind::False,
                        "if" => Kind::If,
                        "let" => Kind::Let,
                        "nil" => Kind::Nil,
                        "true" => Kind::True,
                        s if s.parse::<i32>().is_ok() => Kind::Int(s),
                        s => Kind::Var(s),
                    }
                }
            };
            Token { kind, line, col }
        })
    }
}
