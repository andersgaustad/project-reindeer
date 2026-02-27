use godot::prelude::*;


#[repr(u8)]
#[derive(Clone, Copy, Debug, Export, PartialEq, Eq, GodotConvert, Hash, Var)]
#[godot(via = GString)]
pub enum LevelRunState {
    Running,
    Paused,   
}


impl LevelRunState {
    pub fn is_paused(&self) -> bool {
        // Matching might seem like overkill, but handy to get a compile error/warning if I for some reason add more enum states
        match self {
            LevelRunState::Running => false,
            LevelRunState::Paused => true,
        }
    }
}
