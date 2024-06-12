use crate::grammar::{Expression, Operator, Token};
use crate::lexer::Lexer;
use colored::Colorize;

/// Struct representing a parser for the custom language.
#[derive(Debug)]
pub struct Parser {
    /// Lexer instance to tokenize the program
    tokens: Vec<Token>,
    /// Index to keep track of parsing progress
    parser_index: usize,
    /// Flag indicating whether to optimize the AST
    should_optimize: bool,
    /// Holds the parsed tree
    syntax_tree: Option<Vec<Expression>>,
    /// Number of instructions
    num_of_instr: usize,
}

impl Parser {
    /// Constructs a new `Parser` instance.
    ///
    /// # Arguments
    ///
    /// * `program` - The program string to be parsed.
    /// * `optimize` - A boolean indicating whether to optimize the AST.
    ///
    /// # Returns
    ///
    /// A new instance of `Parser`.
    pub fn new(program: String, optimize: bool) -> Self {
        Parser {
            tokens: Lexer::new(program).move_tokens(),
            parser_index: 0,
            should_optimize: optimize,
            syntax_tree: None,
            num_of_instr: 0,
        }
    }

    /// Generates the abstract syntax tree (AST) by parsing the program.
    ///
    /// # Returns
    ///
    /// The AST represented as a vector of `Expression`.
    pub fn generate_syntax_tree(&mut self) {
        let mut ast = self.parse_to_ast();
        if self.should_optimize {
            Self::optimize_ast(&mut ast);
        }
        self.syntax_tree = Some(ast);
        self.num_of_instr = Parser::count_instructions(self.get_ast());
    }

    /// Gets the number of instructions in the program.
    ///
    /// # Returns
    ///
    /// The number of instructions.
    pub fn get_num_of_instr(&self) -> usize {
        self.num_of_instr
    }

    /// Gets the abstract syntax tree (AST) of the program.
    ///
    /// # Returns
    ///
    /// The AST represented as a vector of `Expression`.
    pub fn get_ast(&self) -> Option<&Vec<Expression>> {
        self.syntax_tree.as_ref()
    }

    /// Parses the tokens into an abstract syntax tree (AST).
    ///
    /// # Returns
    ///
    /// The AST represented as a vector of `Expression`.
    fn parse_to_ast(&mut self) -> Vec<Expression> {
        let mut expressions: Vec<Expression> = vec![];

        while self.parser_index < self.tokens.len() {
            let token = self.tokens[self.parser_index];
            self.parser_index += 1;

            expressions.push(match token {
                Token::LoopStart => Expression::Loop(self.parse_to_ast()),
                Token::LoopEnd => {
                    return expressions;
                }
                _ => Expression::Operator(Box::new(Operator {
                    type_name: token,
                    count: 1,
                })),
            });
        }

        expressions
    }

    /// Optimizes the abstract syntax tree (AST) by removing redundant operations.
    ///
    /// # Arguments
    ///
    /// * `ast` - A mutable reference to the AST.
    fn optimize_ast(ast: &mut Vec<Expression>) {
        let mut prev: Option<&mut Operator> = None;

        // The index of optimized out AST nodes to remove
        let mut nodes_idx: Vec<usize> = vec![];

        for (idx, expression) in ast.iter_mut().enumerate() {
            match expression {
                Expression::Loop(_loop) => {
                    // Optimize the expressions contained in the loop
                    Self::optimize_ast(_loop);
                    prev = None;
                }
                Expression::Operator(new_op) => {
                    match &mut prev {
                        Some(old_op) => {
                            if new_op.type_name != Token::StdOut && new_op.type_name != Token::StdIn
                            {
                                // Groups non - Std(in/out) tokens
                                if old_op.type_name == new_op.type_name {
                                    old_op.count += 1;
                                    nodes_idx.push(idx);
                                    continue;
                                }
                            }

                            // Replace the prev operation if the new one differs or is STD(IN/OUT)
                            prev = Some(new_op);
                        }
                        None => {
                            prev = Some(new_op);
                        }
                    }
                }
            }
        }

        // Delete all operations optimized out
        nodes_idx.iter().rev().for_each(|&idx| {
            ast.remove(idx);
        })
    }

    /// Counts the number of instructions in the AST.
    ///
    /// # Arguments
    ///
    /// * `ast_tree` - The abstract syntax tree (AST) represented as a vector of `Expression`.
    ///
    /// # Returns
    ///
    /// The number of instructions.
    fn count_instructions(ast_tree: Option<&Vec<Expression>>) -> usize {
        let mut count: usize = 0;
        match ast_tree {
            Some(tree) => {
                for node in tree {
                    match node {
                        Expression::Loop(_loop) => {
                            // + 1 is to count the loop itself
                            count += Self::count_instructions(Some(_loop)) + 1;
                        }
                        Expression::Operator(_op) => {
                            count += 1;
                        }
                    }
                }
            }
            None => {
                eprintln!("{}", "Tree has not been generated yet".red());
            }
        }
        count
    }
}
