use godot::{classes::{Button, Control, IControl, InputEventKey, InputMap, ScrollContainer, Texture2D}, prelude::*};

use crate::{core::ui::{controls_menu::controls_menu_request::ControlsMenuRequest, i_sub_menu_state::ISubMenuState}, input_map::{MOVE_BACK, MOVE_DOWN, MOVE_FORWARD, MOVE_LEFT, MOVE_RIGHT, MOVE_UP, TOGGLE_LIGHT, TOGGLE_SPRINT, TOGGLE_VISIBILITY}};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct ControlsMenu {
    #[export_group(name = "Button Defaults")]

    #[export]
    #[var]
    #[init(val = "<Unassigned>".into())]
    default_button_text : GString,

    #[export]
    #[var]
    default_button_icon : Option<Gd<Texture2D>>,


    // Non-exported

    #[var]
    #[init(node = "%ScrollContainer")]
    scroll_containter : OnReady<Gd<ScrollContainer>>,


    #[var]
    #[init(node = "%MoveForwardRebind")]
    move_forward_rebind : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%MoveLeftRebind")]
    move_left_rebind : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%MoveBackRebind")]
    move_back_rebind : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%MoveRightRebind")]
    move_right_rebind : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ToggleSprintRebind")]
    toggle_sprint_rebind : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%MoveUpRebind")]
    move_up_rebind : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%MoveDownRebind")]
    move_down_rebind : OnReady<Gd<Button>>,


    #[var]
    #[init(node = "%ShowBodyRebind")]
    show_body_rebind : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ToggleLightRebind")]
    toggle_light_rebind : OnReady<Gd<Button>>,


    #[var]
    #[init(node = "%BackButton")]
    back_button : OnReady<Gd<Button>>,


    base : Base<Control>
}


#[godot_api]
impl IControl for ControlsMenu {
    fn ready(&mut self) {   
        self
            .back_button
            .signals()
            .pressed()
            .connect_other(
                self,  
                Self::on_back_pressed
            );

        self.refresh();
    }
}


#[godot_dyn]
impl ISubMenuState for ControlsMenu {
    fn enter(&mut self) {
        self.back_button.grab_focus();

        self
            .scroll_containter
            .set_v_scroll(0);
    }


    fn reset(&mut self) {

    }
}


#[godot_api]
impl ControlsMenu {
    #[signal]
    pub fn request(request : ControlsMenuRequest);


    #[func]
    fn on_back_pressed(&mut self) {
        self
            .signals()
            .request()
            .emit(ControlsMenuRequest::Back);
    }


    fn refresh(&mut self) {
        self.update_bindings_from_input_map();
    }


    fn update_bindings_from_input_map(&mut self) {
        let names_and_rebinds = self.get_binding_names_and_rebind_buttons();
        let mut input_map = InputMap::singleton();

        for (name, rebind_button) in names_and_rebinds.iter() {
            let mut rebind_button = rebind_button.clone();
            rebind_button.set_text(&self.default_button_text);
            rebind_button.set_button_icon(self.default_button_icon.as_ref());

            let input_events = input_map.action_get_events(*name);
            let first_associated_input_event_opt = input_events
                .iter_shared()
                .find_map(|input_event| {
                    input_event.clone().try_cast::<InputEventKey>().ok()
                });

            let Some(first_associated_input_event) = first_associated_input_event_opt else {
                continue;
            };

            let key_name = first_associated_input_event.as_text_physical_keycode();
            
            if key_name.is_empty() {
                continue;
            }

            rebind_button.set_text(&key_name);
            rebind_button.set_button_icon(None::<Gd<Texture2D>>.as_ref());
        }
    }


    fn get_binding_names_and_rebind_buttons(&self) -> [(&str, Gd<Button>); 9] {
        let names_and_rebinds = [
            // Movement
            (MOVE_FORWARD, self.move_forward_rebind.clone()),
            (MOVE_LEFT, self.move_left_rebind.clone()),
            (MOVE_BACK, self.move_back_rebind.clone()),
            (MOVE_RIGHT, self.move_right_rebind.clone()),
            (TOGGLE_SPRINT, self.toggle_sprint_rebind.clone()),
            (MOVE_UP, self.move_up_rebind.clone()),
            (MOVE_DOWN, self.move_down_rebind.clone()),

            // Misc
            (TOGGLE_VISIBILITY, self.show_body_rebind.clone()),
            (TOGGLE_LIGHT, self.toggle_light_rebind.clone()),
        ];

        names_and_rebinds
    }
}
