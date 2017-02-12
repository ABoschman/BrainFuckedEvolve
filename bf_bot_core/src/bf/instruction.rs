#[derive(Debug)]
#[derive(PartialEq)]
pub enum Instruction {
    MoveBack,
    MoveForward,
    Increment,
    Decrement,
    WhileNotZeroOpen { target_pointer: usize },
    WhileNotZeroClose { target_pointer: usize },
    DoNothing,
    Comment,
    Placeholder,
}
