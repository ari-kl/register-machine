use std::io::Write;

use register_machine::{assembler::assemble, vm::VM};

pub fn start_repl() {
    let mut vm = VM::new();

    // Print syscall
    // Print the value a register
    // Register to print stored at %80
    vm.register_syscall(0, |vm| {
        let register = vm.registers[80];
        let value = vm.registers[register as usize];

        println!("{}", value);

        true
    });

    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        if let Ok(new_vm) = assemble(input, vm.clone()) {
            vm = new_vm;
            vm.run();

            continue;
        }
    }
}
