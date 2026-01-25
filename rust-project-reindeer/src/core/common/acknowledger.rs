use godot::prelude::*;


#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct Communicator {
    base : Base<RefCounted>,
}


#[godot_api]
impl Communicator {
    #[signal]
    pub fn done();
}
