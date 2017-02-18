#![allow(dead_code, unused_variables)]

extern crate bf_bot_core as core;
extern crate bf_bot_compiler as compiler;

use compiler::parser;
use core::bf::Bot;
use core::simul_game;

fn main() {
    print!("Bot1 input string: ");
    let bot1: Bot = make_test_bot("> +[.]".to_string());
    print!("Bot2 input string: ");
    let bot2: Bot = make_test_bot("> +[.]".to_string());

    let result = simul_game::run_complete(&bot1, &bot2);
    println!("{:?}", result);
}

fn make_test_bot(code: String) -> Bot {
    // println!("{}", code);
    let bot = Bot::new(parser::parse_bot(code));
    println!("{:?}", &bot.get_program());
    bot
}
