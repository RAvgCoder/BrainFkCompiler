use std::fs::File;
use std::io::Read;

use colored::Colorize;

use crate::asm_generator::AsmContext;
use crate::parser::Parser;

mod asm_generator;
mod grammar;
mod lexer;
mod parser;

/// Represents a Brainfuck program with its content and optimization flag.
struct Program {
    content: String,
    should_optimise: bool,
}

/// Main entry point of the program. Reads a Brainfuck program from a file, parses it,
/// optimizes it, and executes it using an interpreter.
fn main() {
    // Read the program from file
    let Program {
        content,
        should_optimise,
    } = read_file();

    // Init a parser that takes the program and converts it to a token stream
    let mut parser = Parser::new(content, should_optimise);

    // Generates an abstract syntax tree for the program
    parser.generate_syntax_tree();

    // Create the asm and generate the x86 representation of the Brain FK program
    let mut asm_context = create_asm_context(&mut parser);

    asm_context.generate_asm();

    println!("\n");
}

fn create_asm_context(parser: &mut Parser) -> AsmContext {
    match parser.get_ast() {
        Some(syntax_tree) => {
            return AsmContext::new(syntax_tree, "resources/program.asm");
        }
        None => {
            eprintln!("{}", "Tree has not been generated yet".red());
            std::process::exit(255);
        }
    }
}

/// Reads a Brainfuck program from a file and returns a `Program` struct containing
/// the program content and optimization flag.
fn read_file() -> Program {
    // let args: Vec<String> = env::args().collect();
    //
    // if args.len() < 2 {
    //     eprintln!("Usage: {} <filename> <flag[-0]>", args[0]);
    //     std::process::exit(404);
    // }
    //
    // let file_path = &args[1];
    // let optimisation = args.len() > 2;

    let file_path = String::from("resources/program.bfk");
    let optimisation = true;

    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", file_path, why),
    };

    let mut prog = String::new();
    match file.read_to_string(&mut prog) {
        Ok(_) => {}
        Err(why) => panic!("couldn't read {}: {}", file_path, why),
    }

    Program {
        content: prog,
        should_optimise: optimisation,
    }
}
