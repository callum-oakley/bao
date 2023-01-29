use std::{iter::Peekable, str::CharIndices};

#[derive(Debug)]
pub enum Kind<'src> {
    CloseParen,
    False,
    Identifier(&'src str),
    If,
    Int(i32),
    Let,
    Nil,
    OpenParen,
    Pipe,
    True,
}

#[derive(Debug)]
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
                _ if c.is_ascii_digit()
                    || c == '-' && self.chars.peek().map_or(false, |(_, c)| c.is_ascii_digit()) =>
                {
                    let (mut j, mut c) = (i, c);
                    while self.chars.peek().map_or(false, |(_, c)| c.is_ascii_digit()) {
                        // safe to unwrap because we just peeked
                        (j, c) = self.next_char().unwrap();
                    }
                    j += c.len_utf8();
                    // safe to unwrap because we've checked the number matches -?[0-9]+
                    // TODO what if it's too big?
                    Kind::Int(self.src[i..j].parse().unwrap())
                }
                _ => {
                    let (mut j, mut c) = (i, c);
                    while self
                        .chars
                        .peek()
                        .map_or(false, |(_, c)| is_identifier_char(*c))
                    {
                        // safe to unwrap because we just peeked
                        (j, c) = self.next_char().unwrap();
                    }
                    j += c.len_utf8();
                    match &self.src[i..j] {
                        "false" => Kind::False,
                        "if" => Kind::If,
                        "let" => Kind::Let,
                        "nil" => Kind::Nil,
                        "true" => Kind::True,
                        s => Kind::Identifier(s),
                    }
                }
            };
            Token { kind, line, col }
        })
    }
}

fn is_identifier_char(c: char) -> bool {
    !(c == '(' || c == ')' || c == '|' || c.is_whitespace())
}
