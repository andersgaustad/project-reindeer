use godot::prelude::*;


#[derive(Clone, Copy, Debug, Export, GodotConvert, Var)]
#[godot(via = GString)]
#[repr(u8)]
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


impl MazeTileState {
    pub const fn to_godot_flag(&self) -> u32 {
        1 << *self as u32
    }


    pub fn is_set_flag_in_bits(&self, bits : u32) -> bool {
        let masked = self.to_godot_flag() & bits;
        masked != 0
    }
}
