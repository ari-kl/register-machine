use std::collections::HashMap;

use crate::opcode::OpCode;

#[derive(Clone)]
pub struct VM {
    pub registers: [i64; 256],
    pub pc: usize,
    pub code: Vec<u8>,
    pub comparison: bool,
    pub syscalls: HashMap<i64, fn(&mut VM) -> bool>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 256],
            pc: 0,
            code: vec![],
            comparison: false,
            syscalls: HashMap::new(),
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
            OpCode::JFW => {
                let offset = self.registers[self.read_u8() as usize] as usize;
                self.pc += offset;
                true
            }
            OpCode::JBK => {
                let offset = self.registers[self.read_u8() as usize] as usize;
                self.pc -= offset;
                true
            }
            OpCode::EQ => {
                let register1 = self.read_u8() as usize;
                let register2 = self.read_u8() as usize;
                self.comparison = self.registers[register1] == self.registers[register2];
                true
            }
            OpCode::NEQ => {
                let register1 = self.read_u8() as usize;
                let register2 = self.read_u8() as usize;
                self.comparison = self.registers[register1] != self.registers[register2];
                true
            }
            OpCode::GT => {
                let register1 = self.read_u8() as usize;
                let register2 = self.read_u8() as usize;
                self.comparison = self.registers[register1] > self.registers[register2];
                true
            }
            OpCode::LT => {
                let register1 = self.read_u8() as usize;
                let register2 = self.read_u8() as usize;
                self.comparison = self.registers[register1] < self.registers[register2];
                true
            }
            OpCode::GTE => {
                let register1 = self.read_u8() as usize;
                let register2 = self.read_u8() as usize;
                self.comparison = self.registers[register1] >= self.registers[register2];
                true
            }
            OpCode::LTE => {
                let register1 = self.read_u8() as usize;
                let register2 = self.read_u8() as usize;
                self.comparison = self.registers[register1] <= self.registers[register2];
                true
            }
            OpCode::JEQ => {
                let address = self.registers[self.read_u8() as usize] as usize;
                if self.comparison {
                    self.pc = address;
                }
                true
            }
            OpCode::JNE => {
                let address = self.registers[self.read_u8() as usize] as usize;
                if !self.comparison {
                    self.pc = address;
                }
                true
            }
            OpCode::SYS => {
                let syscall_id = self.registers[self.read_u8() as usize];
                let syscall = self.syscalls.get(&syscall_id);

                return match syscall {
                    Some(syscall) => syscall(self),
                    None => {
                        println!("Unknown syscall: {}", syscall_id);
                        false
                    }
                };
            }
            OpCode::UKWN => {
                println!("Unknown opcode: {}", self.code[self.pc]);
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

    pub fn add_syscall(&mut self, id: i64, syscall: fn(&mut VM) -> bool) {
        self.syscalls.insert(id, syscall);
    }
}

#[cfg(test)]
mod vm_tests {
    use super::*;

    #[test]
    fn test_new() {
        let vm = VM::new();
        assert_eq!(vm.pc, 0);
        assert_eq!(vm.registers, [0; 256]);
        assert_eq!(vm.comparison, false);
        assert_eq!(vm.code, Vec::new());
    }

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
        vm.write_u16(7); // 2, 3
        vm.write_opcode(OpCode::JMP); // 4
        vm.write_u8(0); // 5
        vm.write_opcode(OpCode::STOP); // 6
        vm.write_opcode(OpCode::LOAD); // 7
        vm.write_u8(1); // 8
        vm.write_u16(111); // 9, 10
        vm.run();
        assert_eq!(vm.registers[1], 111); // JMP should skip the STOP instruction, allowing the LOAD to execute
    }

    #[test]
    fn test_jfw() {
        let mut vm = VM::new();
        vm.write_opcode(OpCode::LOAD); // 0
        vm.write_u8(0); // 1
        vm.write_u16(1); // 2, 3
        vm.write_opcode(OpCode::JFW); // 4
        vm.write_u8(0); // 5
        vm.write_opcode(OpCode::STOP); // 6
        vm.write_opcode(OpCode::LOAD); // 7
        vm.write_u8(1); // 8
        vm.write_u16(111); // 9, 10
        vm.run();
        assert_eq!(vm.registers[1], 111); // JMPF should skip the STOP instruction, allowing the LOAD to execute
    }

    #[test]
    fn test_jbk() {
        // We will skip over a LOAD instruction and then jump back to it to test JMPB
        let mut vm = VM::new();
        vm.write_opcode(OpCode::LOAD); // 0
        vm.write_u8(0); // 1
        vm.write_u16(15); // 2, 3
        vm.write_opcode(OpCode::LOAD); // 4
        vm.write_u8(1); // 5
        vm.write_u16(7); // 6, 7
        vm.write_opcode(OpCode::JMP); // 8
        vm.write_u8(0); // 9
        vm.write_opcode(OpCode::LOAD); // 10
        vm.write_u8(1); // 11
        vm.write_u16(111); // 12, 13
        vm.write_opcode(OpCode::STOP); // 14
        vm.write_opcode(OpCode::JBK); // 15
        vm.write_u8(1); // 16
        vm.run();
        assert_eq!(vm.registers[1], 111); // JMPB should go back to the LOAD instruction at 10
    }

    #[test]
    fn test_eq() {
        let mut vm = VM::new();

        // Test equality = true
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(13);
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(1);
        vm.write_u16(13);
        vm.write_opcode(OpCode::EQ);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.comparison, true);

        // Test equality = false
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(12);
        vm.write_opcode(OpCode::EQ);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.comparison, false);
    }

    #[test]
    fn test_neq() {
        let mut vm = VM::new();

        // Test inequality = true
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(13);
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(1);
        vm.write_u16(14);
        vm.write_opcode(OpCode::NEQ);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.comparison, true);

        // Test inequality = false
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(14);
        vm.write_opcode(OpCode::NEQ);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.comparison, false);
    }

    #[test]
    fn test_gt() {
        let mut vm = VM::new();

        // Test greater than = true
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(14);
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(1);
        vm.write_u16(13);
        vm.write_opcode(OpCode::GT);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.comparison, true);

        // Test greater than = false
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(13);
        vm.write_opcode(OpCode::GT);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.comparison, false);
    }

    #[test]
    fn test_lt() {
        let mut vm = VM::new();

        // Test less than = true
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(13);
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(1);
        vm.write_u16(14);
        vm.write_opcode(OpCode::LT);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.comparison, true);

        // Test less than = false
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(15);
        vm.write_opcode(OpCode::LT);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.comparison, false);
    }

    #[test]
    fn test_gte() {
        let mut vm = VM::new();

        // Test greater than or equal = true
        // Greater than
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(14);
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(1);
        vm.write_u16(13);
        vm.write_opcode(OpCode::GTE);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.comparison, true);

        // Equal to
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(13);
        vm.write_opcode(OpCode::GTE);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.comparison, true);

        // Test greater than or equal = false
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(12);
        vm.write_opcode(OpCode::GTE);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.comparison, false);
    }

    #[test]
    fn test_lte() {
        let mut vm = VM::new();

        // Test less than or equal = true
        // Less than
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(13);
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(1);
        vm.write_u16(14);
        vm.write_opcode(OpCode::LTE);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.comparison, true);

        // Equal to
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(13);
        vm.write_opcode(OpCode::LTE);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.comparison, true);

        // Test less than or equal = false
        vm.write_opcode(OpCode::LOAD);
        vm.write_u8(0);
        vm.write_u16(15);
        vm.write_opcode(OpCode::LTE);
        vm.write_u8(0);
        vm.write_u8(1);
        vm.run();
        assert_eq!(vm.comparison, false);
    }

    #[test]
    fn test_jeq() {
        let mut vm = VM::new();

        vm.write_opcode(OpCode::LOAD); // 0
        vm.write_u8(0); // 1
        vm.write_u16(13); // 2, 3
        vm.write_opcode(OpCode::LOAD); // 4
        vm.write_u8(1); // 5
        vm.write_u16(13); // 6, 7
        vm.write_opcode(OpCode::LOAD); // 8
        vm.write_u8(2); // 9
        vm.write_u16(18); // 10, 11
        vm.write_opcode(OpCode::EQ); // 12
        vm.write_u8(0); // 13
        vm.write_u8(1); // 14
        vm.write_opcode(OpCode::JEQ); // 15
        vm.write_u8(2); // 16
        vm.write_opcode(OpCode::STOP); // 17
        vm.write_opcode(OpCode::LOAD); // 18
        vm.write_u8(3); // 19
        vm.write_u16(14); // 20, 21
        vm.run();

        assert_eq!(vm.comparison, true);
        assert_eq!(vm.registers[3], 14); // Should have jumped to 18 and skipped over the stop instruction
    }

    #[test]
    fn test_jne() {
        let mut vm = VM::new();

        vm.write_opcode(OpCode::LOAD); // 0
        vm.write_u8(0); // 1
        vm.write_u16(13); // 2, 3
        vm.write_opcode(OpCode::LOAD); // 4
        vm.write_u8(1); // 5
        vm.write_u16(13); // 6, 7
        vm.write_opcode(OpCode::LOAD); // 8
        vm.write_u8(2); // 9
        vm.write_u16(18); // 10, 11
        vm.write_opcode(OpCode::EQ); // 12
        vm.write_u8(0); // 13
        vm.write_u8(1); // 14
        vm.write_opcode(OpCode::JNE); // 15
        vm.write_u8(2); // 16
        vm.write_opcode(OpCode::STOP); // 17
        vm.write_opcode(OpCode::LOAD); // 18
        vm.write_u8(3); // 19
        vm.write_u16(14); // 20, 21
        vm.run();

        assert_eq!(vm.comparison, true);
        assert_ne!(vm.registers[3], 14); // Should have jumped to 14 and executed the stop instruction, never loading 14 into register 3
    }

    #[test]
    fn test_sys() {
        let mut vm = VM::new();

        vm.add_syscall(0, |vm| {
            vm.registers[0] = 321;

            true
        });

        vm.write_opcode(OpCode::SYS);
        vm.write_u8(0);
        vm.run();

        assert_eq!(vm.registers[0], 321);
    }
}
