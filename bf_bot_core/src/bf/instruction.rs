#[derive(Debug, PartialEq)]
pub enum Instruction {
    MoveBack,
    MoveForward,
    Increment,
    Decrement,
    StartWhileNotZero { target_pointer: usize },
    EndWhileNotZero { target_pointer: usize },
    SkipExecution,
    StartFor { target_pointer: usize },
    EndFor {
        target_pointer: usize,
        nr_iterations: usize,
    },
    Comment,
}
