

/// Represents a Bot. Holds variables that are related to the Bot in general.
#[derive(Debug)]
pub struct Bot {
    program: Vec<Instruction>,
}

impl Bot {
    pub fn get_program(&self) -> &Vec<Instruction> {
        &self.program
    }

    pub fn new(program: Vec<Instruction>) -> Bot {
        Bot {
            program: program
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Instruction {
    MoveBack,
    MoveForward,
    Increment,
    Decrement,
    ConditionalGoToForward{target_pointer: usize},
    ConditionalGoToBack{target_pointer: usize},
    DoNothing,
    Comment,
    Placeholder,
}