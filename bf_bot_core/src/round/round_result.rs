//TODO: Consideration: Result is a special thing in Rust std. Maybe rename this struct?
pub struct RoundResult {
    pub bot_a_lost: bool,
    pub bot_b_lost: bool,
}

impl RoundResult {

    pub fn new(bot_a_lost: bool, bot_b_lost: bool) -> RoundResult {
        RoundResult { bot_a_lost: bot_a_lost, bot_b_lost: bot_b_lost }        
    }

    /// Returns true if this round has a winner. A round has a winner if and only if exactly one bot is a loser.
    /// If both bots are marked as winner or loser, that means the round is a draw.
    ///
    /// # Examples
    /// 
    /// ```
    /// use bf_bot_core::round::RoundResult;
    /// assert!(!RoundResult::new(false, false).has_winner());
    /// assert!(RoundResult::new(true, false).has_winner());
    /// assert!(RoundResult::new(false, true).has_winner());
    /// assert!(!RoundResult::new(true, true).has_winner());
    /// ```
    pub fn has_winner(&self) -> bool {
        self.bot_a_lost ^ self.bot_b_lost
    }

}