mod code;
mod token;
mod value;
mod vm;

fn main() {
    println!(
        "{:?}",
        vm::VM::new(code::Chunk::new(token::Tokens::new("if false 42 -5"))).run()
    );
}
