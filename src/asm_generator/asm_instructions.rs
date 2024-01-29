// ------------------- [ REGISTERS ] ------------------- \\
// NOTE IF CHANGES TO THE REGISTERS 'e[a-d]x' ARE MADE AN EVALUATION OF ALL SYS CALLS ARE TO BE CHECKED

static REG_TEMP: &str = "%eax";
static REG_TEMP_NOT_PUBLIC: &str = "%ebx";
static REG_CELL_LEN: &str = "%ecx";
static REG_CELL_PTR: &str = "%edx";
static REG_TEMP_SAVE_LEN: &str = "%esi";
static REG_TEMP_SAVE_PTR: &str = "%edi";

// ------------------- [ REGISTERS END ] ------------------- \\

static CELL_MEMORY: &str = "array";
static CELL_MEMORY_LEN: &str = "array_len";
static PROMPT_MEMORY: &str = "input_prompt";
static PROMPT: &str = "Enter a character: ";
static LOOP: &str = "LOOP";

enum SysCall {
    Write = 4,
    Read = 3,
}

enum FileDescriptor {
    StdOut = 1,
    StdIn = 0,
}

// ---------------------- [ REG ] ---------------------- \\

/// Resets the [REG_TEMP]
fn asm_reset_temp() -> String {
    format!("xor    {}, {}", REG_TEMP, REG_TEMP)
}

/// Save [REG_CELL_LEN] & [REG_CELL_PTR]
fn save_reg() -> String {
    format!(
        r#"
    # Save Reg for cell_memory_(len & ptr)
    mov     {}, {}
    mov     {}, {}
    "#,
        REG_CELL_LEN, REG_TEMP_SAVE_LEN,
        REG_CELL_PTR, REG_TEMP_SAVE_PTR
    )
}

/// Restore [REG_CELL_LEN] & [REG_CELL_PTR]
fn restore_reg() -> String {
    format!(
        r#"
    # Restore Reg for cell_(len & ptr)
    mov     {}, {}
    mov     {}, {}
    "#,
        REG_TEMP_SAVE_LEN, REG_CELL_LEN,
        REG_TEMP_SAVE_PTR, REG_CELL_PTR
    )
}

// ------------------ [ END REG ] ------------------ \\

// --------------------- [ INIT ] --------------------- \\
// --------------- [ MUST USE ] --------------- \\

/// Represents the start of the data section of the assembly file
#[must_use]
pub fn asm_data_init() -> String {
    format!(
        r#"
.data
    {}: .long 26        # Define a variable to store the length of the array
    {}: .space 26           # Define an array of characters with length 26
    "#,
        CELL_MEMORY_LEN, CELL_MEMORY
    )
}

/// Represents the entry point of the program
#[must_use]
pub fn asm_main_init() -> String {
    format!(
        r#"
.text
.globl _start
_start:
    movl    {}, {}             # Load the length of the array into ecx
    xor     {}, {}              # Initialize a counter register to 0

fill_array:
    movb    $0, {}({})     # Store 0 to the array at index CELL_PTR_REG
    inc     {}              # Increment the counter
    cmp     {}, {}          # Compare the counter to the length of the array
    jl      fill_array       # Jump to fill_array if counter is less than the length
    xor     {}, {}          # Reset the cell_ptr to 0
"#,
        CELL_MEMORY_LEN,
        REG_CELL_LEN,
        REG_CELL_PTR,
        REG_CELL_PTR,
        CELL_MEMORY,
        REG_CELL_PTR,
        REG_CELL_PTR,
        REG_CELL_LEN,
        REG_CELL_PTR,
        REG_CELL_PTR,
        REG_CELL_PTR
    )
}

/// Marks the exit portion of the program
#[must_use]
pub fn asm_exit() -> String {
    r#"
EXIT:                               # Exiting the program
    movl    $1, %eax               # sys_exit syscall number
    xorl    %ebx, %ebx             # exit status 0
    int     $0x80                   # syscall
    "#
        .to_string()
}

// --------------- [ MUST USE END ] --------------- \\

/// Creates a method for prompting the user when the program needs it
pub fn asm_stdin_init() -> String {
    format!(
        r#"{}:   .asciz "{}"        # Prompt for user
    "#,
        PROMPT_MEMORY, PROMPT,
    )
}

// --------------------- [ INIT END ] --------------------- \\


// ---------------------- [ FUNCTION PARTS ] ---------------------- \\

// ---------------- [ I/O ] ---------------- \\

/// Prints the byte of the current cell
pub fn asm_print_cell() -> String {
    format!(
        r#"
    {}
    "#,
        asm_sys_call(SysCall::Write, FileDescriptor::StdOut, false))
}

/// Reads a character from the console and writes it to that particular index
pub fn asm_read_to_cell() -> String {
    format!(
        r#"
    {}
    {}
    "#,
        asm_sys_call(SysCall::Write, FileDescriptor::StdOut, true),
        asm_sys_call(SysCall::Read, FileDescriptor::StdIn, false))
}

// ------------ [ SYS_CALLS ] ------------ \\

/// Template for a syscall
fn asm_sys_call(
    sys_call: SysCall,
    file_descriptor: FileDescriptor,
    is_prompt: bool,
) -> String {
    /*
       Get the char address to print
       Save reg for cell_len and cell_ptr
       print
       Restore reg for cell_len and cell_ptr
    */
    format!(
        r#"
    # {}
    {}
    {}
    movl    ${}, %eax                 # sys_write syscall number
    movl    ${}, %ebx                 # file descriptor for stdout
    movl    ${}, %edx                 # length of the character
    int     $0x80                    # syscall
    {}
    "#,
        if is_prompt {
            "Prompt user for input"
        } else {
            "Print Character at index"
        },
        save_reg(),
        if is_prompt {
            asm_init_ecx_for_sys_call_prompt()
        } else {
            asm_init_ecx_for_sys_call_index()
        },
        sys_call as usize,
        file_descriptor as usize,
        if is_prompt { PROMPT.len() } else { 1 },
        restore_reg()
    )
}

/// Gives the address to ecx for the prompt when doing a syscall
fn asm_init_ecx_for_sys_call_prompt() -> String {
    format!(
        "\
    mov     ${}, {}                  # Moves the address for prompt",
        PROMPT_MEMORY, REG_CELL_LEN
    )
}

/// Gives the address of the character in the array to be printed
fn asm_init_ecx_for_sys_call_index() -> String {
    format!(
        "\
    {}          # Read Get the address of the Character to Print
    mov     {}, {}                  # Moves the address for printing",
        asm_get_index_mem_offset(),
        REG_TEMP_NOT_PUBLIC, REG_CELL_LEN
    )
}
// ------------ [ SYS_CALLS END ] ------------ \\

// -------------- [ I/O END ] -------------- \\



// ----------------- [ LOOPS ] ----------------- \\

/// Gives the template name of the loops
pub fn asm_loop_label(level: usize, count: usize) -> String {
    format!("{}_L{}_C{}", LOOP, level, count)
}

/// Name of the return loop function
/// Eg = 'LOOP2_C2_RET'
fn asm_loop_ret(level: usize, count: usize) -> String {
    format!("{}_RET", asm_loop_label(level, count))
}

/// Gives the name of the loop function
/// Eg = 'LOOP2_C2:'
pub fn asm_loop_name(level: usize, count: usize) -> String {
    format!(r#"
{}:"#, asm_loop_label(level, count))
}

/// Calls a loop and gives a label to jump back to when it's done
/// jmp LOOP1
/// LOOP1_RET:
pub fn asm_loop_call(level: usize, count: usize) -> String {
    format!(
        r#"
    jmp     {}
    {}:
    "#,
        asm_loop_label(level, count),
        asm_loop_ret(level, count)
    )
}

/// The end of the loop with the necessary checks for if it should go another
/// round or return
pub fn asm_loop_end(level: usize, count: usize) -> String {
    format!(
        r#"
    # Check if current index is zero
    {}
    cmp     $0, {}
    jne     {}
    # End loop if curr index is zero
    jmp {}
    "#,
        asm_extract_at_index(),
        REG_TEMP,
        asm_loop_label(level, count),
        asm_loop_ret(level, count)
    )
}

// ----------------- [ LOOPS END ] ----------------- \\




// ----------------- [ CELL_PTR & MEMORY MODIFICATIONS ] ----------------- \\

/// Increments the cell ptr
pub fn asm_cell_ptr_increment(offset: usize) -> String {
    asm_offset_cell_ptr("add", offset)
}

/// Decrements the cell ptr
pub fn asm_cell_ptr_decrement(offset: usize) -> String {
    asm_offset_cell_ptr("sub", offset)
}

/// Increments the value in the cell
pub fn asm_cell_increment(offset: usize) -> String {
    asm_modify_cell("add", offset)
}

/// Decrements the value in the cell
pub fn asm_cell_decrement(offset: usize) -> String {
    asm_modify_cell("sub", offset)
}

/// Adds an offset to the cell ptr
fn asm_offset_cell_ptr(instr: &str, amount: usize) -> String {
    // add $1, %eax
    format!(
        "
    {}      ${}, {} \n",
        instr, amount, REG_CELL_PTR
    )
}

// TODO: Fix incrementation and decrementation so as to be in set of { Z mod 255 }
/// Incr/Decr the value in the cell
fn asm_modify_cell(instr: &str, amount: usize) -> String {
    /*
        access val at index
        add to that val
        put back to index
    */
    format!(
        "
    {}
    {}      ${}, {}
    {}
    ",
        asm_extract_at_index(),
        instr, amount, REG_TEMP,
        asm_store_to_index()
    )
}

/// Extracts the num at index [REG_CELL_PTR] into [REG_TEMP]
fn asm_extract_at_index() -> String {
    format!(
        r#"
    {}
    mov     ({}), {}"#,
        asm_get_index_mem_offset(),
        REG_TEMP_NOT_PUBLIC, REG_TEMP
    )
}

/// Gets the memory address of the value in
/// [CELL_MEMORY] at index [REG_CELL_PTR] to [REG_TEMP_NOT_PUBLIC]
fn asm_get_index_mem_offset() -> String {
    format!(
        "lea     {}({}), {}",
        CELL_MEMORY, REG_CELL_PTR, REG_TEMP_NOT_PUBLIC,
    )
}

/// Stores the value in [REG_TEMP] to memory located in [REG_TEMP_NOT_PUBLIC]
fn asm_store_to_index() -> String {
    format!("mov    {}, ({})", REG_TEMP, REG_TEMP_NOT_PUBLIC)
}

// ------------ [ CELL_PTR & MEMORY MODIFICATIONS END ] ------------ \\


// ---------------------- [ FUNCTIONS END  ] ---------------------- \\




// ---------------------- [ Extra ] ---------------------- \\

/// Prints out all the characters in the [CELL_MEMORY]
pub fn asm_debug_memory() -> String {
    format!(
        r#"
DEBUG_PRINT_CELL_MEMORY:        # Printing the array
    movl    ${}, %eax              # sys_write syscall number
    movl    ${}, %ebx              # file descriptor for stdout
    movl    ${}, %ecx              # pointer to the array
    movl    {}, %edx               # length of the array
    int     $0x80                   # syscall
    "#,
        SysCall::Write as usize,
        FileDescriptor::StdOut as usize,
        CELL_MEMORY,
        CELL_MEMORY_LEN
    )
}

// ---------------------- [ END Extra ] ---------------------- \\
