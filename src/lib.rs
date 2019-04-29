#[derive(Debug, Copy, Clone)]
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

    pub fn run(&mut self) {
        while self.index < self.instructions.len() {
            self.exec_instruction();
            self.index += 1;
        }
    }

    fn exec_instruction(&mut self) {}
}
