#![allow(dead_code, unused_variables)]

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Severity {
    WARNING,
    ERROR
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct CodeLocation {
    /// Starts at line 1.
    line: u32,
    /// Starts at column 1.
    column: u32
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Issue {
    severity: Severity,
    code_location: CodeLocation,
    description: String
}

impl Issue {
    fn new_unintentional_dot(code_location: CodeLocation) -> Issue {
        Issue {
            severity: Severity::WARNING,
            code_location: code_location,
            description: "Possible unintentional dot. It looks like you accidentally used a dot as part of your documentation. A dot or full-stop (.) is a command in the BrainFuck Jousting language. It tells the bot to do nothing that turn.".to_string()
        }
    }

    pub fn to_string(&self) -> String {
        let severity_str: &str = match self.severity {
            Severity::WARNING => "WARNING",
            Severity::ERROR => "ERROR"
        };
        format!("{} on line {}, column {} {}", severity_str, self.code_location.line, self.code_location.column, self.description)
    }
}

pub fn lint_check(program: &str) -> Vec<Issue> {
    vec![]
}

fn check_bracket_mismatch(program: &str) -> Vec<Issue> {
    vec![]
}

/// Checks for dots that were probably intended as part of the bot's documentation. 
/// This lint rule checks for a few simple patterns:
///
/// - A dot that immediatelly follows a non-whitespace comment character.
/// - A dot that follows a body of comments including at least one non-whitespace character and precedes a body of comments followed by a line break or the program's end.
///
fn check_unintended_dot(program: &str) -> Vec<Issue> {
    let vec: Vec<Issue> = vec![];
    for line in program.split("\n") {
        println!("{}", line);
    }
    vec
}

fn check_comma(program: &str) -> Vec<Issue> {
    vec![]
}

#[test]
#[allow(non_snake_case)]
fn checkUnintendedDot_forEmptyProgram_raisesNoWarnings() {
    let input: &str = "";
    let expected: Vec<Issue> = vec![];
    assert_eq!(&expected, &check_unintended_dot(input));
}

#[test]
#[allow(non_snake_case)]
fn checkUnintendedDot_forDotSurroundedByComments_raisesWarning() {
    let input: &str = "a.a";
    let expected: Vec<Issue> = vec![Issue::new_unintentional_dot(CodeLocation{line: 1, column: 2})];
    assert_eq!(&expected, &check_unintended_dot(input));
}