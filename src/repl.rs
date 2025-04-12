use std::{
    io::{Write, stdin, stdout},
    process::exit,
};

enum StatementType {
    Insert,
    Select,
}

enum ParserError {
    PrepareUnrecognizedStatement,
}

struct Statement {
    statement_type: StatementType,
}

pub fn main() {
    loop {
        let text = read_prompt();
        match text.as_str() {
            t if t.starts_with(".") => {
                parse_meta_command(t);
            }
            t => match prepare_statement(t) {
                Ok(_) => {}
                Err(_) => {}
            },
        }
    }
}

fn parse_meta_command(command_string: &str) {
    match command_string {
        "exit" => exit(0),
        t => {
            println!("Unrecognized command: {}", t);
        }
    }
}

fn prepare_statement(statement_string: &str) -> Result<Statement, ParserError> {
    match statement_string {
        "select" => {
            println!("We do a select here");
            return Ok(Statement {
                statement_type: StatementType::Select,
            });
        }
        "insert" => {
            println!("We do a insert here");
            return Ok(Statement {
                statement_type: StatementType::Insert,
            });
        }
        t => {
            println!("Unrecognized keyword at start of: {}", t);
            return Err(ParserError::PrepareUnrecognizedStatement);
        }
    }
}

fn read_prompt() -> String {
    let mut s = String::new();
    print!("db > ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    return s;
}
