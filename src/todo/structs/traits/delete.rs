use serde_json::{Map, Value};
use crate::state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file(&String::from("./state.json"), state);
        println!("\n\n{} is being deleted\n\n", title);
    }
}