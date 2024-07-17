use super::lexer::{Token, TokenType};
use crate::vm::VM;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    vm: VM,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, vm: VM) -> Self {
        Self {
            tokens,
            current: 0,
            vm,
        }
    }

    pub fn parse(&mut self) -> VM {
        while !self.is_at_end() {
            self.next_instruction();
        }

        self.vm.clone()
    }

    fn next_instruction(&mut self) {
        let token = self.advance();

        let token_type = token.token_type;
        match token_type {
            TokenType::OpCode(opcode) => {
                self.vm.write_opcode(opcode);
            }
            TokenType::Register(register) => {
                self.vm.write_u8(register);
            }
            TokenType::Integer(integer) => {
                self.vm.write_u16(integer);
            }
        }
    }

    fn advance(&mut self) -> &Token {
        self.current += 1;
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
