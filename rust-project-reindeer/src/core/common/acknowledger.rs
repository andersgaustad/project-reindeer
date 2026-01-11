use godot::prelude::*;


#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct Acknowledger {
    base : Base<RefCounted>,
}


#[godot_api]
impl Acknowledger {
    #[signal]
    pub fn ok();
}
