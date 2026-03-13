use godot::{classes::{Camera3D, CharacterBody3D, ICharacterBody3D, Input, InputEvent, InputEventMouseMotion, OmniLight3D, input::MouseMode, light_3d::Param}, global::move_toward, prelude::*};

use crate::input_map::{CAMERA_DOWN, CAMERA_LEFT, CAMERA_RIGHT, CAMERA_UP, MOUSE_LEFT, MOVE_BACK, MOVE_DOWN, MOVE_FORWARD, MOVE_LEFT, MOVE_RIGHT, MOVE_UP, TOGGLE_LIGHT, TOGGLE_SPRINT, TOGGLE_VISIBILITY, UI_CANCEL};


#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct Player {
    #[export]
    #[var]
    #[init(val = 5.0)]
    speed : f32,

    #[export(range=(0.0, 10.0))]
    #[var]
    #[init(val = 3.0)]
    camera_sensitivity_for_controller : f32,


    // Non-exported

    #[var]
    #[init(node = "%Camera3D")]
    camera : OnReady<Gd<Camera3D>>,

    #[var]
    #[init(node = "%Body")]
    body : OnReady<Gd<Node3D>>,

    #[var]
    #[init(node = "%RedLight")]
    red_light : OnReady<Gd<OmniLight3D>>,

    #[var(get, set = set_light_on)]
    #[init(val = false)]
    light_on : bool,

    #[var(get, set = set_body_visible)]
    #[init(val = false)]
    body_visible : bool,

    #[var(get, set = set_body_visible)]
    #[init(val = false)]
    sprint_toggled : bool,

    base : Base<CharacterBody3D>,
}


#[godot_api]
impl ICharacterBody3D for Player {
    fn ready(&mut self) {
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);

        let light_on = self.get_light_on();
        self.set_light_on(light_on);
        
        let body_visible = self.get_body_visible();
        self.set_body_visible(body_visible); 
    }
    

    fn physics_process(&mut self, _delta: f64) {
        let input = Input::singleton();

        // Check camera input (for controllers)
        let camera_vector = input.get_vector(
            CAMERA_LEFT,
            CAMERA_RIGHT,
            CAMERA_UP,
            CAMERA_DOWN,
        );

        // If camera movement, handle rotation
        // Note: Vector is normalized, so multipliying with a sensitivity factor
        if !camera_vector.is_zero_approx() {
            self.handle_camera_vector_movement(camera_vector * self.camera_sensitivity_for_controller);
        }

        // Double speed if sprinting        
        let mut speed = self.speed;
        if self.sprint_toggled {
            speed *= 2.0;
        }


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
        // Check cancel
        let mut input = Input::singleton();
        if event.is_action_pressed(UI_CANCEL) {
            input.set_mouse_mode(MouseMode::VISIBLE);
            return;
        }

        // Else, check mouse click
        if event.is_action_pressed(MOUSE_LEFT) {
            input.set_mouse_mode(MouseMode::CAPTURED);
            return;
        }

        // Else, check mouse motion
        let input_event_mouse_motion_result = event.clone().try_cast::<InputEventMouseMotion>();
        if let Ok(input_event_mouse_motion) = input_event_mouse_motion_result {
            let event_relative = input_event_mouse_motion.get_relative();
            self.handle_camera_vector_movement(event_relative);
            return;
        }

        // Else, check sprint
        if event.is_action_pressed(TOGGLE_SPRINT) {
            let toggled = !self.sprint_toggled;
            self.sprint_toggled = toggled;
            return;
        }

        // Else, check visibility
        if event.is_action_pressed(TOGGLE_VISIBILITY) {
            let toggled = !self.get_body_visible();
            self.set_body_visible(toggled);
            return;
        }

        // Else, check light input
        if event.is_action_pressed(TOGGLE_LIGHT) {
            let toggled = !self.get_light_on();
            self.set_light_on(toggled);
            return;
        }
    }
}


#[godot_api]
impl Player {
    #[func]
    pub fn set_light_on(&mut self, value : bool) {
        self.light_on = value;

        let energy = if value { 10.0 } else { 0.0 };
        self.red_light.set_param(Param::ENERGY, energy);
    }


    #[func]
    pub fn set_body_visible(&mut self, value : bool) {
        self.body_visible = value;

        self.body.set_visible(value);
    }


    fn handle_camera_vector_movement(&mut self, delta_vector : Vector2) {
        let mut rotation_degrees = self.base().get_rotation_degrees();
        rotation_degrees.y -= delta_vector.x * 0.5;

        let mut camera_rotation_degrees = self.camera.get_rotation_degrees();
        camera_rotation_degrees.x -= delta_vector.y * 0.2;
        camera_rotation_degrees.x = camera_rotation_degrees.x.clamp(-60.0, 60.0);


        self.base_mut().set_rotation_degrees(rotation_degrees);
        self.camera.set_rotation_degrees(camera_rotation_degrees);
    }
}
