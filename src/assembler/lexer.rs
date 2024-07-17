use crate::opcode::OpCode;

#[derive(Copy, Clone)]
pub enum TokenType {
    OpCode(OpCode),
    Register(u8),
    Integer(u16),
}

pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    input: String,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<(), String> {
        while !self.is_at_end() {
            if let Err(e) = self.next_token() {
                return Err(e);
            }
        }

        Ok(())
    }

    fn next_token(&mut self) -> Result<(), String> {
        self.skip_whitespace();

        if self.is_at_end() {
            return Ok(());
        }

        self.start = self.current;

        let c = self.advance();

        match c {
            '!' => Ok(self.comment()),
            'a'..='z' | 'A'..='Z' => Ok(self.opcode()),
            '%' => Ok(self.register()),
            '#' => Ok(self.integer()),
            _ => Err(format!("Unexpected character: {}", c)),
        }
    }

    fn opcode(&mut self) {
        while self.peek().is_alphabetic() {
            self.advance();
        }

        let text = &self.input[self.start..self.current];
        let opcode = match text.to_lowercase().as_str() {
            "stop" => OpCode::STOP,
            "load" => OpCode::LOAD,
            "add" => OpCode::ADD,
            "sub" => OpCode::SUB,
            "mul" => OpCode::MUL,
            "div" => OpCode::DIV,
            "jmp" => OpCode::JMP,
            "jfw" => OpCode::JFW,
            "jbk" => OpCode::JBK,
            "eq" => OpCode::EQ,
            "neq" => OpCode::NEQ,
            "gt" => OpCode::GT,
            "lt" => OpCode::LT,
            "gte" => OpCode::GTE,
            "lte" => OpCode::LTE,
            "jeq" => OpCode::JEQ,
            "jne" => OpCode::JNE,
            "sys" => OpCode::SYS,
            _ => OpCode::UKWN,
        };

        self.add_token(TokenType::OpCode(opcode));
    }

    fn register(&mut self) {
        let mut value = 0;
        while self.peek().is_digit(10) {
            let digit = self.advance().to_digit(10).unwrap();
            value = value * 10 + digit as u16;
        }

        self.add_token(TokenType::Register(value as u8));
    }

    fn integer(&mut self) {
        let mut value = 0;
        while self.peek().is_digit(10) {
            let digit = self.advance().to_digit(10).unwrap();
            value = value * 10 + digit as u16;
        }

        self.add_token(TokenType::Integer(value));
    }

    fn comment(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let token = Token {
            token_type,
            line: self.line,
            column: self.column,
        };

        self.tokens.push(token);
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.column = 1;
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;
        self.input.chars().nth(self.current - 1).unwrap()
    }

    fn peek(&self) -> char {
        self.input.chars().nth(self.current).unwrap_or('\0')
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
}
