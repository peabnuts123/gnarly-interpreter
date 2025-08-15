use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;

use crate::{
    interpreter::{Interpreter, Operand},
    lexer::{Lexer, Token},
};

mod interpreter;
mod lexer;

#[derive(Parser)]
#[command(bin_name = "gnarly")]
#[command(name = "Gnarly")]
#[command(about = "The Gnarly language interpreter")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[arg(help = "Path to the program's main entrypoint. Start REPL if not provided.")]
    file: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    match cli.file {
        Some(file_path) => {
            run_file(file_path);
        }
        None => {
            run_repl();
        }
    }
}

fn run_file(file_path: PathBuf) {
    // Read the file contents
    let file_contents = match fs::read_to_string(&file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", file_path.display(), err);
            process::exit(1);
        }
    };

    // @TODO pre-processor

    // Lex file
    let lexer_result = match Lexer::scan(&file_contents) {
        Ok(lexer) => lexer,
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            process::exit(1);
        }
    };

    // @DEBUG Dump tokens
    println!("Tokens:");
    for token in lexer_result.token_list.iter() {
        // @TODO token string function
        match token {
            Token::NumberLiteral(value) => print!("[Number({})] ", value),
            Token::StringLiteral(value) => print!("[Number({})] ", value.replace("\n", "\\n")),
            Token::Operator(op) => print!("[Operator({})] ", op),
            Token::VariableIdentifier(var_name) => print!("[Variable(${var_name})] "),
        }
    }
    println!();

    // Run interpreter
    let mut interpreter = Interpreter::new();
    match interpreter.run(lexer_result.token_list) {
        Ok(_) => { /* 😎 */ }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn run_repl() {
    println!("Gnarly REPL v{}", env!("CARGO_PKG_VERSION"));
    println!("Type '.exit' to quit");

    let mut interpreter = Interpreter::new();

    loop {
        print!("gnarly> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                // Handle special commands
                if input == ".exit" {
                    println!("Have a gnarly day!");
                    break;
                }

                // Skip empty lines
                if input.is_empty() {
                    continue;
                }

                // Lex the input
                let lexer_result = match Lexer::scan(input) {
                    Ok(lexer) => lexer,
                    Err(err) => {
                        eprintln!("Lexer error: {}", err);
                        continue;
                    }
                };

                // Run the tokens through interpreter
                let operand_stack_size = interpreter.current_scope_readonly().get_operand_stack().len() as isize;
                match interpreter.run(lexer_result.token_list) {
                    Ok(_) => {
                        // Print most recent operand, if any pushed to the stack
                        let operand_stack = interpreter.current_scope_readonly().get_operand_stack();
                        let operand_stack_delta = operand_stack.len() as isize - operand_stack_size;
                        if operand_stack_delta > 0 {
                            match operand_stack.last().unwrap() {
                                Operand::Number(value) => println!("{}", value),
                                Operand::String(value) => println!("\"{}\"", value),
                                Operand::Variable(name) => println!("${}", name),
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}
