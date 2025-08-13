use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process;

use crate::{
    interpreter::{Interpreter, Operand},
    lexer::{Lexer, Token},
};

mod interpreter;
mod lexer;

#[derive(Parser)]
#[command(name = "Gnarly")]
#[command(about = "The Gnarly language interpreter")]
#[command(version = "0.1.0")]
struct Cli {
    /// Input file entrypoint to read and process.
    #[arg(help = "Path to the program's main entrypoint")]
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    // Read the file contents
    let file_contents = match fs::read_to_string(&cli.file) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", cli.file.display(), err);
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

    // @DEBUG Dump token
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
    let mut interpreter = Interpreter::new(lexer_result.token_list);
    interpreter.run();

    // Print results @DEBUG
    // if interpreter.operand_stack.len() == 1 {
    //     if let Some(Operand::Number(result)) = interpreter.operand_stack.last() {
    //         println!("Result: {}", result);
    //     }
    // } else if interpreter.operand_stack.len() > 1 {
    //     println!(
    //         "Warning: {} operands left on stack",
    //         interpreter.operand_stack.len()
    //     );
    // } else {
    //     println!("No result");
    // }
}
