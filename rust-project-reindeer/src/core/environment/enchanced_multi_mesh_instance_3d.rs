use godot::{classes::{MultiMeshInstance3D, RandomNumberGenerator}, prelude::*};


#[derive(GodotClass)]
#[class(init, base=MultiMeshInstance3D)]
pub struct EnchancedMultiMeshInstance3D {
    #[export]
    #[init(val = Vector3::ONE)]
    mesh_scale : Vector3,

    #[export]
    mesh_rotation_degrees : Vector3,

    #[export_group(name = "Random Rotation")]
    #[export]
    #[init(val = false)]
    pitch_random_rotation : bool,

    #[export]
    #[init(val = false)]
    yaw_random_rotation : bool,

    #[export]
    #[init(val = false)]
    roll_random_rotation : bool,

    base : Base<MultiMeshInstance3D>
}


#[godot_api]
impl EnchancedMultiMeshInstance3D {
    #[func]
    pub fn create_object_transform(
        &self,
        position : Vector3,
        mut rng : Gd<RandomNumberGenerator>

    ) -> Transform3D {
        let axis_and_rotation_radians = [
            (Vector3::RIGHT, self.pitch_random_rotation),
            (Vector3::UP, self.yaw_random_rotation),
            (Vector3::BACK, self.roll_random_rotation)

        ].map(|(axis, rotate_random)| {
            let rotation_radians = if rotate_random {
                rng.randf_range(0.0, std::f32::consts::TAU)

            } else {
                0.0
            };

            (axis, rotation_radians)
        });

        let mut basis = Basis::default();
        for (axis, rotation_radians) in axis_and_rotation_radians {
            basis = basis.rotated(axis, rotation_radians);
        }
        let basis = basis.scaled(self.mesh_scale);

        let transform = Transform3D::new(basis, position);

        transform
    }   
}
