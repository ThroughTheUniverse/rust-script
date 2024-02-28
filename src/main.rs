use std::{
    env::args,
    fs,
    io::{self, Write},
    process::exit,
};

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

    match args.len() {
        1 => repl(&mut vm),
        2 => {
            run_file(&mut vm, &args[1]).expect(&format!("Could not open file {}", &args[1]));
            exit(74);
        }
        _ => {
            println!("Usage: rust_script [script]");
            exit(64);
        }
    }
}

fn repl(vm: &mut VirtualMachine) {
    println!("RustScript REPL (Ctrl+C to exit)");
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Pass the input to your compiler or interpreter
        let result = vm.interpret(&input);

        match result {
            Ok(_) => {}
            Err(compiler::InterpretError::CompileError) => exit(65),
            Err(compiler::InterpretError::RuntimeError) => exit(70),
        }
    }
}

fn run_file(vm: &mut VirtualMachine, path: &str) -> io::Result<()> {
    let file = fs::read_to_string(path)?;
    let result = vm.interpret(&file);

    match result {
        Ok(()) => Ok(()),
        Err(compiler::InterpretError::CompileError) => exit(65),
        Err(compiler::InterpretError::RuntimeError) => exit(70),
    }
}
