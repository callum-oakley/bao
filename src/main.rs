mod code;
mod token;
mod value;
mod vm;

fn main() {
    println!(
        "{:?}",
        token::Tokens::new(
            "let fib |n|
               if (<= n 0) 0
               let loop |a b n|
                 if (= 1 n) b
                 (loop b (+ a b) (- n 1))
               (loop 0 1 n)"
        )
        .map(|t| t.kind)
        .collect::<Vec<_>>()
    );
}
