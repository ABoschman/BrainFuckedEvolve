use bf::Bot;
use simul_round::RoundParams;
use simul_round::RoundResult;
use engine::Arena;

pub fn play(bot_a: &Bot, bot_b: &Bot, round_params: &RoundParams) -> RoundResult {
    StepsIterator::new(bot_a, bot_b, round_params)
        .find(|&ref outcome| outcome.round_is_finished())
        .unwrap()
}

#[derive(Debug)]
struct StepsIterator<'a> {
    arena: Arena<'a>,
}

impl<'a> StepsIterator<'a> {
    fn new<'b>(bot_a: &'b Bot, bot_b: &'b Bot, round_params: &RoundParams) -> StepsIterator<'b> {
        StepsIterator { arena: Arena::new(bot_a, bot_b, round_params) }
    }
}

impl<'a> Iterator for StepsIterator<'a> {
    type Item = RoundResult;

    fn next(&mut self) -> Option<RoundResult> {
        Some(self.arena.step())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use simul_round::{RoundResult, RoundParams};
    use bf::Instruction;

    /// Constructs a Bot with an empty program.
    fn make_empty_bot() -> Bot {
        Bot::new(vec![])
    }

    /// Constructs a Bot that waits three turns and then terminates its program.
    /// Its program, in BrainFuck: ...
    fn make_bot_idle_three_turns() -> Bot {
        Bot::new(vec![Instruction::DoNothing, Instruction::DoNothing, Instruction::DoNothing])
    }

    /// Constructs a suicidal Bot that moves one step away from the other bot, off the tape.
    /// Its program, in BrainFuck: <
    fn make_suicidal_bot() -> Bot {
        Bot::new(vec![Instruction::MoveBack])
    }

    fn make_round_params(max_steps: u32) -> RoundParams {
        RoundParams {
            tape_length: 10,
            invert_polarity: false,
            max_steps: max_steps,
        }
    }

    #[test]
    fn iter_maxStepsIsZero_returnsDrawAtFirstStep() {
        let round_params = make_round_params(0);
        let bot_a = make_bot_idle_three_turns();
        let bot_b = make_bot_idle_three_turns();
        let mut steps_iter = StepsIterator::new(&bot_a, &bot_b, &round_params);
        assert_eq!(steps_iter.next().unwrap(), RoundResult::draw());
    }

    #[test]
    fn iter_maxStepsIsOne_returnsDrawAtSecondStep() {
        let round_params = make_round_params(1);
        let bot_a = make_bot_idle_three_turns();
        let bot_b = make_bot_idle_three_turns();
        let mut steps_iter = StepsIterator::new(&bot_a, &bot_b, &round_params);
        assert_eq!(steps_iter.next().unwrap(), RoundResult::round_ongoing());
        assert_eq!(steps_iter.next().unwrap(), RoundResult::draw());
    }

    #[test]
    fn iter_maxStepsIsZeroButStartFlagZero_returnsDrawAtFirstStep() {
        let round_params = make_round_params(0);
        let bot_a = make_bot_idle_three_turns();
        let bot_b = make_bot_idle_three_turns();
        let mut steps_iter = StepsIterator::new(&bot_a, &bot_b, &round_params);
        steps_iter.arena.tape[0] = 0;
        assert_eq!(steps_iter.next().unwrap(), RoundResult::draw());
    }

    #[test]
    fn iter_maxStepsIsOneButStartFlagZero_returnsEndBotWinsAtFirstStep() {
        let round_params = make_round_params(1);
        let bot_a = make_bot_idle_three_turns();
        let bot_b = make_bot_idle_three_turns();
        let mut steps_iter = StepsIterator::new(&bot_a, &bot_b, &round_params);
        steps_iter.arena.tape[0] = 0;
        assert_eq!(steps_iter.next().unwrap(), RoundResult::end_bot_wins());
    }

    #[test]
    fn iter_bothEmptyBots_returnsDrawAfterFirstStep() {
        let round_params = make_round_params(100_000);
        let bot_a = make_empty_bot();
        let bot_b = make_empty_bot();
        let mut steps_iter = StepsIterator::new(&bot_a, &bot_b, &round_params);
        assert_eq!(steps_iter.next().unwrap(), RoundResult::draw());
    }

    #[test]
    fn iter_bothFlagsStartAtZero_returnsDrawAfterFirstStep() {
        let round_params = make_round_params(100_000);
        let bot_a = make_bot_idle_three_turns();
        let bot_b = make_bot_idle_three_turns();
        let mut steps_iter = StepsIterator::new(&bot_a, &bot_b, &round_params);
        steps_iter.arena.tape = vec!(0i8; round_params.tape_length as usize);
        assert_eq!(steps_iter.next().unwrap(), RoundResult::draw());
    }

    #[test]
    fn iter_bothProgramsEmptyButOneFlagAtZero_otherBotWins() {
        let round_params = make_round_params(100_000);
        let bot_a = make_empty_bot();
        let bot_b = make_empty_bot();
        let mut steps_iter = StepsIterator::new(&bot_a, &bot_b, &round_params);
        steps_iter.arena.tape[0] = 0;
        assert_eq!(steps_iter.next().unwrap(),
                   RoundResult::end_bot_wins(),
                   "Expected end_bot_wins!");
    }

    #[test]
    fn iter_startBotSuicidal_losesRightAway() {
        let round_params = make_round_params(100_000);
        let bot_a = make_suicidal_bot();
        let bot_b = make_empty_bot();
        let mut steps_iter = StepsIterator::new(&bot_a, &bot_b, &round_params);
        assert_eq!(steps_iter.next().unwrap(),
                   RoundResult::end_bot_wins(),
                   "Expected end_bot_wins!");
    }

    #[test]
    fn iter_endBotSuicidal_losesRightAway() {
        let round_params = make_round_params(100_000);
        let bot_a = make_empty_bot();
        let bot_b = make_suicidal_bot();
        let mut steps_iter = StepsIterator::new(&bot_a, &bot_b, &round_params);
        assert_eq!(steps_iter.next().unwrap(),
                   RoundResult::start_bot_wins(),
                   "Expected start_bot_wins!");
    }

    #[test]
    fn iter_bothBotsSuicidal_drawRightAway() {
        let round_params = make_round_params(100_000);
        let bot_a = make_suicidal_bot();
        let bot_b = make_suicidal_bot();
        let mut steps_iter = StepsIterator::new(&bot_a, &bot_b, &round_params);
        assert_eq!(steps_iter.next().unwrap(),
                   RoundResult::draw(),
                   "Expected draw!");
    }

    #[test]
    fn iter_bothBotsSuicidalAndOneFlagZero_drawRightAway() {
        let round_params = make_round_params(100_000);
        let bot_a = make_suicidal_bot();
        let bot_b = make_suicidal_bot();
        let mut steps_iter = StepsIterator::new(&bot_a, &bot_b, &round_params);
        steps_iter.arena.tape[0] = 0;
        assert_eq!(steps_iter.next().unwrap(),
                   RoundResult::draw(),
                   "Expected draw!");
    }
}
