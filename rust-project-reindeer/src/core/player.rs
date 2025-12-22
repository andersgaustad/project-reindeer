use godot::{classes::{Camera3D, CharacterBody3D, ICharacterBody3D, Input, InputEvent, InputEventMouseMotion, input::MouseMode}, global::move_toward, prelude::*};

use crate::input_map::{MOVE_BACK, MOVE_DOWN, MOVE_FORWARD, MOVE_LEFT, MOVE_RIGHT, MOVE_UP};


#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct Player {
    #[export]
    #[var]
    #[init(val = 5.0)]
    speed : f32,

    #[var]
    #[init(node = "%Camera3D")]
    camera : OnReady<Gd<Camera3D>>,

    base : Base<CharacterBody3D>,
}


#[godot_api]
impl ICharacterBody3D for Player {
    fn ready(&mut self) {
        let mut input = Input::singleton();

        input.set_mouse_mode(MouseMode::CAPTURED);
    }
    

    fn physics_process(&mut self, _delta: f64) {
        let input = Input::singleton();
        let speed = self.speed;

        let movement_vector_2d = input.get_vector(
            MOVE_RIGHT,
            MOVE_LEFT,
            MOVE_BACK,
            MOVE_FORWARD,
        );
        let movement_vector_3d = Vector3::new(movement_vector_2d.x, 0.0, movement_vector_2d.y);

        let up_down_movement_axis = input.get_axis(
            MOVE_DOWN,
            MOVE_UP
        );

        let transform = self.base().get_transform();

        let direction_opt = (transform.basis * movement_vector_3d).try_normalized();

        let mut velocity = self.base().get_velocity();

        if let Some(direction) = direction_opt {
            velocity.x = direction.x * speed;
            velocity.z = direction.z * speed;

        } else {
            velocity.x = move_toward(velocity.x.into(), 0.0, speed.into()) as f32;
            velocity.z = move_toward(velocity.z.into(), 0.0, speed.into()) as f32;
        };

        if up_down_movement_axis.is_zero_approx() {
            velocity.y = move_toward(up_down_movement_axis.into(), 0.0, speed.into()) as f32;
            
        } else {
            velocity.y = up_down_movement_axis * speed;
        }

        self.base_mut().set_velocity(velocity);

        self.base_mut().move_and_slide();
    }

    
    fn unhandled_input(&mut self, event : Gd<InputEvent>) {
        // Mouse motion
        let input_event_mouse_motion_result = event.clone().try_cast::<InputEventMouseMotion>();
        if let Ok(input_event_mouse_motion) = input_event_mouse_motion_result {
            let event_relative = input_event_mouse_motion.get_relative();

            let mut rotation_degrees = self.base().get_rotation_degrees();
            rotation_degrees.y -= event_relative.x * 0.5;

            let mut camera_rotation_degrees = self.camera.get_rotation_degrees();
            camera_rotation_degrees.x -= event_relative.y * 0.2;
            camera_rotation_degrees.x = camera_rotation_degrees.x.clamp(-60.0, 60.0);


            self.base_mut().set_rotation_degrees(rotation_degrees);
            self.camera.set_rotation_degrees(camera_rotation_degrees);

            return;
        }

        // Else, check cancel
        if event.is_action_pressed("ui_cancel") {
            let mut input = Input::singleton();

            input.set_mouse_mode(MouseMode::VISIBLE);
        }
    }
}
