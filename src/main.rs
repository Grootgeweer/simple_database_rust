use std::io::{self, Write};
use std::iter::Iterator;
use std::process;

enum StatementType {
    StatementInsert,
    StatementSelect,
}

enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

struct Statement {
    statement_type: StatementType,
}

#[derive(Debug)]
struct InputBuffer {
    buffer: String,
    input_length: isize,
}

impl InputBuffer {
    fn new() -> Self {
        InputBuffer {
            buffer: String::new(),
            input_length: 0,
        }
    }
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn read_input(input_buffer: &mut InputBuffer) {
    input_buffer.buffer.clear();

    match io::stdin().read_line(&mut input_buffer.buffer) {
        Ok(bytes_read) => {
            input_buffer.input_length = bytes_read as isize - 2;
            input_buffer.buffer.truncate(bytes_read - 2);
        }
        Err(_) => {
            println!("Error reading input");
            std::process::exit(1);
        }
    }
}

fn do_meta_command(input_buffer: &mut InputBuffer) -> MetaCommandResult {
    if input_buffer.buffer == ".exit" {
        println!("Closing down Pipi Database");
        process::exit(0);
    } else {
        MetaCommandResult::MetaCommandUnrecognizedCommand
    }
}

fn prepare_statement(input_buffer: &mut InputBuffer, statement: &mut Statement) -> PrepareResult {}

fn main() {
    let mut input_buffer = InputBuffer::new();

    loop {
        print_prompt();
        read_input(&mut input_buffer);

        if input_buffer.buffer.chars().nth(0).unwrap() == '.' {
            match do_meta_command(&mut input_buffer) {
                MetaCommandResult::MetaCommandSuccess => continue,
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command: {}", input_buffer.buffer)
                }
            }
        } else {
            println!("test");
        }
    }
}
