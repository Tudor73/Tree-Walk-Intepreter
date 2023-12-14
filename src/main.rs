use std::{env, io::Write, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Too many arguments");
        process::exit(1);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(file_path: &String) {
    let content = std::fs::read_to_string(file_path).expect("File not found or something");
    println!("{content}");
    println!("Running file {}", file_path);
}
fn run_prompt() {
    let stdin = std::io::stdin();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        stdin.read_line(&mut line).expect("error when reading line");
        if line.trim() == "exit" {
            break;
        }
        run(line)
    }
}

fn run(source: String) {
    println!("{source}");
}
