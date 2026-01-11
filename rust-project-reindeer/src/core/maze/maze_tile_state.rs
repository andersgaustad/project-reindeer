use godot::prelude::*;


#[derive(Clone, Debug, Export, GodotConvert, Var)]
#[godot(via = GString)]
pub enum MazeTileState {
    Normal,
    Touched,
    Committed,
    Active,    
}


impl Default for MazeTileState {
    fn default() -> Self {
        Self::Normal
    }
}
