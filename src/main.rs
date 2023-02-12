mod compiler;
mod native;
mod token;
mod value;
mod vm;

fn main() {
    let src = r#"

      let fib |fib <= + - n|
        if (<= n 1) n
        (+ (fib fib <= + - (- n 2)) (fib fib <= + - (- n 1)))

      (fib fib <= + - 10)

    "#;
    println!(
        "{:?}",
        vm::VM::new(compiler::compile(token::Tokens::new(src))).run()
    );
}
