use crate::opcode::OpCode;

pub struct VM {
    pub registers: [i64; 256],
    pub pc: usize,
    pub code: Vec<u8>,
    pub equality: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 256],
            pc: 0,
            code: vec![],
            equality: false,
        }
    }

    pub fn run(&mut self) {
        while self.pc < self.code.len() {
            if !self.execute_instruction() {
                break;
            }
        }
    }

    fn execute_instruction(&mut self) -> bool {
        let opcode = OpCode::from(self.read_u8());

        match opcode {
            OpCode::STOP => false,
            OpCode::LOAD => {
                let register = self.read_u8() as usize;
                let value = self.read_u16() as i64;
                self.registers[register] = value;
                true
            }
            OpCode::ADD => {
                let destination = self.read_u8() as usize;
                let source1 = self.read_u8() as usize;
                let source2 = self.read_u8() as usize;
                self.registers[destination] = self.registers[source1] + self.registers[source2];
                true
            }
            OpCode::SUB => {
                let destination = self.read_u8() as usize;
                let source1 = self.read_u8() as usize;
                let source2 = self.read_u8() as usize;
                self.registers[destination] = self.registers[source1] - self.registers[source2];
                true
            }
            OpCode::MUL => {
                let destination = self.read_u8() as usize;
                let source1 = self.read_u8() as usize;
                let source2 = self.read_u8() as usize;
                self.registers[destination] = self.registers[source1] * self.registers[source2];
                true
            }
            OpCode::DIV => {
                let destination = self.read_u8() as usize;
                let source1 = self.read_u8() as usize;
                let source2 = self.read_u8() as usize;
                self.registers[destination] = self.registers[source1] / self.registers[source2];
                true
            }
            OpCode::JMP => {
                let address = self.registers[self.read_u8() as usize] as usize;
                self.pc = address;
                true
            }
            OpCode::JMPF => {
                let offset = self.registers[self.read_u8() as usize] as usize;
                self.pc += offset;
                true
            }
            OpCode::JMPB => {
                let offset = self.registers[self.read_u8() as usize] as usize;
                self.pc -= offset;
                true
            }
            OpCode::EQ => {
                let register1 = self.read_u8() as usize;
                let register2 = self.read_u8() as usize;
                self.equality = self.registers[register1] == self.registers[register2];
                true
            }
            OpCode::NEQ => {
                let register1 = self.read_u8() as usize;
                let register2 = self.read_u8() as usize;
                self.equality = self.registers[register1] != self.registers[register2];
                true
            }
            OpCode::GT => {
                let register1 = self.read_u8() as usize;
                let register2 = self.read_u8() as usize;
                self.equality = self.registers[register1] > self.registers[register2];
                true
            }
            OpCode::LT => {
                let register1 = self.read_u8() as usize;
                let register2 = self.read_u8() as usize;
                self.equality = self.registers[register1] < self.registers[register2];
                true
            }
            OpCode::GTEQ => {
                let register1 = self.read_u8() as usize;
                let register2 = self.read_u8() as usize;
                self.equality = self.registers[register1] >= self.registers[register2];
                true
            }
            OpCode::LTEQ => {
                let register1 = self.read_u8() as usize;
                let register2 = self.read_u8() as usize;
                self.equality = self.registers[register1] <= self.registers[register2];
                true
            }
            OpCode::JEQ => {
                let address = self.registers[self.read_u8() as usize] as usize;
                if self.equality {
                    self.pc = address;
                }
                true
            }
            OpCode::JNEQ => {
                let address = self.registers[self.read_u8() as usize] as usize;
                if !self.equality {
                    self.pc = address;
                }
                true
            }
            OpCode::SYS => {
                // TODO: Implement system calls
                true
            }
            OpCode::UKWN => {
                println!("Unknown OpCode: {}", self.code[self.pc]);
                false
            }
        }
    }

    // Used for reading OpCodes and register numbers
    fn read_u8(&mut self) -> u8 {
        let result = self.code[self.pc];
        self.pc += 1;
        result
    }

    // Used for reading numeric values (should only be used in the LOAD instruction)
    fn read_u16(&mut self) -> u16 {
        let result = ((self.code[self.pc] as u16) << 8) | self.code[self.pc + 1] as u16;
        self.pc += 2;
        result
    }

    pub fn write_opcode(&mut self, opcode: OpCode) {
        self.code.push(opcode as u8);
    }

    pub fn write_u8(&mut self, value: u8) {
        self.code.push(value);
    }

    pub fn write_u16(&mut self, value: u16) {
        self.code.push((value >> 8) as u8);
        self.code.push(value as u8);
    }
}

#[cfg(test)]
mod vm_tests {
    use super::*;

    #[test]
    fn test_stop() {
        let mut vm = VM::new();
        vm.write_opcode(OpCode::STOP);
        vm.run();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_load() {
        let mut vm = VM::new();
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(76);
        vm.run();
        assert_eq!(vm.registers[0], 76);
    }

    #[test]
    fn test_add() {
        let mut vm = VM::new();
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(42);
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(1);
        vm.write_u16(23);
        vm.write_opcode(OpCode::ADD);
        vm.write_u8(2);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.registers[2], 65);
    }

    #[test]
    fn test_sub() {
        let mut vm = VM::new();
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(42);
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(1);
        vm.write_u16(18);
        vm.write_opcode(OpCode::SUB);
        vm.write_u8(2);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.registers[2], 24);
    }

    #[test]
    fn test_mul() {
        let mut vm = VM::new();
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(6);
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(1);
        vm.write_u16(8);
        vm.write_opcode(OpCode::MUL);
        vm.write_u8(2);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.registers[2], 48);
    }

    #[test]
    fn test_div() {
        let mut vm = VM::new();
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(48);
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(1);
        vm.write_u16(6);
        vm.write_opcode(OpCode::DIV);
        vm.write_u8(2);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.registers[2], 8);
    }

    #[test]
    fn test_jmp() {
        let mut vm = VM::new();
        vm.write_opcode(OpCode::LOAD); // 0
        vm.write_u8(0); // 1
        vm.write_u16(8); // 2, 3
        vm.write_opcode(OpCode::JMP); // 4
        vm.write_u16(0); // 5, 6
        vm.write_opcode(OpCode::STOP); // 7
        vm.write_opcode(OpCode::LOAD); // 8
        vm.write_u8(1); // 9
        vm.write_u16(111); // 10, 11
        vm.run();
        assert_eq!(vm.registers[1], 111); // JMP should skip the STOP instruction, allowing the LOAD to execute
    }

    #[test]
    fn test_jmpf() {
        let mut vm = VM::new();
        vm.write_opcode(OpCode::LOAD); // 0
        vm.write_u8(0); // 1
        vm.write_u16(2); // 2, 3
        vm.write_opcode(OpCode::JMPF); // 4
        vm.write_u16(0); // 5, 6
        vm.write_opcode(OpCode::STOP); // 7
        vm.write_opcode(OpCode::LOAD); // 8
        vm.write_u8(1); // 9
        vm.write_u16(111); // 10, 11
        vm.run();
        assert_eq!(vm.registers[1], 111); // JMPF should skip the STOP instruction, allowing the LOAD to execute
    }
}
