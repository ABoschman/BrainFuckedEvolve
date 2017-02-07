#![allow(dead_code, unused_variables, unused_imports)]//TODO: Remove this debug line.

use bot::Bot;
use bot_in_play::{BotInPlay, Mutation, Polarity, Orientation};
use round::round_result::RoundResult;


#[derive(Debug)]
pub struct Arena<'a> {
    tape: Vec<i8>,
    start_bot: BotInPlay<'a>,
    end_bot: BotInPlay<'a>,
    flag_a_previously_zero: bool,
    flag_b_previously_zero: bool,    
}

impl<'a> Arena<'a> {

    pub fn new<'b>(bot1: &'b Bot, bot2: &'b Bot, length: u32, has_reversed_polarity: bool) -> Arena<'b>  {
        let polarity = if has_reversed_polarity { Polarity::Reversed } else { Polarity::Normal };
        Arena {
            tape: Arena::make_tape(length as usize),
            start_bot: BotInPlay::new(bot1, length as i32, Orientation::Normal, Polarity::Normal),
            end_bot: BotInPlay::new(bot2, length as i32, Orientation::Reversed, polarity),
            flag_a_previously_zero: false,
            flag_b_previously_zero: false,
        }
    }

    fn make_tape(length: usize) -> Vec<i8> {
        let mut tape = vec!(0i8; length);
        tape[0] = i8::min_value();
        tape[length - 1] = i8::min_value();
        tape
    }

    pub fn step(&mut self) {
        self.flag_a_previously_zero = self.tape[0] == 0;
        self.flag_b_previously_zero = self.tape[self.tape.len()] == 0;
        let optional_cell_mutation_1 = Arena::step_bot(&mut self.start_bot, &self.tape);
        let optional_cell_mutation_2 = Arena::step_bot(&mut self.end_bot, &self.tape);
        if let Some(mutation) = optional_cell_mutation_1 {
            self.tape[mutation.get_index()] = self.tape[mutation.get_index()].wrapping_add(mutation.get_addend()); 
        }
        if let Some(mutation) = optional_cell_mutation_2 {
            self.tape[mutation.get_index()] = self.tape[mutation.get_index()].wrapping_add(mutation.get_addend()); 
        }
    }

    /// Make the given BotInPlay execute the next instruction. 
    fn step_bot(bot_in_play: &mut BotInPlay, tape: &Vec<i8>) -> Option<Mutation> {
        if bot_in_play.program_has_ended() {
            return None;
        }
        let current_cell_is_zero = tape[bot_in_play.get_pos()] == 0;
        let option = bot_in_play.execute_code(current_cell_is_zero);
        bot_in_play.increment_code_pointer();
        option
    }

    /// Checks if at least one of the participating bots has lost.
    /// Call this after each step, if the result is true then the round can be ended.
    pub fn has_loser(&self) -> bool {
        self.start_bot.bot_is_off_tape(&(self.tape.len() as i32))
        || (self.flag_a_previously_zero && self.tape[0]==0)
        || self.end_bot.bot_is_off_tape(&(self.tape.len() as i32))
        || (self.flag_b_previously_zero && self.tape[self.tape.len()]==0)
    }

    pub fn generate_result(&self) -> RoundResult {
        RoundResult::new(false,false)//TODO
    }

    fn bot_lost(&self, bot_in_play: &BotInPlay) -> bool {
        true//todo
    }

    pub fn get_tape(&self) -> &Vec<i8> {
        &self.tape
    }

}