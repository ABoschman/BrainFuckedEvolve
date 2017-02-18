use bf::Bot;
use engine::{BotInPlay, Mutation, Polarity, Orientation};
use simul_round::{RoundResult, RoundParams};

#[derive(Debug)]
pub struct Arena<'a> {
    max_steps: u32,
    step_nr: u32,
    pub tape: Vec<i8>, //FIXME: make this not public. Had to do this to give unit tests access.
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

    pub fn step(&mut self) -> RoundResult {
        if self.exceeded_max_steps() || self.sink_state_detected() {
            return RoundResult::draw();
        }
        let flag_a_previously_zeroed = self.flag_a_zeroed();
        let flag_b_previously_zeroed = self.flag_b_zeroed();
        self.execute_instructions();
        self.generate_result(flag_a_previously_zeroed, flag_b_previously_zeroed)
    }

    fn execute_instructions(&mut self) {
        let optional_cell_mutation_1 = Arena::step_bot(&mut self.start_bot, &self.tape);
        let optional_cell_mutation_2 = Arena::step_bot(&mut self.end_bot, &self.tape);
        self.apply_mutation(optional_cell_mutation_1);
        self.apply_mutation(optional_cell_mutation_2);
        self.step_nr += 1;
    }

    /// Make the given BotInPlay execute the next instruction.
    fn step_bot(bot_in_play: &mut BotInPlay, tape: &[i8]) -> Option<Mutation> {
        if bot_in_play.program_has_ended() {
            return None;
        }
        let cell_is_zero = Arena::cell_is_zero(tape, bot_in_play.get_pos());
        bot_in_play.execute_code(cell_is_zero)
    }

    fn cell_is_zero(tape: &[i8], index: usize) -> bool {
        tape[index] == 0
    }

    fn apply_mutation(&mut self, optional_cell_mutation: Option<Mutation>) {
        if let Some(mutation) = optional_cell_mutation {
            self.tape[mutation.get_index()] = self.tape[mutation.get_index()]
                .wrapping_add(mutation.get_addend());
        }
    }

    fn generate_result(&self,
                       flag_a_previously_zeroed: bool,
                       flag_b_previously_zeroed: bool)
                       -> RoundResult {
        let start_bot_lost = self.start_bot.bot_is_off_tape(&(self.tape.len() as i32)) ||
                             (flag_a_previously_zeroed && self.flag_a_zeroed());
        let end_bot_lost = self.end_bot.bot_is_off_tape(&(self.tape.len() as i32)) ||
                           (flag_b_previously_zeroed && self.flag_b_zeroed());
        RoundResult::new(start_bot_lost, end_bot_lost)
    }

    fn exceeded_max_steps(&self) -> bool {
        self.step_nr >= self.max_steps
    }

    /// Returns true if it detects that the game is in a sink state; meaning that both bots have
    /// ended their programs and neither flag is currently zero.
    fn sink_state_detected(&self) -> bool {
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
}
