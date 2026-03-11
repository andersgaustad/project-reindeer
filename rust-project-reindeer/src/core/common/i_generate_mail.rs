use godot::prelude::*;


pub trait IGenerateMail {
    fn generate_mail(&self) -> GString;
}
