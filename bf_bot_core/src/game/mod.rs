//! Brainfuck Joust
//!
//! Rather than running a single round (which would have a random tape length from 10 to 30),
//! tournaments generally run 42 rounds (or "jousts") between the two warriors, in order to make
//! the results deterministic.
//! Two things are varied: the tape has one of 21 different lengths (the integers from 10 to 30
//! inclusive); and one of the warriors may have its polarity exchanged, i.e., exchanging the
//! meaning of + and -.
//!
//! The terminology used by this program is as follows: A complete game consists of 42 rounds. Each
//! round consists of steps. At each step, both bots execute one instruction in their program.
//! For performance reasons, it is possible to run an incomplete game, consisting of fewer than 42
//! rounds. An incomplete game gives non-deterministic results.

pub use self::game::{run, run_complete};
mod game;

pub mod game_result;

mod all_rounds;
