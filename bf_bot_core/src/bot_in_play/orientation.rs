

/// The orientation of a BotInPlay is determined by its starting position on the tape.
/// Orientation doesn't affect the gameplay from the bot's perspective, each bot may write their
/// code as though they start at cell zero.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Orientation {
    /// Bot starts off at the start of the tape. To advance forward means to move in the positive
    /// direction.
    Normal,
    /// Bot starts off at the end of the tape. To advance forward means to move in the negative
    /// direction.
    Reversed,
}

impl Orientation {
    /// Takes a desired movement relative to the Bot (i.e.: Forwards towards the enemy flag or
    /// backwards towards its own flag, not taking into account orientation on the tape) and makes
    /// it relative to the tape. For the bot that started out at the start of the tape, the result
    /// will be the same. For the other bot, the result will be inverted.
    ///
    /// # Examples
    ///
    /// ```
    /// use bf_bot_core::bot_in_play::Orientation;
    /// assert_eq!(Orientation::Normal.calc_movement_relative_to_tape(1), 1);
    /// assert_eq!(Orientation::Normal.calc_movement_relative_to_tape(-1), -1);
    /// assert_eq!(Orientation::Reversed.calc_movement_relative_to_tape(1), -1);
    /// assert_eq!(Orientation::Reversed.calc_movement_relative_to_tape(-1), 1);
    /// ```
    pub fn calc_movement_relative_to_tape(&self, direction: i32) -> i32 {
        if self == &Orientation::Normal {
            direction
        } else {
            -direction
        }
    }
}
