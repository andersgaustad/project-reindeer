use godot::prelude::*;
use strum::EnumIter;


#[repr(u8)]
#[derive(Clone, Copy, Debug, EnumIter, Export, PartialEq, Eq, GodotConvert, Hash,  Var)]
#[godot(via = GString)]
pub enum AboutMenuIconButtonType {
    Godot,
    GodotRust,
    Rust,
}
