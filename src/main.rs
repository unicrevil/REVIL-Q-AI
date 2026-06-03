use std::io::{self, Write};

fn main() {
    println!("REVIL-Q-AI v0.1.0 ⚛️");
    println!("Kernel Quântico Online\n");

    loop {
        print!("REVIL> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "exit" { break; }
        if input.starts_with("ask ") {
            println!("Q-AI: Processando em 0.003ms... 🚬");
            println!("Q-AI: Kernel respondendo: {}\n", &input[4..]);
        }
    }
  }
