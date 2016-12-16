use bot::Bot;
use bot::Instruction;

/// Represents a Bot during one specific game. This struct holds variables whose lifetime do not exceed that of a single game.
#[derive(Debug)]
pub struct BotInPlay<'a> {
    /// A reference to the Bot itself.
    bot: &'a Bot,
    /// Position of the bot on the tape.
    pos: i32,
    /// The index of the instruction that was last executed. 
    /// This value always starts at 0 and is incremented at the end of each round. 
    code_pointer: usize,
    /// The starting position of the bot during this game. 
    /// This doesn't just determine the initial value of the bot's position; it is also used to determine what the MoveBack and MoveForward instructions mean. 
    /// The bot that starts at the end of the tap will decrement its position on the tape upon MoveForward. 
    /// The other bot will increment its position when executing that instruction.
    starting_pos: StartingPos,
    /// The polarity of the bot during this game. 
    polarity: Polarity,
}

impl<'a> BotInPlay<'a> {

    pub fn new(bot: &Bot, length: i32, starting_pos: StartingPos, polarity: Polarity) -> BotInPlay {
        BotInPlay {
            bot: bot,
            pos: if starting_pos == StartingPos::Start { 0 } else { length - 1 },
            code_pointer: 0,
            starting_pos: starting_pos,
            polarity: polarity,
        }
    }

    /// Returns the current position of the bot as a usize. It is not allowed to call this method if the bot is not currently on the tape.
    pub fn get_pos(&self) -> usize {
        self.pos as usize
    }

    pub fn program_has_ended(&self) -> bool {
         self.code_pointer >= self.bot.get_program().len()
    }

    pub fn execute_code(&mut self, current_cell_is_zero: bool) -> Option<Mutation> {
        match self.bot.get_program()[self.code_pointer] {
            Instruction::MoveBack => {self.pos += direction_for_orientation(&self.starting_pos, -1); None},
            Instruction::MoveForward => {self.pos += direction_for_orientation(&self.starting_pos, 1); None},
            Instruction::Increment => Some(Mutation { index: self.pos as usize, addend: cell_mutation_for_polarity(&self.polarity, 1) }),
            Instruction::Decrement => Some(Mutation { index: self.pos as usize, addend: cell_mutation_for_polarity(&self.polarity, -1) }),
            Instruction::ConditionalGoToForward{target_pointer} => {
                if current_cell_is_zero {
                    self.code_pointer = target_pointer;
                }
                None
            },
            Instruction::ConditionalGoToBack{target_pointer} => {
                if !current_cell_is_zero {
                    self.code_pointer = target_pointer;
                }
                None
            },
            _ => None,
        }
    }

    pub fn increment_code_pointer(&mut self) {
        self.code_pointer += 1;
    }

    pub fn bot_is_off_tape(&self, tape_length: i32) -> bool {
        self.pos < 0 || self.pos >= tape_length
    }

    pub fn get_bot(&self) -> &Bot {
        self.bot
    }
}

/// Which side of the tape the bot starts at. 
/// This doesn't affect the gameplay from the bot's perspective, each bot may write their code as though they start at cell zero. 
#[derive(Debug)]
#[derive(PartialEq)]
pub enum StartingPos {
    Start, End,
}

/// In half the matches, one of the bots will have its polarity reversed. 
/// This eliminates the strategy of taking a successful bot's code and merely exchanging + for - and vice versa.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Polarity {
    /// Original polarity, aka Sieve. Decrement is interpreted as lowering the value of the cell, Increment as increasing it.
    Normal, 
    /// Reversed polarity, aka Kettle. Decrement is interpreted as increasing the value of a cell, Increment as lowering it.
    Reversed,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Mutation {
    /// Index of the cell on the tape that is mutated.
    index: usize,
    /// The amount that is added to the value of the cell. This will be between 1 and -1.
    addend: i8,
}

impl Mutation {
    pub fn get_index(&self) -> usize {
        self.index
    }
    
    pub fn get_addend(&self) -> i8 {
        self.addend
    }
}

/// Takes a bot's orientation (i.e. its starting position) and the direction it wants to move in from the bot's perspective 
/// (i.e. forward towards the enemy or back towards its own flag) and converts this into the direction from the perspective of the tape. 
/// For the bot that started out at the start of the tape, the result will be the same. For the other bot, the result will be inverted.
fn direction_for_orientation(orientation: &StartingPos, direction: i32) -> i32 {
    if orientation == &StartingPos::Start { direction } else { -direction }
}

fn cell_mutation_for_polarity(polarity: &Polarity, addend: i8) -> i8 {
    if polarity == &Polarity::Normal { addend } else { -addend }
}

