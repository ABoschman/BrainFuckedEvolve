//! Brainfuck Joust
//!
//! Rather than running a single round (which would have a random tape length from 10 to 30), tournaments generally run 42 rounds (or "jousts") between the two warriors, in order to make the results deterministic. 
//! Two things are varied: the tape has one of 21 different lengths (the integers from 10 to 30 inclusive); and one of the warriors may have its polarity exchanged, i.e., exchanging the meaning of + and -.
//!
//! The terminology used by this program is as follows: A complete match consists of 42 rounds. Each round consists of steps. At each step, both bots execute one command in their program. 
//! For performance reasons, it is possible to run an incomplete match, consisting of fewer than 42 rounds. An incomplete match gives non-deterministic results. 

/// Compares two bots in a (complete) match. 
pub fn run_match(bot_a: &Bot, bot_b: &Bot) -> MatchResult {
    //NB: This has to be functional, no side effects. That way you can run match simulations in parallel.
    MatchResult { }
}

//TODO: Some function that allows the simulation of an incomplete match. When determining the fitness of a bot, running complete matches is probably too inefficient.

pub struct MatchResult {
    //TODO:...
    // A minimal implementation is just the score of each bot, perhaps a float in the range [0,1] where the score is the ratio of rounds won. Or maybe just two ints, for nr games won.
}

struct Match {
    //TODO:...
}

