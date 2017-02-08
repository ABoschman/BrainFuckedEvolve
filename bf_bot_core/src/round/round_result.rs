#[derive(PartialEq, Debug)]
pub struct RoundResult {
    pub bot_a_lost: bool,
    pub bot_b_lost: bool,
}

impl RoundResult {

    fn new(bot_a_lost: bool, bot_b_lost: bool) -> RoundResult {
        RoundResult { 
            bot_a_lost: bot_a_lost, 
            bot_b_lost: bot_b_lost 
        }
    }

    pub fn round_ongoing() -> RoundResult {
        RoundResult::new(false, false)
    }

    pub fn start_bot_wins() -> RoundResult {
        RoundResult::new(false, true)
    }

    pub fn end_bot_wins() -> RoundResult {
        RoundResult::new(true, false)
    }

    pub fn draw() -> RoundResult {
        RoundResult::new(true, true)
    }

    /// Returns true if this round has a winner. A round has a winner if and only if exactly one bot is a loser.
    /// If both bots are marked as NOT loser, that means that the round is ongoing.
    /// If both bots are marked as loser, that means that the round is a draw.
    ///
    /// # Examples
    /// 
    /// ```
    /// use bf_bot_core::round::RoundResult;
    /// assert!(!RoundResult::round_ongoing().has_winner());
    /// assert!(RoundResult::start_bot_wins().has_winner());
    /// assert!(RoundResult::end_bot_wins().has_winner());
    /// assert!(!RoundResult::draw().has_winner());
    /// ```
    pub fn has_winner(&self) -> bool {
        self.bot_a_lost ^ self.bot_b_lost
    }

    pub fn has_loser(&self) -> bool {
        self.bot_a_lost || self.bot_b_lost
    }

}