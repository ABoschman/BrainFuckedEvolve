//! Iterator that returns RoundParams.
//! This returns RoundParams for every round in a complete game; meaning 42 rounds, half with
//! reversed polarity and covering tape lengths from 10 through 30.

use round::RoundParams;

/// A complete game consists of rounds with tape lengths from MIN_TAPE_LENGTH to MAX_TAPE_LENGTH.
/// Both bounds are inclusive.
const MIN_TAPE_LENGTH: u32 = 10;
/// A complete game consists of rounds with tape lengths from MIN_TAPE_LENGTH to MAX_TAPE_LENGTH.
/// Both bounds are inclusive.
const MAX_TAPE_LENGTH: u32 = 30;
/// Max steps in a round for a complete game.
/// If an incomplete game is run for performance reasons, the max steps may be smaller than this
/// value to save CPU time. However, if a smaller value than this is used, note that the result of
/// the game may differ from reality.
const COMPLETE_GAME_MAX_STEPS: u32 = 100_000;

/// An iterator that returns all rounds in a complete game, covering all possible tape lengths and
/// both polarities.
pub struct AllRounds {
    tape_length: u32,
    invert_polarity: bool,
}

impl AllRounds {
    pub fn new() -> AllRounds {
        AllRounds {
            tape_length: MIN_TAPE_LENGTH,
            invert_polarity: false,
        }
    }

    fn current_item(&self) -> RoundParams {
        RoundParams {
            tape_length: self.tape_length,
            invert_polarity: self.invert_polarity,
            max_steps: COMPLETE_GAME_MAX_STEPS,
        }
    }

    fn update_state(&mut self) {
        self.tape_length = if self.invert_polarity {
            self.tape_length + 1
        } else {
            self.tape_length
        };
        self.invert_polarity = !self.invert_polarity;
    }
}

impl Iterator for AllRounds {
    type Item = RoundParams;

    fn next(&mut self) -> Option<RoundParams> {
        if self.tape_length <= MAX_TAPE_LENGTH {
            let params = self.current_item();
            self.update_state();
            Some(params)
        } else {
            None
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use super::*;

    #[test]
    fn allRounds_gives42Rounds() {
        assert_eq!(AllRounds::new().count(), 42);
    }

    #[test]
    fn allRounds_gives21ReversedPolarityRounds() {
        let all_inverted_rounds = AllRounds::new().filter(|round| round.invert_polarity);
        assert_eq!(all_inverted_rounds.count(), 21);
    }

    #[test]
    fn allRounds_tapeIsAtLeast10Long() {
        assert!(AllRounds::new().all(|round| round.tape_length >= 10));
    }

    #[test]
    fn allRounds_tapeIsAtMost30Long() {
        assert!(AllRounds::new().all(|round| round.tape_length <= 30));
    }

}
