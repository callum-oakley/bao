mod compiler;
mod token;
mod value;
mod vm;

fn main() {
    let src = r#"
        let foo ||
          42
        let id |x|
          x
        if true ((id id) (foo)) (id nil)
    "#;
    println!("{:?}", vm::VM::new(compiler::compile(token::Tokens::new(src))).run());
}
