use register_machine::{assembler::assemble, vm::VM};

mod repl;

fn main() {
    // If no arguments are passed, start the REPL
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        repl::start_repl();
    } else {
        // If arguments are passed, read the file and run the program
        let filename = &args[1];
        let input = std::fs::read_to_string(filename).unwrap();

        let mut vm = assemble(input, VM::new()).expect("Failed to assemble program");

        // Print syscall
        // Print the value a register
        // Register to print stored at %80
        vm.add_syscall(0, |vm| {
            let register = vm.registers[80];
            let value = vm.registers[register as usize];

            println!("{}", value);

            true
        });

        vm.run();
    }
}
