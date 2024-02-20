use scanner::Scanner;

mod chunk;
mod debug;
mod object;
mod scanner;
mod value;
mod vm;

fn main() {
    let mut scanner = Scanner::new("or\0");
    let token = scanner.scan_token();
    println!("{:?}", token);
}
