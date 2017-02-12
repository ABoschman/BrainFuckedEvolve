use bf_bot_core::bf::Instruction;

pub fn parse_bot(code: String) -> Vec<Instruction> {
    // let mut bracket_stack: Vec<usize> = Vec::new();
    let mut vec: Vec<Instruction> = Vec::new();

    for character in code.chars() {
        let instruction: Instruction = match character {
            '<' => Instruction::MoveBack,
            '>' => Instruction::MoveForward,
            '+' => Instruction::Increment,
            '-' => Instruction::Decrement,
            '[' => Instruction::Placeholder,
            ']' => {
                //Look for index of openingbrace.
                let opening_index: Option<usize> = find_matching_opening_brace(&vec);
                match opening_index {
                    Some(value) => {
                        vec[value] = Instruction::WhileNotZeroOpen { target_pointer: vec.len() };
                        Instruction::WhileNotZeroClose { target_pointer: value }
                    }
                    None => panic!("Unmatched square closing bracket."),
                }
            }
            '.' => Instruction::DoNothing,
            _ => Instruction::Comment,
        };
        if instruction != Instruction::Comment {
            vec.push(instruction);
        }
    }

    if (&vec).into_iter().any(|instruction| instruction == &Instruction::Placeholder) {
        panic!("Unmatched square opening bracket.");
    }
    vec
}

fn find_matching_opening_brace(vec: &[Instruction]) -> Option<usize> {
    for (i, instruction) in vec.iter().rev().enumerate() {
        // println!("The item at index:{} is a {:?}", i, instruction);
        if instruction == &Instruction::Placeholder {
            println!("Placeholder {:?} found at index: {} which translates to {}",
                     instruction,
                     i,
                     vec.len() - (i + 1));
            return Some(vec.len() - (i + 1));
        }
    }
    None
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
        let expected: Vec<Instruction> = vec![Instruction::WhileNotZeroOpen { target_pointer: 1 },
                                              Instruction::WhileNotZeroClose { target_pointer: 0 }];
        assert_eq!(&expected, &parse_bot(input));
    }

    #[test]
    fn parseBot_nestedSquareBrackets_returnsNestedWhileNotZeroLoop() {
        let input: String = "[[]]".to_string();
        let expected: Vec<Instruction> = vec![Instruction::WhileNotZeroOpen { target_pointer: 3 },
                                              Instruction::WhileNotZeroOpen { target_pointer: 2 },
                                              Instruction::WhileNotZeroClose { target_pointer: 1 },
                                              Instruction::WhileNotZeroClose { target_pointer: 0 }];
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
