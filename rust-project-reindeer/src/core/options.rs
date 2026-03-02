use godot::prelude::*;


#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Options {
    #[export]
    #[var]
    #[init(val = false)]
    low_performance_mode : bool,

    base : Base<Resource>
}
