
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