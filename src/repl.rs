use crate::{
    constant::{PAGE_SIZE, ROW_SIZE, ROWS_PER_PAGE},
    statement::prepare_statement,
    table::Table,
};
use std::{
    io::{Write, stdin, stdout},
    process::exit,
};

pub fn row_slot(table: &mut Table, row_num: usize) -> &mut [u8] {
    let page_num = row_num / ROWS_PER_PAGE;
    let row_offset = row_num % ROWS_PER_PAGE;
    let byte_offset = row_offset * ROW_SIZE;

    // Get or initialize the page if it doesn't exist
    if table.pages[page_num].is_none() {
        table.pages[page_num] = Some(Box::new([0; PAGE_SIZE]));
    }

    // Return a mutable reference to the row's slot
    &mut table.pages[page_num].as_mut().unwrap()[byte_offset..byte_offset + ROW_SIZE]
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
