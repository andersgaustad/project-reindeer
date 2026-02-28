use godot::prelude::*;
use strum::{EnumCount, VariantArray};


#[derive(Clone, Copy, Debug, EnumCount, Export, PartialEq, Eq, GodotConvert, Var, VariantArray)]
#[godot(via = GString)]
pub enum MainMenuState {
    Title,
    LoadMap    
}
