/// Specifies the conditions of a single round of Brainfuck Jousting.
#[derive(Debug)]
#[derive(PartialEq)]
pub struct RoundParams {
    pub tape_length: u32,
    pub invert_polarity: bool,
    pub max_steps: u32,
}