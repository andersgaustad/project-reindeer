use godot::prelude::*;
use strum::{EnumCount, EnumIter, VariantArray};


#[derive(Clone, Copy, Debug, EnumCount, EnumIter, Export, GodotConvert, Var, VariantArray)]
#[godot(via = GString)]
#[repr(u8)]
pub enum SFXEntry {
    Click,
    Error,
}
