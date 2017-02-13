use bf_bot_core::bf::Instruction;

//TODO: Take &str?
pub fn parse_bot<'a>(source_code: String) -> Vec<Instruction> {
    let (bracket_stack, instructions): (Vec<usize>, Vec<Instruction>) = source_code.chars()
        .enumerate()
        .fold((vec![], vec![]),
              |(mut bracket_stack, mut instructions), (index, character)| {
            match character {
                '<' => instructions.push(Instruction::MoveBack),
                '>' => instructions.push(Instruction::MoveForward),
                '+' => instructions.push(Instruction::Increment),
                '-' => instructions.push(Instruction::Decrement),
                '.' => instructions.push(Instruction::SkipExecution),
                '[' => open_square_bracket(&mut bracket_stack, &mut instructions),
                ']' => close_square_bracket(&mut bracket_stack, &mut instructions),
                '(' => open_round_bracket(&mut bracket_stack, &mut instructions),
                ')' => close_round_bracket(&mut bracket_stack, &mut instructions),
                _ => {
                    //Comment character, ignore.
                }
            };
            (bracket_stack, instructions)
        });
    assert!(bracket_stack.is_empty(), "Unmatched opening bracket(s).");
    instructions
}

fn open_square_bracket(bracket_stack: &mut Vec<usize>, instructions: &mut Vec<Instruction>) {
    bracket_stack.push(instructions.len());
    instructions.push(start_while_not_zero_placeholder());
}

fn close_square_bracket(bracket_stack: &mut Vec<usize>, instructions: &mut Vec<Instruction>) {
    let opening_index: Option<usize> = bracket_stack.pop();
    match opening_index {
        Some(value) => {
            assert_eq!(instructions[value], start_while_not_zero_placeholder());
            instructions[value] =
                Instruction::StartWhileNotZero { target_pointer: instructions.len() };
            instructions.push(Instruction::EndWhileNotZero { target_pointer: value });
        }
        None => panic!("Unmatched square closing bracket."),
    }
}

fn open_round_bracket(bracket_stack: &mut Vec<usize>, instructions: &mut Vec<Instruction>) {
    bracket_stack.push(instructions.len());
    instructions.push(start_for_placeholder());
}

fn close_round_bracket(bracket_stack: &mut Vec<usize>, instructions: &mut Vec<Instruction>) {
    let opening_index: Option<usize> = bracket_stack.pop();
    match opening_index {
        Some(value) => {
            assert_eq!(instructions[value], start_for_placeholder());
            instructions[value] = Instruction::StartFor { target_pointer: instructions.len() };
            instructions.push(Instruction::EndFor {
                target_pointer: value,
                nr_iterations: 1,
            });
        }
        None => panic!("Unmatched square closing bracket."),
    }
}

fn start_while_not_zero_placeholder() -> Instruction {
    Instruction::StartWhileNotZero { target_pointer: <usize>::max_value() }
}

fn start_for_placeholder() -> Instruction {
    Instruction::StartFor { target_pointer: <usize>::max_value() }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use bf_bot_core::bf::Instruction;

    #[test]
    fn parseBot_emptyBotCode_shouldReturnEmptyVec() {
        let input: String = "".to_string();
        let expected: Vec<Instruction> = vec![];
        assert_eq!(&expected, &parse_bot(input));
    }

    #[test]
    fn parseBot_dot_shouldReturnSkipExecution() {
        let input: String = ".".to_string();
        let expected: Vec<Instruction> = vec![Instruction::SkipExecution];
        assert_eq!(&expected, &parse_bot(input));
    }

    /// Note that this test is not exhaustive, the program should ignore all ascii
    /// characters that aren't part of the BrainFuck dialect used by BF Joust.
    #[test]
    fn parseBot_arbitraryCommentCharacters_shouldReturnEmptyVec() {
        let input: String = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
            .to_string();
        let expected: Vec<Instruction> = vec![];
        assert_eq!(&expected, &parse_bot(input));
    }

    /// In Brainfuck, the comma character is a valid instruction. It accepts a byte of input and
    /// stores it at the pointer. However, the BrainFuck dialect used by these bots does not
    /// recognise the comma and interprets it as a comment instead.
    #[test]
    fn parseBot_comma_shouldReturnEmptyVec() {
        let input: String = ",".to_string();
        let expected: Vec<Instruction> = vec![];
        assert_eq!(&expected, &parse_bot(input));
    }

    #[test]
    fn parseBot_smallerThanSign_shouldReturnMoveBack() {
        let input: String = "<".to_string();
        let expected: Vec<Instruction> = vec![Instruction::MoveBack];
        assert_eq!(&expected, &parse_bot(input));
    }

    #[test]
    fn parseBot_greaterThanSign_shouldReturnMoveForward() {
        let input: String = ">".to_string();
        let expected: Vec<Instruction> = vec![Instruction::MoveForward];
        assert_eq!(&expected, &parse_bot(input));
    }

    #[test]
    fn parseBot_minusSign_shouldReturnDecrement() {
        let input: String = "-".to_string();
        let expected: Vec<Instruction> = vec![Instruction::Decrement];
        assert_eq!(&expected, &parse_bot(input));
    }

    #[test]
    fn parseBot_plusSign_shouldReturnIncrement() {
        let input: String = "+".to_string();
        let expected: Vec<Instruction> = vec![Instruction::Increment];
        assert_eq!(&expected, &parse_bot(input));
    }

    #[test]
    #[should_panic]
    fn parseBot_unmatchedSquareOpeningBracket_shouldPanic() {
        let input: String = "[".to_string();
        parse_bot(input);
    }

    #[test]
    #[should_panic]
    fn parseBot_unmatchedSquareClosingBracket_shouldPanic() {
        let input: String = "]".to_string();
        parse_bot(input);
    }

    #[test]
    fn parseBot_squareBrackets_returnsWhileNotZeroLoop() {
        let input: String = "[]".to_string();
        let expected: Vec<Instruction> = vec![Instruction::StartWhileNotZero { target_pointer: 1 },
                                              Instruction::EndWhileNotZero { target_pointer: 0 }];
        assert_eq!(&expected, &parse_bot(input));
    }

    #[test]
    fn parseBot_nestedSquareBrackets_returnsNestedWhileNotZeroLoop() {
        let input: String = "[[]]".to_string();
        let expected: Vec<Instruction> = vec![Instruction::StartWhileNotZero { target_pointer: 3 },
                                              Instruction::StartWhileNotZero { target_pointer: 2 },
                                              Instruction::EndWhileNotZero { target_pointer: 1 },
                                              Instruction::EndWhileNotZero { target_pointer: 0 }];
        assert_eq!(&expected, &parse_bot(input));
    }

    #[test]
    #[should_panic]
    fn parseBot_unmatchedNestedSquareOpeningBracket_shouldPanic() {
        let input: String = "[[]".to_string();
        parse_bot(input);
    }

    #[test]
    #[should_panic]
    fn parseBot_unmatchedNestedSquareClosingBracket_shouldPanic() {
        let input: String = "[]]".to_string();
        parse_bot(input);
    }

    #[test]
    #[should_panic]
    fn parseBot_unmatchedRoundOpeningBracket_shouldPanic() {
        let input: String = "(".to_string();
        parse_bot(input);
    }

    #[test]
    #[should_panic]
    fn parseBot_unmatchedRoundClosingBracket_shouldPanic() {
        let input: String = ")".to_string();
        parse_bot(input);
    }

    #[test]
    #[should_panic]
    fn parseBot_wrongTypeOfBracketRoundSquare_shouldPanic() {
        let input: String = "(]".to_string();
        parse_bot(input);
    }

    #[test]
    #[should_panic]
    fn parseBot_wrongTypeOfBracketSquareRound_shouldPanic() {
        let input: String = "[)".to_string();
        parse_bot(input);
    }

    #[test]
    fn parseBot_roundBrackets_returnsForLoop() {
        let input: String = "()*1".to_string();
        let expected: Vec<Instruction> = vec![Instruction::StartFor { target_pointer: 1 },
                                              Instruction::EndFor {
                                                  target_pointer: 0,
                                                  nr_iterations: 1,
                                              }];
        assert_eq!(&expected, &parse_bot(input));
    }

    #[test]
    #[ignore]
    fn parseBot_roundBracketsDifferentNumberIterations_returnsForLoop() {
        let input: String = "()*2".to_string();
        let expected: Vec<Instruction> = vec![Instruction::StartFor { target_pointer: 1 },
                                              Instruction::EndFor {
                                                  target_pointer: 0,
                                                  nr_iterations: 2,
                                              }];
        assert_eq!(&expected, &parse_bot(input));
    }

}
