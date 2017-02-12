use bf::{Bot, Instruction};
use engine::{Mutation, Orientation, Polarity};

/// Represents a Bot during one specific game. This struct holds variables whose lifetime do not
/// exceed that of a single game.
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
    /// This doesn't just determine the initial value of the bot's position; it is also used to
    /// determine what the MoveBack and MoveForward instructions mean.
    /// The bot that starts at the end of the tap will decrement its position on the tape upon
    /// MoveForward. The other bot will increment its position when executing that instruction.
    orientation: Orientation,
    /// The polarity of the bot during this game.
    polarity: Polarity,
}

impl<'a> BotInPlay<'a> {
    pub fn new(bot: &Bot, length: i32, orientation: Orientation, polarity: Polarity) -> BotInPlay {
        BotInPlay {
            bot: bot,
            pos: if orientation == Orientation::Normal {
                0
            } else {
                length - 1
            },
            code_pointer: 0,
            orientation: orientation,
            polarity: polarity,
        }
    }

    /// Returns the current position of the bot as a usize. It is not allowed to call this method
    /// if the bot is not currently on the tape.
    pub fn get_pos(&self) -> usize {
        self.pos as usize
    }

    pub fn program_has_ended(&self) -> bool {
        self.code_pointer >= self.bot.get_program().len()
    }

    pub fn execute_code(&mut self, current_cell_is_zero: bool) -> Option<Mutation> {
        match self.bot.get_program()[self.code_pointer] {
            Instruction::MoveBack => {
                self.pos += self.orientation.calc_movement_relative_to_tape(-1);
                None
            }
            Instruction::MoveForward => {
                self.pos += self.orientation.calc_movement_relative_to_tape(1);
                None
            }
            Instruction::Increment => {
                Some(Mutation::new(self.pos as usize,
                                   self.polarity.mutation_relative_to_tape(1)))
            }
            Instruction::Decrement => {
                Some(Mutation::new(self.pos as usize,
                                   self.polarity.mutation_relative_to_tape(-1)))
            }
            Instruction::WhileNotZeroOpen { target_pointer } => {
                if current_cell_is_zero {
                    self.code_pointer = target_pointer;
                }
                None
            }
            Instruction::WhileNotZeroClose { target_pointer } => {
                if !current_cell_is_zero {
                    self.code_pointer = target_pointer;
                }
                None
            }
            _ => None,
        }
    }

    pub fn increment_code_pointer(&mut self) {
        self.code_pointer += 1;
    }

    pub fn bot_is_off_tape(&self, tape_length: &i32) -> bool {
        self.pos < 0 || &self.pos >= tape_length
    }

    pub fn get_bot(&self) -> &Bot {
        self.bot
    }
}
