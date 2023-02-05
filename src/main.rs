mod compiler;
mod token;
mod value;
mod vm;

fn main() {
    println!(
        "{:?}",
        vm::VM::new(compiler::compile(token::Tokens::new(
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
