use godot::prelude::*;


#[repr(u8)]
#[derive(Clone, Copy, Debug, Export, PartialEq, Eq, GodotConvert, Hash, Var)]
#[godot(via = GString)]
pub enum LoadMapMenuRequest {
    Back,
}
