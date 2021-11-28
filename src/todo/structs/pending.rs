use crate::todo::structs::traits::create::Create;
use crate::todo::structs::traits::delete::Delete;
use crate::todo::structs::traits::edit::Edit;
use crate::todo::structs::traits::get::Get;
use super::base::Base;

pub struct Pending {
    pub super_struct: Base
}

impl Pending {
    pub fn new(input_title: &str) -> Pending {
        let base: Base = Base::new(input_title, "pending");
        return Pending{super_struct: base}
    }
}

impl Create for Pending {}
impl Edit for Pending {}
impl Get for Pending {}
impl Delete for Pending {}