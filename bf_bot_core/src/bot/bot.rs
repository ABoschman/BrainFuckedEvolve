use bot::instruction::Instruction;

/// Represents a Bot. Holds variables that are related to the Bot in general.
#[derive(Debug)]
pub struct Bot {
    program: Vec<Instruction>,
}

impl Bot {
 
    pub fn new(program: Vec<Instruction>) -> Bot {
        Bot {
            program: program
        }
    }

    pub fn get_program(&self) -> &Vec<Instruction> {
        &self.program
    }

}