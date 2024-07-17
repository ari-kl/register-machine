# register-machine

A simple, embeddable register-based virtual machine written in Rust.
This project was made to learn more about register VMs and how they work, potentially to use in a future programming language project.

## Usage
This project is intended to be embedded in other projects, but it also comes with a simple repl and file execution for testing purposes.

After cloning the repository, you can run the repl with:
```sh
cargo run
```

You can also run a file with:
```sh
cargo run -- examples/<your_file>.rm
```

Note that the default repl and file execution have a registered syscall with an id `0` that prints the value of the register pointed to by register `%80`. For example, the following code will print `321`:
```asm
load %1 #321 ! This is the register we want to print
load %80 #1 ! We are setting %80 to the id of the register we want to print, in this case %1
sys #0 ! This will call the syscall with id 0, which will print the value of the register pointed to by %80
```

## Features
- 256 registers
- 19 instructions
- Arithmetic operations
- Comparison operations
- Conditional and unconditional jumps
- User-defined syscalls
- Repl & file execution for the base VM

For examples on how to use the default assembly, check the `examples` directory.

## Instructions

Note: values wrapped in `<>` are registers, and values wrapped in `[]` are integer values.
When writing bytecode directly to the VM, registers should be written using `write_u8` and integer values using `write_u16`.

| Instruction | Opcode | Description | Usage |
|-------------|--------|-------------|-------|
| STOP        | 0      | Stops the current VM execution | `stop` |
| LOAD        | 1      | Loads an integer value into a register | `load <register> [value]` |
| MOV         | 2      | Moves the value of a register into another register | `mov <dst> <src>` |
| ADD         | 3      | Adds two registers and stores the result in a register | `add <dst> <src1> <src2>` |
| SUB         | 4      | Subtracts two registers and stores the result in a register | `sub <dst> <src1> <src2>` |
| MUL         | 5      | Multiplies two registers and stores the result in a register | `mul <dst> <src1> <src2>` |
| DIV         | 6      | Divides two registers and stores the result in a register | `div <dst> <src1> <src2>` |
| JMP         | 7      | Jumps to a specific instruction stored in a register by modifying the program counter | `jmp <address>`
| JFW         | 8      | Jumps forward by incrementing the program counter | `jfw <offset>` |
| JBW         | 9      | Jumps backward by decrementing the program counter | `jbw <offset>` |
| EQ          | 10     | Compares the equality of 2 registers and sets the comparison flag | `eq <reg1> <reg2>` |
| NEQ         | 11     | Compares the inequality of 2 registers and sets the comparison flag | `neq <reg1> <reg2>` |
| GT          | 12     | Compares if the first register is greater than the second register and sets the comparison flag | `gt <reg1> <reg2>` |
| LT          | 13     | Compares if the first register is less than the second register and sets the comparison flag | `lt <reg1> <reg2>` |
| GTE         | 14     | Compares if the first register is greater than or equal to the second register and sets the comparison flag | `gte <reg1> <reg2>` |
| LTE         | 15     | Compares if the first register is less than or equal to the second register and sets the comparison flag | `lte <reg1> <reg2>` |
| JEQ         | 16     | Jumps to a specific instruction stored in a register if the comparison flag is set to true | `jeq <address>` |
| JNE         | 17     | Jumps to a specific instruction stored in a register if the comparison flag is set to false | `jne <address>` |
| SYS     | 18     | Calls a user-defined syscall | `sys [syscall_id]` |

## Syscalls

The VM supports user-defined syscalls, which can be used to interact with the host environment. You can define a syscall as follows:

```rust
use register_machine::vm::VM;

fn main() {
    let mut vm = VM::new();
    vm.register_syscall(0, |vm| {
        println!("Hello from syscall 0!");

        true
    });

    // Insert code generation here

    vm.run();
}
```

This code will register a syscall with the id `0` that will print `Hello from syscall 0!` to the console. The closure passed to `register_syscall` will be called when the `SYS` instruction is executed with the id `0`.
The closure should return a boolean value determining the success of the syscall. If the syscall fails, the VM will stop execution.

## Future Ideas:
- [ ] Bytecode writing documentation
- [ ] Memory Access
- [ ] Improved error handling
- [ ] Improve assembler
- [ ] more?
