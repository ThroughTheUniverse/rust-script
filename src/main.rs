use std::{
    env::args,
    fs,
    io::{self, stdin, stdout, Write},
};

use chunk::{opcode::OpCode, Chunk};
use scanner::Scanner;
use vm::VirtualMachine;

mod chunk;
mod compiler;
mod object;
mod scanner;
mod value;
mod vm;

fn main() {
    let args: Vec<String> = args().collect();
    let mut vm = VirtualMachine::new();

    // match args.len() {
    //     1 => repl(&mut vm),
    //     2 => run_file(&mut vm, &args[1]).expect("not file"),
    //     _ => {
    //         println!("Usage: rusts [script]")
    //     }
    // }
    let _ = vm.interpret(
        r#"
        fn fib(n) {
  if (n < 2) return n;
  return fib(n - 2) + fib(n - 1);
}

let start = clock();
print fib(30);
print clock() - start;
        "#,
    );
}

fn repl(vm: &mut VirtualMachine) {
    let mut line = String::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut line);
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        vm.interpret(line);
    }
}

fn run_file(vm: &mut VirtualMachine, path: &str) -> io::Result<()> {
    let file = fs::read_to_string(path)?;
    vm.interpret(&file);
    Ok(())
}
