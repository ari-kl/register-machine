use register_machine::{opcode::OpCode, vm};

fn main() {
    let mut vm = vm::VM::new();

    vm.write_opcode(OpCode::LOAD); // 0
    vm.write_u8(0); // 1
    vm.write_u16(10); // 2, 3

    vm.write_opcode(OpCode::LOAD); // 4
    vm.write_u8(1); // 5
    vm.write_u16(100); // 6, 7

    vm.write_opcode(OpCode::LOAD); // 8
    vm.write_u8(3); // 9
    vm.write_u16(12); // 10, 11

    vm.write_opcode(OpCode::ADD); // 12
    vm.write_u8(2); // 13
    vm.write_u8(2); // 14
    vm.write_u8(0); // 15

    vm.write_opcode(OpCode::EQ); // 16
    vm.write_u8(1); // 17
    vm.write_u8(2); // 18

    vm.write_opcode(OpCode::JNEQ); // 19
    vm.write_u8(3); // 20

    vm.run();
    println!("{:?}", vm.registers);
}
