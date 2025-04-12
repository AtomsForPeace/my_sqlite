use std::{
    io::{Write, stdin, stdout},
    process::exit,
};

pub fn main() {
    loop {
        let text = read_prompt();
        match text.as_str() {
            ".exit" => exit(0),
            t => {
                println!("Unrecognized command: {}", t);
                exit(1)
            }
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
