mod code;
mod token;
mod value;
mod vm;

fn main() {
    println!(
        "{:?}",
        vm::VM::new(code::Chunk::new(token::Tokens::new(
            r#"
            let a "foo bar"
            let a 22
              a
            a
            "#
        )))
        .run()
    );
}
