
.data
    array_len: .long 256             # Define a variable to store the length of the array
    array: .space 256                # Define an array of characters with length 256
    
.text
.globl _start
_start:
    movl    array_len, %ecx         # Load the length of the array into ecx
    xor     %edx, %edx              # Initialize a counter register to 0

fill_array:
    movb    $0, array(%edx)         # Store 0 to the array at index CELL_PTR_REG
    inc     %edx                    # Increment the counter
    cmp     %ecx, %edx              # Compare the counter to the length of the array
    jl      fill_array              # Jump to fill_array if counter is less than the length
    xor     %edx, %edx              # Reset the cell_ptr to 0

# Token::Add | Count:2
    
    lea     array(%edx), %ebx
    mov     (%ebx), %eax
    add      $2, %eax
    mov    %eax, (%ebx)
    
# Token::MoveForward | Count:1
    add      $1, %edx 

# Token::Add | Count:5
    
    lea     array(%edx), %ebx
    mov     (%ebx), %eax
    add      $5, %eax
    mov    %eax, (%ebx)
    
    jmp     LOOP_L0_C1
    LOOP_L0_C1_RET:
    
# Token::Add | Count:8
    
    lea     array(%edx), %ebx
    mov     (%ebx), %eax
    add      $8, %eax
    mov    %eax, (%ebx)
    
    jmp     LOOP_L0_C2
    LOOP_L0_C2_RET:
    
# Token::MoveBack | Count:1
    sub      $1, %edx 

# Token::StdOut | Count:1
    
    # Print Character at index
    # Save Reg for cell_memory_(len & ptr)
    mov     %ecx, %esi
    mov     %edx, %edi

    lea     array(%edx), %ebx        # Read Get the address of the Character to Print
    mov     %ebx, %ecx               # Moves the address for printing
    movl    $4, %eax                 # sys_write syscall number
    movl    $1, %ebx                 # file descriptor for stdout
    movl    $1, %edx                 # length of the character
    int     $0x80                    # syscall

    # Restore Reg for cell_(len & ptr)
    mov     %esi, %ecx
    mov     %edi, %edx
    
    
EXIT:                               # Exiting the program
    movl    $1, %eax                # sys_exit syscall number
    xorl    %ebx, %ebx              # exit status 0
    int     $0x80                   # syscall
    
LOOP_L0_C2:

# Token::MoveBack | Count:1
    sub      $1, %edx 

# Token::Add | Count:6
    
    lea     array(%edx), %ebx
    mov     (%ebx), %eax
    add      $6, %eax
    mov    %eax, (%ebx)
    
# Token::MoveForward | Count:1
    add      $1, %edx 

# Token::Sub | Count:1
    
    lea     array(%edx), %ebx
    mov     (%ebx), %eax
    sub      $1, %eax
    mov    %eax, (%ebx)
    
    # Check if current index is zero
    
    lea     array(%edx), %ebx
    mov     (%ebx), %eax
    cmp     $0, %eax
    jne     LOOP_L0_C2
    # End loop if curr index is zero
    jmp LOOP_L0_C2_RET
    
LOOP_L0_C1:

# Token::MoveBack | Count:1
    sub      $1, %edx 

# Token::Add | Count:1
    
    lea     array(%edx), %ebx
    mov     (%ebx), %eax
    add      $1, %eax
    mov    %eax, (%ebx)
    
# Token::MoveForward | Count:1
    add      $1, %edx 

# Token::Sub | Count:1
    
    lea     array(%edx), %ebx
    mov     (%ebx), %eax
    sub      $1, %eax
    mov    %eax, (%ebx)
    
    # Check if current index is zero
    
    lea     array(%edx), %ebx
    mov     (%ebx), %eax
    cmp     $0, %eax
    jne     LOOP_L0_C1
    # End loop if curr index is zero
    jmp LOOP_L0_C1_RET
    