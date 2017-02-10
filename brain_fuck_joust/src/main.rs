#![allow(dead_code, unused_variables)]

extern crate bf_bot_core as core;
extern crate bf_bot_compiler as compiler;

use compiler::parser;
use core::bf::Bot;

fn main() {
    // print!("Bot1 input string: ");
    // let bot1: Bot = make_test_bot(">+[[[+]]]--------".to_string());
    // print!("Bot2 input string: ");
    // let bot2: Bot = make_test_bot(">+++++_+++++ >-----_----- >+>- >+>- >+>-".to_string());
    print!("Bot1 input string: ");
    let bot1: Bot = make_test_bot("[>-+-<]".to_string());
    print!("Bot2 input string: ");
    let bot2: Bot = make_test_bot("[>+-+<]".to_string());
    // let arena = Arena::new(&bot1, &bot2, 10, false);
    // println!("{:?} << Initial Tape", arena.get_tape());
    println!("Bot1: {:#?}", bot1);
    println!("Bot2: {:#?}", bot2);
}

fn make_test_bot(code: String) -> Bot {
    println!("{}", code);
    Bot::new(parser::parse_bot(code))
}
