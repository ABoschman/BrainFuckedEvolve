use bf_bot_core::bf::Instruction;

pub fn parse_bot(code: String) -> Vec<Instruction> {
    let mut bracket_stack: Vec<usize> = Vec::new();
    let mut vec: Vec<Instruction> = Vec::new();
    for character in code.chars() {
        //todo: change into scan with stack as mutable state?

        let instruction: Instruction = match character {
            '<' => Instruction::MoveBack,
            '>' => Instruction::MoveForward,
            '+' => Instruction::Increment,
            '-' => Instruction::Decrement,
            '[' => {
                bracket_stack.push(vec.len());
                start_while_not_zero_placeholder()
            }
            ']' => {
                let opening_index: Option<usize> = bracket_stack.pop();
                match opening_index {
                    Some(value) => {
                        assert_eq!(vec[value], start_while_not_zero_placeholder());
                        vec[value] = Instruction::StartWhileNotZero { target_pointer: vec.len() };
                        Instruction::EndWhileNotZero { target_pointer: value }
                    }
                    None => panic!("Unmatched square closing bracket."),
                }
            }
            '(' => Instruction::StartFor { target_pointer: 0 },
            ')' => {
                Instruction::EndFor {
                    target_pointer: 0,
                    nr_iterations: 1,
                }
            }
            '.' => Instruction::DoNothing,
            _ => Instruction::Comment,
        };
        if instruction != Instruction::Comment {
            vec.push(instruction);
        }
    }
    if !bracket_stack.is_empty() {
        panic!("Unmatched square opening bracket.");
    }
    vec
}

fn start_while_not_zero_placeholder() -> Instruction {
    Instruction::StartWhileNotZero { target_pointer: usize::max_value() }
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
    fn parseBot_dot_shouldReturnDoNothing() {
        let input: String = ".".to_string();
        let expected: Vec<Instruction> = vec![Instruction::DoNothing];
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

}
