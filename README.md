Below is a template for a README.md file for your Brainfuck to Assembly (ASM) code generator project:


# Brainfuck to Assembly (ASM) Code Generator

This project provides a tool for converting Brainfuck expressions into equivalent assembly code. Brainfuck is a minimalist programming language with a small set of commands, making it an interesting target for code generation.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Usage](#usage)
- [Installation](#installation)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Brainfuck is an esoteric programming language known for its minimalistic design and simplicity. It consists of a set of eight commands, making it a challenge to write and understand programs in it. This project aims to provide a tool for converting Brainfuck expressions into assembly code, allowing for easier execution on various architectures.

## Features

- Converts Brainfuck expressions to equivalent assembly code.
- Handles input/output operations, loop structures, memory modifications, and debugging utilities.
- Generates optimized assembly code.
- Provides options for customizing output and optimization levels.

## Usage

To use the Brainfuck to ASM code generator, follow these steps:

1. **Installation**: Clone the repository and build the project using Cargo.
2. **Usage**: Import the `AsmContext` struct and use it to generate assembly code from Brainfuck expressions.
3. **Customization**: Adjust optimization levels and output settings as needed.
4. **Output**: The generated assembly code can be written to a file for further processing or execution.

## Installation

To install the Brainfuck to ASM code generator, follow these steps:

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/brainfuck-to-asm.git
   ```

2. Build the project using Cargo:

   ```bash
   cd brainfuck-to-asm
   cargo build --release
   ```

3. Run the executable:

   ```bash
   ./target/release/brainfuck-to-asm <input_file> <output_file>
   ```

## Examples

Here's an example of how to use the Brainfuck to ASM code generator:

```rust
use brainfuck_to_asm::AsmContext;

fn main() {
    let bf_code = ">+++<[>+++<]+++++.,.";
    let should_optimise = true;

    // Init a parser that takes the program and converts it to a token stream
    let mut parser = Parser::new(bf_code, should_optimise);

    // Generates an abstract syntax tree for the program
    parser.generate_syntax_tree();

    // Create the asm to generate the x86 representation of the Brain FK program
    let mut asm_context = AsmContext::new(
        parser.get_ast().unwrap(),
        "resources/program.asm"     // Location to store the assembly file
    );

    // Generate the actual assembly file
    asm_context.generate_asm();

}
```

This will convert the Brainfuck code `>+++<[>+++<]+++++.,.` into equivalent assembly code and write it to a file.

## Contributing

Contributions to the Brainfuck to ASM code generator project are welcome! Feel free to open issues for bug fixes, feature requests, or other improvements. Pull requests are also appreciated.

## License

This project is licensed under the [MIT License](LICENSE).
