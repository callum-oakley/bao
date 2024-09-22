use std::io;

use anyhow::Result;

mod compiler;
mod parser;
mod tokenizer;

fn main() -> Result<()> {
    let path = "scrap.bao";
    let src = std::fs::read_to_string(path)?;
    let exp = parser::parse(path, &src)?;
    compiler::write_js(&mut io::stdout(), &exp)?;
    Ok(())
}
