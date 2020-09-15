extern crate caklimas_rust_gameboy as rust_gameboy;

fn main() {
    let bytes = vec![0; 1000];
    println!("{}", rust_gameboy::run(bytes));
}