use std::io::{self, Write};
use std::iter::Iterator;
use std::process;

const ID_SIZE: usize = 4;
const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;
const ID_OFFSET: usize = 0;
const USER_NAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USER_NAME_OFFSET + COLUMN_USERNAME_SIZE;
const ROW_SIZE: usize = ID_SIZE + COLUMN_USERNAME_SIZE + COLUMN_EMAIL_SIZE;
const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

enum ExecuteResult {
    ExecuteSucces,
    ExecuteTableFull,
}

enum StatementType {
    StatementInsert,
    StatementSelect,
    StatementNone,
}

enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

#[derive(Debug)]
enum PrepareResult {
    PrepareSuccess,
    PrepareSyntaxError,
    PrepareUnrecognizedStatement,
}

struct Row {
    id: u32,
    username: String,
    email: String,
}

impl Row {
    fn new() -> Self {
        Self {
            id: 0,
            username: String::with_capacity(COLUMN_USERNAME_SIZE),
            email: String::with_capacity(COLUMN_EMAIL_SIZE),
        }
    }
}

struct Table {
    num_rows: usize,
    pages: [Option<Vec<u8>>; TABLE_MAX_PAGES],
}

impl Default for Table {
    fn default() -> Self {
        Table {
            num_rows: 0,
            pages: [(); TABLE_MAX_PAGES].map(|_| None),
        }
    }
}

impl Table {
    fn new() -> Self {
        Self::default()
    }
}

struct Statement {
    statement_type: StatementType,
    row_to_insert: Row,
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
            process::exit(1);
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

fn prepare_statement(input_buffer: &mut InputBuffer, statement: &mut Statement) -> PrepareResult {
    if input_buffer.input_length >= 6 && &input_buffer.buffer[..6] == "insert" {
        statement.statement_type = StatementType::StatementInsert;
        PrepareResult::PrepareSuccess
    } else if input_buffer.buffer == "select" {
        statement.statement_type = StatementType::StatementSelect;
        PrepareResult::PrepareSuccess
    } else {
        PrepareResult::PrepareUnrecognizedStatement
    }
}

fn execute_statement(statement: &mut Statement) {
    match statement.statement_type {
        StatementType::StatementInsert => println!("Insert statement would be executed"),
        StatementType::StatementSelect => println!("Select statement would be executed"),
        StatementType::StatementNone => println!("No statement would be executed"),
    }
}

fn main() {
    let mut table = Table::new();
    let mut input_buffer = InputBuffer::new();

    loop {
        print_prompt();
        read_input(&mut input_buffer);

        if input_buffer.buffer.chars().nth(0).unwrap() == '.' {
            match do_meta_command(&mut input_buffer) {
                MetaCommandResult::MetaCommandSuccess => continue,
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized meta command: {}", input_buffer.buffer)
                }
            }
        } else {
            let mut statement = Statement {
                statement_type: StatementType::StatementNone,
                row_to_insert: Row::new(),
            };
            match prepare_statement(&mut input_buffer, &mut statement) {
                PrepareResult::PrepareSuccess => break,
                PrepareResult::PrepareSyntaxError => {
                    println!("Syntax error. Could not parse statement.");
                    continue
                },
                PrepareResult::PrepareUnrecognizedStatement => {
                    println!("Unrecognized statement at the start: {}", input_buffer.buffer);
                    continue
                },
            }
        }
    }
}
