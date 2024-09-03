use anyhow::{bail, Result};

pub struct Token<'a> {
    pub kind: Kind,
    pub location: Location<'a>,
}

pub enum Kind {
    Char,
    Comma,
    Eq,
    Int,
    LBrace,
    Let,
    LParen,
    RBrace,
    RParen,
    Semi,
    String,
    Use,
    Var,
}

pub struct Location<'a> {
    path: &'a str,
    src: &'a str,
    start: usize,
    end: usize,
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &self.location.src[self.location.start..self.location.end]
        )
    }
}

pub struct Tokens<'a> {
    path: &'a str,
    src: &'a str,
    offset: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(path: &'a str, src: &'a str) -> Self {
        Self {
            path,
            src,
            offset: 0,
        }
    }

    fn token(&self, kind: Kind, start: usize, end: usize) -> Token<'a> {
        Token {
            kind,
            location: Location {
                path: self.path,
                src: self.src,
                start,
                end,
            },
        }
    }

    fn is_eof(&self) -> bool {
        self.src.as_bytes().get(self.offset).is_none()
    }

    fn peek(&self) -> Result<u8> {
        match self.src.as_bytes().get(self.offset).copied() {
            Some(c) => Ok(c),
            None => bail!("EOF reading {}", self.path),
        }
    }

    fn next(&mut self) -> Result<u8> {
        let c = self.peek()?;
        self.offset += 1;
        Ok(c)
    }

    fn consume(&mut self, expected: u8) -> Result<()> {
        let offset = self.offset;
        let c = self.next()?;
        if c != expected {
            let (line, col) = offset_to_line_and_col(self.src, offset);
            bail!(
                "expected {} but got {} at {}:{}:{}",
                char::from(expected),
                char::from(c),
                self.path,
                line,
                col,
            )
        };
        Ok(())
    }

    fn consume_whitespace(&mut self) -> Result<()> {
        while !self.is_eof() {
            match self.peek()? {
                b'#' => while self.next()? != b'\n' {},
                c if c.is_ascii_whitespace() => {
                    self.next()?;
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn tokenize_punctuation(&mut self, c: u8, kind: Kind) -> Result<Token<'a>> {
        let start = self.offset;
        self.consume(c)?;
        Ok(self.token(kind, start, self.offset))
    }

    fn tokenize_quoted(&mut self, quote: u8, kind: Kind) -> Result<Token<'a>> {
        let start = self.offset;
        self.consume(quote)?;
        loop {
            let c = self.next()?;
            if c == quote {
                break;
            }
            if c == b'\\' {
                self.next()?;
            }
        }
        Ok(self.token(kind, start, self.offset))
    }

    fn tokenize_int(&mut self) -> Result<Token<'a>> {
        let start = self.offset;
        if self.peek()? == b'-' {
            self.next()?;
        }
        while self.peek()?.is_ascii_digit() {
            self.next()?;
        }
        Ok(self.token(Kind::Int, start, self.offset))
    }

    fn tokenize_word(&mut self) -> Result<Token<'a>> {
        fn is_word_char(c: u8) -> bool {
            c.is_ascii_alphanumeric() || b"_!?".contains(&c)
        }
        let start = self.offset;
        while is_word_char(self.peek()?) {
            self.next()?;
        }
        let kind = match &self.src[start..self.offset] {
            "let" => Kind::Let,
            "use" => Kind::Use,
            _ => Kind::Var,
        };
        Ok(self.token(kind, start, self.offset))
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Err(err) = self.consume_whitespace() {
            Some(Err(err))
        } else if self.is_eof() {
            None
        } else {
            let token = self.peek().and_then(|c| match c {
                b',' => self.tokenize_punctuation(b',', Kind::Comma),
                b'=' => self.tokenize_punctuation(b'=', Kind::Eq),
                b'{' => self.tokenize_punctuation(b'{', Kind::LBrace),
                b'(' => self.tokenize_punctuation(b'(', Kind::LParen),
                b'}' => self.tokenize_punctuation(b'}', Kind::RBrace),
                b')' => self.tokenize_punctuation(b')', Kind::RParen),
                b';' => self.tokenize_punctuation(b';', Kind::Semi),
                b'-' | b'0'..=b'9' => self.tokenize_int(),
                b'\'' => self.tokenize_quoted(b'\'', Kind::Char),
                b'"' => self.tokenize_quoted(b'"', Kind::String),
                _ => self.tokenize_word(),
            });
            Some(token)
        }
    }
}

fn offset_to_line_and_col(src: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for i in 0..offset {
        if src.as_bytes()[i] == b'\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}