use crate::{
    constant::{EMAIL_SIZE, USERNAME_SIZE},
    row::Row,
    utils::pad_or_truncate,
};

pub enum StatementType {
    Insert(Row),
    Select,
}

pub enum ParserError {
    PrepareUnrecognizedStatement,
}

pub struct Statement {
    statement_type: StatementType,
}

pub fn prepare_statement(statement_string: &str) -> Result<Statement, ParserError> {
    match statement_string {
        "select" => {
            println!("We do a select here");
            return Ok(Statement {
                statement_type: StatementType::Select,
            });
        }
        t if t.starts_with("insert") => {
            println!("We do a insert here");

            let tokens: Vec<&str> = statement_string.trim().split_whitespace().collect();
            if tokens.len() == 4 && tokens[0] == "insert" {
                let id: u32 = tokens[1].parse().unwrap();
                let username: [u8; USERNAME_SIZE] = pad_or_truncate(tokens[2].as_bytes(), true);
                let email: [u8; EMAIL_SIZE] = pad_or_truncate(tokens[3].as_bytes(), true);

                return Ok(Statement {
                    statement_type: StatementType::Insert(Row {
                        id,
                        username,
                        email,
                    }),
                });
            } else {
                panic!("Syntax error. Usage: insert <id> <username> <email>");
            }
        }
        t => {
            println!("Unrecognized keyword at start of: {}", t);
            return Err(ParserError::PrepareUnrecognizedStatement);
        }
    }
}
