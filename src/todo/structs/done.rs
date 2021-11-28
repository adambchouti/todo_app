use crate::todo::structs::traits::delete::Delete;
use crate::todo::structs::traits::edit::Edit;
use crate::todo::structs::traits::get::Get;
use super::base::Base;

pub struct Done {
    pub super_struct: Base
}

impl Done {
    pub fn new(input_title: &str) -> Done {
        let base: Base = Base::new(input_title, "done");
        return Done{super_struct: base}
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}