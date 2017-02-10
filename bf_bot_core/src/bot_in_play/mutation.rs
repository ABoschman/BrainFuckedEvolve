#[derive(Debug)]
#[derive(PartialEq)]
pub struct Mutation {
    /// Index of the cell on the tape that is mutated.
    index: usize,
    /// The amount that is added to the value of the cell. This will be between 1 and -1.
    addend: i8,
}

impl Mutation {
    pub fn new(index: usize, addend: i8) -> Mutation {
        Mutation {
            index: index,
            addend: addend,
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_addend(&self) -> i8 {
        self.addend
    }
}
