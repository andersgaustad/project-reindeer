use godot::prelude::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Export, PartialEq, Eq, GodotConvert, Hash, Var)]
#[godot(via = GString)]
pub enum LetterMenuInboxState {
    Empty,
    NewMail,
    AllRead,    
}


impl Default for LetterMenuInboxState {
    fn default() -> Self {
        Self::Empty
    }
}
