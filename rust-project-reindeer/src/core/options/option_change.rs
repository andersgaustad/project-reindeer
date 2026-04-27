use godot::prelude::*;
use strum::EnumIter;


#[derive(Clone, Copy, Debug, EnumIter, Export, GodotConvert, Var)]
#[godot(via = GString)]
#[repr(u8)]
pub enum OptionChange {
    LowPerformanceMode,
    VolumeChange,
    EffectChange,
}
