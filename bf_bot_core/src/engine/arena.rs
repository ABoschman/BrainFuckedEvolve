use bf::Bot;
use engine::{BotInPlay, Mutation, Polarity, Orientation};
use simul_round::{RoundResult, RoundParams};

#[derive(Debug)]
pub struct Arena<'a> {
    max_steps: u32,
    step_nr: u32,
    tape: Vec<i8>,
    start_bot: BotInPlay<'a>,
    end_bot: BotInPlay<'a>,
}

impl<'a> Arena<'a> {
    pub fn new<'b>(bot1: &'b Bot, bot2: &'b Bot, round_params: &RoundParams) -> Arena<'b> {
        let polarity = if round_params.invert_polarity {
            Polarity::Reversed
        } else {
            Polarity::Normal
        };
        Arena {
            max_steps: round_params.max_steps,
            step_nr: 0,
            tape: Arena::make_tape(round_params.tape_length as usize),
            start_bot: BotInPlay::new(bot1,
                                      round_params.tape_length as i32,
                                      Orientation::Normal,
                                      Polarity::Normal),
            end_bot: BotInPlay::new(bot2,
                                    round_params.tape_length as i32,
                                    Orientation::Reversed,
                                    polarity),
        }
    }

    fn make_tape(length: usize) -> Vec<i8> {
        let mut tape = vec!(0i8; length);
        tape[0] = i8::min_value();
        tape[length - 1] = i8::min_value();
        tape
    }

    pub fn get_tape(&self) -> &Vec<i8> {
        &self.tape
    }

    //FIXME: Code duplication.
    fn step(&mut self) {
        let optional_cell_mutation_1 = Arena::step_bot(&mut self.start_bot, &self.tape);
        let optional_cell_mutation_2 = Arena::step_bot(&mut self.end_bot, &self.tape);
        if let Some(mutation) = optional_cell_mutation_1 {
            self.tape[mutation.get_index()] = self.tape[mutation.get_index()]
                .wrapping_add(mutation.get_addend());
        }
        if let Some(mutation) = optional_cell_mutation_2 {
            self.tape[mutation.get_index()] = self.tape[mutation.get_index()]
                .wrapping_add(mutation.get_addend());
        }
        self.step_nr += 1;
    }

    /// Make the given BotInPlay execute the next instruction.
    fn step_bot(bot_in_play: &mut BotInPlay, tape: &[i8]) -> Option<Mutation> {
        if bot_in_play.program_has_ended() {
            return None;
        }
        let current_cell_is_zero = tape[bot_in_play.get_pos()] == 0;
        let option = bot_in_play.execute_code(current_cell_is_zero);
        bot_in_play.increment_code_pointer();
        option
    }

    //TODO
    fn generate_result(&self) -> RoundResult {
        RoundResult::draw()
    }

    fn exceeded_max_steps(&self) -> bool {
        self.step_nr >= self.max_steps
    }

    /// Returns true if it detects that the game is in a sink state; meaning that both bots have
    /// ended their programs and neither flag is zero.
    fn both_programs_ended(&self) -> bool {
        let neither_flag_is_zero = !self.flag_a_zeroed() && !self.flag_b_zeroed();
        let both_ended = self.start_bot.program_has_ended() && self.end_bot.program_has_ended();
        neither_flag_is_zero && both_ended
    }

    fn flag_a_zeroed(&self) -> bool {
        self.tape[0] == 0
    }

    fn flag_b_zeroed(&self) -> bool {
        self.tape[self.tape.len() - 1] == 0
    }

    /// Checks if at least one of the participating bots has lost.
    /// Call this after each step, if the result is true then the round can be ended.
    fn has_loser(&self, flag_a_previously_zeroed: bool, flag_b_previously_zeroed: bool) -> bool {
        self.start_bot.bot_is_off_tape(&(self.tape.len() as i32)) ||
        (flag_a_previously_zeroed && self.flag_a_zeroed()) ||
        self.end_bot.bot_is_off_tape(&(self.tape.len() as i32)) ||
        (flag_b_previously_zeroed && self.flag_b_zeroed())
    }
}

impl<'a> Iterator for Arena<'a> {
    type Item = RoundResult;

    fn next(&mut self) -> Option<RoundResult> {
        if self.exceeded_max_steps() || self.both_programs_ended() {
            return Some(RoundResult::draw());
        }
        let flag_a_previously_zeroed = self.flag_a_zeroed();
        let flag_b_previously_zeroed = self.flag_b_zeroed();
        self.step();
        if self.has_loser(flag_a_previously_zeroed, flag_b_previously_zeroed) {
            Some(self.generate_result())
        } else {
            Some(RoundResult::round_ongoing())
        }
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

    fn make_round_params(max_steps: u32) -> RoundParams {
        RoundParams {
            tape_length: 10,
            invert_polarity: false,
            max_steps: max_steps,
        }
    }

    #[test]
    fn arena_maxStepsIsZero_returnsDrawAfterFirstStep() {
        let round_params = make_round_params(0);
        let bot_a = make_bot_idle_three_turns();
        let bot_b = make_bot_idle_three_turns();
        let mut arena = Arena::new(&bot_a, &bot_b, &round_params);
        assert_eq!(arena.next().unwrap(), RoundResult::draw());
    }

    #[test]
    fn iterator_maxStepsIsOne_returnsDrawAfterSecondStep() {
        let round_params = make_round_params(1);
        let bot_a = make_bot_idle_three_turns();
        let bot_b = make_bot_idle_three_turns();
        let mut arena = Arena::new(&bot_a, &bot_b, &round_params);
        assert_eq!(arena.next().unwrap(), RoundResult::round_ongoing());
        assert_eq!(arena.next().unwrap(), RoundResult::draw());
    }

    #[test]
    fn iterator_bothEmptyBots_returnsDrawAfterFirstStep() {
        let round_params = make_round_params(100_000);
        let bot_a = make_empty_bot();
        let bot_b = make_empty_bot();
        let mut arena = Arena::new(&bot_a, &bot_b, &round_params);
        assert_eq!(arena.next().unwrap(), RoundResult::draw());
    }

    #[test]
    fn iterator_bothFlagsStartAtZero_returnsDrawAfterFirstStep() {
        let round_params = make_round_params(100_000);
        let bot_a = make_bot_idle_three_turns();
        let bot_b = make_bot_idle_three_turns();
        let mut arena = Arena::new(&bot_a, &bot_b, &round_params);
        arena.tape = vec!(0i8; round_params.tape_length as usize);
        assert_eq!(arena.next().unwrap(), RoundResult::draw());
    }

}
