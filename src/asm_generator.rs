use std::collections::LinkedList;
use std::fs::File;
use std::io;
use std::io::Write;

use colored::Colorize;

use crate::asm_generator::asm_instructions::*;
use crate::grammar::{Expression, Token};

mod array_list_asm;
mod asm_instructions;

/// Represents the context for generating assembly code from Brainfuck expressions.
pub struct AsmContext<'a> {
    // Tree containing instructions to be executed
    syntax_tree: &'a [Expression],
    // File to write into
    asm_file: File,
    // The main function of the program
    main_func: LinkedList<String>,
    // All Loops used in the program
    loop_func: LinkedList<String>,
    // Unique id's to be given to the loops
    loop_uuid: usize,
    // Used to check if the program requires dealing the console
    used_stdin: bool,
    used_stdout: bool,
}

impl<'a> AsmContext<'a> {
    /// Creates a new `AsmContext` with the provided syntax tree and file path.
    pub fn new(syntax_tree: &'a [Expression], file_path: &str) -> Self {
        let asm_file = match File::create(file_path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("{:?}", error);
                std::process::exit(404);
            }
        };

        AsmContext {
            syntax_tree,
            asm_file,
            main_func: LinkedList::new(),
            loop_func: LinkedList::new(),
            loop_uuid: 1,
            used_stdin: false,
            used_stdout: false,
        }
    }

    /// Generates the assembly code from the syntax tree and writes it to the file.
    pub fn generate_asm(&mut self) {
        self.generate_asm_(self.syntax_tree, false, 0, 0);

        // Adds init functions and other helper functions depending on what the program
        // requires as recorded in its context
        self.inject_helper_functions();

        match self.write_to_file() {
            Ok(_) => {
                println!("{}", "The ASM code was successfully generated".green());
            }
            Err(error) => {
                eprintln!("{}", "Could not generate the ASM file".red());
                eprintln!("{}", error)
            }
        };
    }

    /// Recursively generates assembly code for the given expressions.
    fn generate_asm_(&mut self,
                     expression: &[Expression],
                     in_expanded_loop: bool,
                     parent_loop_id: usize,
                     loop_depth: usize,
    ) {
        // Instructions for current level usefull for loops
        let mut instructions: Vec<String> = vec![];

        for expr in expression.iter() {
            match expr {
                Expression::Loop(_loop) => {
                    let loop_id = self.assign_loop_uuid();
                    // Push the loop call and return position
                    instructions.push(asm_loop_call(loop_depth, loop_id));

                    self.generate_asm_(_loop, true, loop_id, loop_depth + 1)
                }
                Expression::Operator(_op) => {
                    instructions.push(format!(
                        "\n\t# Token::{:?} | Count:{}",
                        _op.type_name, _op.count
                    ));
                    instructions.push(match _op.type_name {
                        Token::MoveBack => asm_cell_ptr_decrement(_op.count),
                        Token::MoveForward => asm_cell_ptr_increment(_op.count),
                        Token::Add => asm_cell_increment(_op.count),
                        Token::Sub => asm_cell_decrement(_op.count),
                        Token::StdOut => {
                            self.used_stdout = true;
                            asm_print_cell()
                        }
                        Token::StdIn => {
                            self.used_stdin = true;
                            asm_read_to_cell()
                        }
                        _ => {
                            eprintln!(
                                "Unexpected Token: {:?} when generating assembly ",
                                _op.type_name
                            );
                            std::process::exit(123);
                        }
                    });
                }
            }
        }

        // Prepare for loop return if applicable
        let list = if in_expanded_loop {
            // --------------- [ If only in loop ] ------------ \\

            // Give the curr loop a name
            instructions.insert(0, asm_loop_name(loop_depth - 1, parent_loop_id));
            // End the loop
            instructions.push(asm_loop_end(loop_depth - 1, parent_loop_id));
            &mut self.loop_func
        } else {
            &mut self.main_func
        };

        instructions
            .iter()
            .rev()
            .for_each(|elem| list.push_front(elem.to_string()))
    }

    /// Writes the generated assembly code to the file.
    fn write_to_file(&self) -> Result<(), io::Error> {
        let mut file = &self.asm_file;

        // Write the main instructions to file
        for instruction in &self.main_func {
            file.write_all(instruction.as_bytes())?;
        }

        // Write the loop func instructions to file
        for functions in &self.loop_func {
            file.write_all(functions.as_bytes())?;
        }

        // File read successfully
        Ok(())
    }


    /// Adds init functions and other helper functions depending on what the program
    //  requires as recorded in its context
    /// Inits push must be backwards {push_front()}
    fn inject_helper_functions(&mut self) {
        // ------------- [ Top Part ] ------------- \\

        // Main entry
        self.main_func.push_front(asm_main_init());

        // Input for user
        if self.used_stdin {
            self.main_func.push_front(asm_stdin_init());
        }

        // Data section
        self.main_func.push_front(asm_data_init());

        // ---------- [ Top Part END ] ---------- \\

        // Print out the array used by the program
        // self.main_func.push_back(asm_debug_memory());

        // Exit the program
        self.main_func.push_back(asm_exit());
    }

    /// Assigns a unique loop UUID.
    fn assign_loop_uuid(&mut self) -> usize {
        self.loop_uuid += 1;
        self.loop_uuid - 1
    }
}
