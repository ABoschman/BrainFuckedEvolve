/// In half the matches, one of the bots will have its polarity reversed. 
/// This eliminates the strategy of taking a successful bot's code and merely exchanging + for - and vice versa.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Polarity {
    /// Original polarity, aka Sieve. Decrement is interpreted as lowering the value of the cell, Increment as increasing it.
    Normal, 
    /// Reversed polarity, aka Kettle. Decrement is interpreted as increasing the value of a cell, Increment as lowering it.
    Reversed,
}

impl Polarity {
    /// Converts the intended cell mutation from the Bot's perspective to fit the tape's perspective, 
    /// by adding consideration for the BotInPlay's polarity. 
    /// Nothing changes if the BotInPlay has normal polarity, but BotInPlay's with reversed polarity will 
    /// increment when they mean to decrement and vice versa.
    ///
    /// # Examples
    ///
    /// ```
    /// use bf_bot_core::bot_in_play::Polarity;
    /// assert_eq!(Polarity::Normal.mutation_relative_to_tape(1), 1);
    /// assert_eq!(Polarity::Normal.mutation_relative_to_tape(-1), -1);
    /// assert_eq!(Polarity::Reversed.mutation_relative_to_tape(1), -1);
    /// assert_eq!(Polarity::Reversed.mutation_relative_to_tape(-1), 1);
    /// ```
    pub fn mutation_relative_to_tape(&self, addend: i8) -> i8 {
        if self == &Polarity::Normal { addend } else { -addend }
    }
}