use anyhow::Result;

mod parser;
mod tokenizer;

fn main() -> Result<()> {
    let path = "scrap.bao";
    let src = std::fs::read_to_string(path)?;
    println!("{:?}", parser::parse(path, &src)?);
    Ok(())
}
