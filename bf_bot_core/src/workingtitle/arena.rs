
#![allow(unused_variables, unused_imports)]
#![allow(dead_code)]

use bot::Bot;
use bot::Instruction;
use workingtitle::bot_in_play::BotInPlay;
use workingtitle::bot_in_play::Mutation;
use workingtitle::bot_in_play::Polarity;
use workingtitle::bot_in_play::StartingPos;


#[derive(Debug)]
pub struct Arena<'a> {
    tape: Vec<i8>,
    start_bot: BotInPlay<'a>,
    end_bot: BotInPlay<'a>,
}

impl<'a> Arena<'a> {

    pub fn new<'b>(bot1: &'b Bot, bot2: &'b Bot, length: u32, has_reversed_polarity: bool) -> Arena<'b>  {
        let polarity = if has_reversed_polarity { Polarity::Reversed } else { Polarity::Normal };
        Arena {
            tape: Arena::make_tape(length as usize),
            start_bot: BotInPlay::new(bot1, length as i32, StartingPos::Start, Polarity::Normal),
            end_bot: BotInPlay::new(bot2, length as i32, StartingPos::End, polarity),
        }
    }

    pub fn step(&mut self) {
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

    pub fn make_tape(length: usize) -> Vec<i8> {
        let mut tape = vec!(0i8; length);
        tape[0] = i8::min_value();
        tape[length - 1] = i8::min_value();
        tape
    }

    pub fn get_tape(&self) -> &Vec<i8> {
        &self.tape
    }

}

#[deprecated]
pub fn determine_winner(arena: &mut Arena) -> i32 {
    for i in 0..100_000 {
        arena.step();
    }
    println!("{:?}", arena.tape);
    1 //TODO: return total nr rounds that this game took.
}