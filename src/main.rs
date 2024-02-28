use std::{
    env::args,
    fs,
    io::{self, stdin, stdout, Write},
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

    // match args.len() {
    //     1 => repl(&mut vm),
    //     2 => run_file(&mut vm, &args[1]).expect("not file"),
    //     _ => {
    //         println!("Usage: rusts [script]")
    //     }
    // }
    let _ = vm.interpret(
        r#"
        struct Point {
            new() {
                self.x = 0;
                self.y = 0;
            }

            printf() {
                print("( ");
                print(self.x);
                print(", ");
                print(self.y);
                print(" )");
            }
        }
        struct Origin {
            new() {
                self.point = Point();
                self.shape = "red";
            }
        }
        let a = Origin();
        a.point.printf();
        print(a.shape);
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
