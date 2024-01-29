
.data
    array_len: .long 26        # Define a variable to store the length of the array
    array: .space 26           # Define an array of characters with length 26
    
.text
.globl _start
_start:
    movl    array_len, %ecx             # Load the length of the array into ecx
    xor     %edx, %edx              # Initialize a counter register to 0

fill_array:
    movb    $0, array(%edx)     # Store 0 to the array at index CELL_PTR_REG
    inc     %edx              # Increment the counter
    cmp     %ecx, %edx          # Compare the counter to the length of the array
    jl      fill_array       # Jump to fill_array if counter is less than the length
    xor     %edx, %edx          # Reset the cell_ptr to 0

    jmp     LOOP_L0_C1
    LOOP_L0_C1_RET:
    
EXIT:                               # Exiting the program
    movl    $1, %eax               # sys_exit syscall number
    xorl    %ebx, %ebx             # exit status 0
    int     $0x80                   # syscall
    
LOOP_L0_C1:
	# Token::MoveForward | Count:1
    add      $1, %edx 

	# Token::Add | Count:67
    
    lea     array(%edx), %ebx
    mov     (%ebx), %eax
    add      $67, %eax
    mov    %eax, (%ebx)
    
	# Token::StdOut | Count:1
    
    # Print Character at index
    
    # Save Reg for cell_memory_(len & ptr)
    mov     %ecx, %esi
    mov     %edx, %edi
    
    lea     array(%edx), %ebx          # Read Get the address of the Character to Print
    mov     %ebx, %ecx                  # Moves the address for printing
    movl    $4, %eax                 # sys_write syscall number
    movl    $1, %ebx                 # file descriptor for stdout
    movl    $1, %edx                 # length of the character
    int     $0x80                    # syscall
    
    # Restore Reg for cell_(len & ptr)
    mov     %esi, %ecx
    mov     %edi, %edx
    
    
    
	# Token::MoveBack | Count:1
    sub      $1, %edx 

    # Check if current index is zero
    
    lea     array(%edx), %ebx
    mov     (%ebx), %eax
    cmp     $0, %eax
    jne     LOOP_L0_C1
    # End loop if curr index is zero
    jmp LOOP_L0_C1_RET
    