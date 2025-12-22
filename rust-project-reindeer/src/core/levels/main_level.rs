use godot::{classes::IStaticBody3D, prelude::*};


#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
pub struct MainLevel {
    #[export]
    #[var]
    dim_x : i32,

    #[export]
    #[var]
    dim_y : i32,
}


#[godot_api]
impl IStaticBody3D for MainLevel {

}


#[godot_api]
impl MainLevel {
    #[func]
    pub fn update_with_dimensions(&mut self) {
        

    }

}