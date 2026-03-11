use godot::prelude::*;
use strum::EnumIter;


#[repr(u8)]
#[derive(Clone, Copy, Debug, EnumIter, Export, PartialEq, Eq, GodotConvert, Hash,  Var)]
#[godot(via = GString)]
pub enum PauseMenuButtonType {
    Start,
    Resume,
    Mail,
    Options,
    Controls,
    MainMenu,
    Exit,    
}
