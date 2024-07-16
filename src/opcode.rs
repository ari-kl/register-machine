// Instruction comment format:
// registers (u8) are indicated by <reg>
// values (u16) are indicated by [value]

pub enum OpCode {
    // Stops the VM execution
    STOP,
    // Load a value into a register
    // load <dst> [value]
    LOAD,
    // Add two register values together and store the result in a register
    // add <dst> <src1> <src2>
    // dst = src1 + src2
    ADD,
    // Subtract two register values and store the result in a register
    // sub <dst> <src1> <src2>
    // dst = src1 - src2
    SUB,
    // Multiply two register values and store the result in a register
    // mul <dst> <src1> <src2>
    // dst = src1 * src2
    MUL,
    // Divide two register values and store the result in a register
    // div <dst> <src1> <src2>
    // dst = src1 / src2
    DIV,
    // Jump to a specific instruction, address is stored in a register
    // jmp <addr>
    JMP,
    // Jump to a specific instruction, address is stored in a register
    // jmp <offset>
    JMPF,
    // Jump to a specific instruction, address is stored in a register
    // jmp <offset>
    JMPB,
    // Set the equality flag to if two registers contain equal values
    // eq <reg1> <reg2>
    // equality = reg1 == reg2
    EQ,
    // Set the equality flag to if two registers contain different values
    // neq <reg1> <reg2>
    // equality = reg1 != reg2
    NEQ,
    // Set the equality flag to if one register is greater than another
    // gt <reg1> <reg2>
    // equality = reg1 > reg2
    GT,
    // Set the equality flag to if one register is less than another
    // lt <reg1> <reg2>
    // equality = reg1 < reg2
    LT,
    // Set the equality flag to if one register is greater than or equal to another
    // gteq <reg1> <reg2>
    // equality = reg1 >= reg2
    GTEQ,
    // Set the equality flag to if one register is less than or equal to another
    // lteq <reg1> <reg2>
    // equality = reg1 <= reg2
    LTEQ,
    // Jump to an instruction if the equality flag is set to true
    // jeq <addr>
    JEQ,
    // Jump to an instruction if the equality flag is set to false
    // jneq <addr>
    JNEQ,
    // Used to execute user-defined "system calls"
    // Allows adding custom functionality to the VM
    // sys <syscall_id>
    SYS,
    // Unknown opcode
    UKWN,
}

impl From<u8> for OpCode {
    fn from(v: u8) -> Self {
        match v {
            0 => OpCode::STOP,
            1 => OpCode::LOAD,
            2 => OpCode::ADD,
            3 => OpCode::SUB,
            4 => OpCode::MUL,
            5 => OpCode::DIV,
            6 => OpCode::JMP,
            7 => OpCode::JMPF,
            8 => OpCode::JMPB,
            9 => OpCode::EQ,
            10 => OpCode::NEQ,
            11 => OpCode::GT,
            12 => OpCode::LT,
            13 => OpCode::GTEQ,
            14 => OpCode::LTEQ,
            15 => OpCode::JEQ,
            16 => OpCode::JNEQ,
            17 => OpCode::SYS,
            _ => OpCode::UKWN,
        }
    }
}

impl From<&str> for OpCode {
    fn from(s: &str) -> Self {
        match s {
            "stop" => OpCode::STOP,
            "load" => OpCode::LOAD,
            "add" => OpCode::ADD,
            "sub" => OpCode::SUB,
            "mul" => OpCode::MUL,
            "div" => OpCode::DIV,
            "jmp" => OpCode::JMP,
            "jmpf" => OpCode::JMPF,
            "jmpb" => OpCode::JMPB,
            "eq" => OpCode::EQ,
            "neq" => OpCode::NEQ,
            "gt" => OpCode::GT,
            "lt" => OpCode::LT,
            "gteq" => OpCode::GTEQ,
            "lteq" => OpCode::LTEQ,
            "jeq" => OpCode::JEQ,
            "jneq" => OpCode::JNEQ,
            "sys" => OpCode::SYS,
            _ => OpCode::UKWN,
        }
    }
}
