//! Takes BrainFuck bot script as string or as text file.
//! Checks script for validity.
//! Error if mismatching brackets, warning if possible accidental use of full-stop as comment. Also warn if use comma, it's a reserved character in original BF.
//! Minifier that reduces code complexity without changing the functionality.
//! Prettyfier that tries to format the code in an intellinent way using spacing and newlines.
extern crate bf_bot_core;


pub mod parser;