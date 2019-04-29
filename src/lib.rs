use std::io::stdin;

#[derive(Debug)]
pub enum Instruction {
    PointerIncrement,
    PointerDesrement,
    Increment,
    Decrement,
    Put,
    Get,
    Begin,
    End,
    Nothing,
}

impl Instruction {
    pub fn new(input: char) -> Self {
        match input {
            '>' => Instruction::PointerIncrement,
            '<' => Instruction::PointerDesrement,
            '+' => Instruction::Increment,
            '-' => Instruction::Decrement,
            '.' => Instruction::Put,
            ',' => Instruction::Get,
            '[' => Instruction::Begin,
            ']' => Instruction::End,
            _ => Instruction::Nothing,
        }
    }

    pub fn custom(input: &str, custom: &CustomInstruction) -> Self {
        if input == custom.pointer_increment {
            Instruction::PointerIncrement
        } else if input == custom.pointer_desrement {
            Instruction::PointerDesrement
        } else if input == custom.increment {
            Instruction::Increment
        } else if input == custom.decrement {
            Instruction::Decrement
        } else if input == custom.put {
            Instruction::Put
        } else if input == custom.get {
            Instruction::Get
        } else if input == custom.begin {
            Instruction::Begin
        } else if input == custom.end {
            Instruction::End
        } else {
            Instruction::Nothing
        }
    }
}

#[derive(Debug)]
pub struct CustomInstruction {
    pub pointer_increment: String,
    pub pointer_desrement: String,
    pub increment: String,
    pub decrement: String,
    pub put: String,
    pub get: String,
    pub begin: String,
    pub end: String,
}

pub struct Machine {
    memory: [u8; 256],
    pointer: usize,
    index: usize,
    instructions: Vec<Instruction>,
}

impl Machine {
    pub fn new(input: &str) -> Self {
        let instructions = input.chars().map(Instruction::new).collect();

        Self {
            memory: [0; 256],
            pointer: 0,
            index: 0,
            instructions,
        }
    }

    pub fn custom(input: &str, custom: &CustomInstruction) -> Self {
        let instructions = input
            .split_whitespace()
            .map(|c| Instruction::custom(c, &custom))
            .collect();

        Self {
            memory: [0; 256],
            pointer: 0,
            index: 0,
            instructions,
        }
    }

    pub fn run(&mut self) {
        while self.index < self.instructions.len() {
            self.exec_instruction();
            self.index += 1;
        }
    }

    fn exec_instruction(&mut self) {
        match self.instructions[self.index] {
            Instruction::PointerIncrement => self.pointer += 1,
            Instruction::PointerDesrement => self.pointer -= 1,
            Instruction::Increment => self.memory[self.pointer] += 1,
            Instruction::Decrement => self.memory[self.pointer] -= 1,
            Instruction::Put => print!("{}", char::from(self.memory_content())),
            Instruction::Get => self.exec_get_instruction(),
            Instruction::Begin => self.exec_begin_instruction(),
            Instruction::End => self.exec_end_instruction(),
            Instruction::Nothing => (),
        }
    }

    fn exec_get_instruction(&mut self) {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        self.memory[self.pointer] = buf.as_bytes()[0]
    }

    fn exec_begin_instruction(&mut self) {
        if self.memory_content() != 0 {
            return;
        }

        let mut end_count = 1;
        while end_count > 0 {
            self.index += 1;
            match self.instructions[self.index] {
                Instruction::Begin => end_count += 1,
                Instruction::End => end_count -= 1,
                _ => (),
            }
        }
    }
    fn exec_end_instruction(&mut self) {
        if self.memory_content() == 0 {
            return;
        }

        let mut begin_count = 1;
        while begin_count > 0 {
            self.index -= 1;
            match self.instructions[self.index] {
                Instruction::Begin => begin_count -= 1,
                Instruction::End => begin_count += 1,
                _ => (),
            }
        }
    }

    fn memory_content(&self) -> u8 {
        self.memory[self.pointer]
    }
}
