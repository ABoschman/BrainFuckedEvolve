use bf_bot_core::bot::Instruction;

pub fn parse_bot(code: String) -> Vec<Instruction> {
    let mut vec: Vec<Instruction> = Vec::new();
    for character in code.chars() {
        let instruction: Instruction  = match character {
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
                        vec[value] = Instruction::ConditionalGoToForward{target_pointer: vec.len()};
                        Instruction::ConditionalGoToBack{target_pointer: value}
                    },
                    None => panic!("Help!"),
                }
            },
            '.' => Instruction::DoNothing,
            _ => Instruction::Comment,
        };
        if instruction != Instruction::Comment {
            vec.push(instruction);
        }
    }
    vec
}

fn find_matching_opening_brace(vec: &Vec<Instruction>) -> Option<usize> {
    for (i, instruction) in vec.iter().rev().enumerate() {
        // println!("The item at index:{} is a {:?}", i, instruction);
        if instruction == &Instruction::Placeholder {
            println!("Placeholder {:?} found at index: {} which translates to {}", instruction, i, vec.len() - (i + 1));
            return Some(vec.len() - (i + 1));
        }
    }
    None
}

#[test]
#[allow(non_snake_case)]
fn parseBot_emptyBotCode_shouldReturnEmptyVec() {
    let input: String = "".to_string();
    let expected: Vec<Instruction> = vec![];
    assert_eq!(&expected, &parse_bot(input));
}

#[test]
#[allow(non_snake_case)]
fn parseBot_dot_shouldReturnDoNothing() {
    let input: String = ".".to_string();
    let expected: Vec<Instruction> = vec![Instruction::DoNothing];
    assert_eq!(&expected, &parse_bot(input));
}

#[test]
#[allow(non_snake_case)]
fn parseBot_arbitraryCommentCharacter_shouldReturnEmptyVec() {
    let input: String = "a".to_string();
    let expected: Vec<Instruction> = vec![];
    assert_eq!(&expected, &parse_bot(input));
}

#[test]
#[allow(non_snake_case)]
fn parseBot_smallerThanSign_shouldReturnMoveBack() {
    let input: String = "<".to_string();
    let expected: Vec<Instruction> = vec![Instruction::MoveBack];
    assert_eq!(&expected, &parse_bot(input));
}

#[test]
#[allow(non_snake_case)]
fn parseBot_multipleInstructions_shouldReturnLongerVector() {
    let input: String = "<<".to_string();
    let expected: Vec<Instruction> = vec![Instruction::MoveBack,Instruction::MoveBack];
    assert_eq!(&expected, &parse_bot(input));
}

