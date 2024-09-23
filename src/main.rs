use std::{io, path::Path};

use anyhow::Result;

mod compiler;
mod parser;
mod tokenizer;

fn main() -> Result<()> {
    compiler::compile(&mut io::stdout(), Path::new("scrap.bao"))
}
