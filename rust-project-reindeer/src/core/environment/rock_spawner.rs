use godot::{classes::{MultiMeshInstance3D, RandomNumberGenerator}, prelude::*};


#[derive(GodotClass)]
#[class(init, base=MultiMeshInstance3D)]
pub struct RockSpawner {
    #[export]
    #[init(val = Vector3::ONE)]
    rock_scale : Vector3,

    base : Base<MultiMeshInstance3D>
}


#[godot_api]
impl RockSpawner {
    #[func]
    pub fn create_rock_transform(
        &self,
        position : Vector3,
        mut rng : Gd<RandomNumberGenerator>

    ) -> Transform3D {
        let rotated_radians = rng.randf_range(0.0, std::f32::consts::TAU);

        let basis = 
            Basis::default()
            .rotated(Vector3::new(0.0, 1.0, 0.0), rotated_radians)
            .scaled(self.rock_scale);

        let transform = Transform3D::new(basis, position);

        transform
    }   
}
