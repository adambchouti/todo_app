use crate::todo::{todo_factory};
use std::env;
use serde_json::{Map, Value};
use crate::processes::process_input;
use crate::state::{read_file};

mod todo;
mod state;
mod processes;

fn main() {
    let file_name = String::from("./state.json");
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];

    let state: Map<String, Value> = read_file(&file_name);

    let status: String;
    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            status = String::from("pending");
        }
    }
    let item = todo_factory(&status, title).expect(&status);
    process_input(item, command.to_string(), &state);
}
