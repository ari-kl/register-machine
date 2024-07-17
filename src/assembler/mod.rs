use crate::vm::VM;

mod lexer;
mod parser;

pub fn assemble(input: String, vm: VM) -> VM {
    let mut lexer = lexer::Lexer::new(input);
    lexer.scan_tokens();

    let mut parser = parser::Parser::new(lexer.tokens, vm);

    parser.parse()
}

#[cfg(test)]
mod assembler_tests {
    use super::*;

    #[test]
    fn test_assemble() {
        let input = String::from("load %0 #123\nload %1 #456\nadd %2 %0 %1\n");
        let mut vm = assemble(input, VM::new());

        vm.run();

        assert_eq!(vm.registers[0], 123);
        assert_eq!(vm.registers[1], 456);
        assert_eq!(vm.registers[2], 579);
    }

    #[test]
    fn test_append_code() {
        let input = String::from("load %0 #123\nload %1 #456\nadd %2 %0 %1\n");
        let mut vm = assemble(input, VM::new());

        vm.run();

        let input2 = String::from("load %3 #579\neq %3 %2\n");
        vm = assemble(input2, vm);

        vm.run();

        assert_eq!(vm.registers[0], 123);
        assert_eq!(vm.registers[1], 456);
        assert_eq!(vm.registers[3], 579);
        assert_eq!(vm.comparison, true);
    }
}