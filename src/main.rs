mod code;
mod token;
mod value;
mod vm;

fn main() {
    println!(
        "{:?}",
        vm::VM::new(code::Chunk::new(token::Tokens::new(
            "
            let a 1
            let a 22
              a
            a"
        )))
        .run()
    );
}
