#![allow(dead_code, unused_variables)]

extern crate bf_bot_core;
extern crate bf_bot_compiler;

use bf_bot_compiler::parser;
use bf_bot_core::bot::Bot;
use bf_bot_core::bot::Instruction;
use bf_bot_core::workingtitle;
use bf_bot_core::workingtitle::arena::Arena;
use bf_bot_core::workingtitle::bot_in_play::BotInPlay;
use bf_bot_core::workingtitle::bot_in_play::StartingPos;
use bf_bot_core::workingtitle::bot_in_play::Polarity;
use bf_bot_core::workingtitle::bot_in_play::Mutation;

fn main() {
    // print!("Bot1 input string: ");
    // let bot1: Bot = make_test_bot(">+[[[+]]]--------".to_string());
    // print!("Bot2 input string: ");
    // let bot2: Bot = make_test_bot(">+++++_+++++ >-----_----- >+>- >+>- >+>-".to_string());
    print!("Bot1 input string: ");
    let bot1: Bot = make_test_bot("[>-+-<]".to_string());
    print!("Bot2 input string: ");
    let bot2: Bot = make_test_bot("[>+-+<]".to_string());    
    let mut arena = Arena::new(&bot1, &bot2, 10, false);
    println!("{:?} << Initial Tape", arena.get_tape());
    println!("Bot1: {:#?}", bot1);
    println!("Bot2: {:#?}", bot2);
    game::determine_winner(&mut arena);
}

fn make_test_bot(code: String) -> Bot {
    println!("{}", code);
    Bot::new(parser::parse_bot(code))
}