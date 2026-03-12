use godot::{classes::{Button, Control, IControl, InputEvent, InputEventKey, InputMap, ScrollContainer, Texture2D, object::ConnectFlags}, obj::WithBaseField, prelude::*};

use crate::{core::ui::{controls_menu::{controls_menu_request::ControlsMenuRequest, controls_menu_state::ControlsMenuState}, i_sub_menu_state::IState}, input_map::{CANCEL, MOVE_BACK, MOVE_DOWN, MOVE_FORWARD, MOVE_LEFT, MOVE_RIGHT, MOVE_UP, TOGGLE_LIGHT, TOGGLE_SPRINT, TOGGLE_VISIBILITY}};


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

    
    #[init(val = ControlsMenuState::Default)]
    state : ControlsMenuState,

    base : Base<Control>
}


#[godot_api]
impl IControl for ControlsMenu {
    fn ready(&mut self) {
        let gd = self.to_gd();

        let event_names_and_buttons = self.get_binding_names_and_rebind_buttons();

        for (event_name, event_button) in event_names_and_buttons.iter() {
            let event_name_gstring = GString::from(*event_name);
            let event_button = event_button.clone();

            event_button
                .clone()
                .signals()
                .toggled()
                .builder()
                .flags(ConnectFlags::DEFERRED)
                .connect_other_gd(
                    &gd,
                    move |mut me, toggled| {
                        let event_name_gstring = event_name_gstring.clone();
                        let event_button = event_button.clone();

                        me
                            .bind_mut()
                            .on_rebind_button_toggled(
                                event_name_gstring,
                                event_button,
                                toggled
                            );
                    }
                );
        }


        // back_button
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


    fn unhandled_input(&mut self, event : Gd<InputEvent>) {
        if !self.base().is_visible_in_tree() {
            return;
        }

        match &self.state {
            ControlsMenuState::Default => {
                // Exit shortcut
                if event.is_action_pressed(CANCEL) {
                    self.on_back_pressed();
                    return;
                }
            },

            ControlsMenuState::WaitingForInput((event_name, event_button)) => {
                let key_input_event_opt = event.clone().try_cast::<InputEventKey>().ok();
                let Some(key_input_event) = key_input_event_opt else {
                    return;
                };

                let mut input_map = InputMap::singleton();
                input_map.action_erase_events(event_name.arg());

                // Ignore cancel - treat as unassigned
                if !event.is_action_pressed(CANCEL) {
                    input_map.action_add_event(event_name.arg(), &key_input_event);
                }

                let mut event_button = event_button.clone();
                event_button.set_pressed(false);

                self.rust_set_state(ControlsMenuState::Default);
            },
        }
    }
}


#[godot_dyn]
impl IState for ControlsMenu {
    fn do_enter(&mut self) {
        self.base_mut().set_process_unhandled_input(true);

        self.rust_set_state(ControlsMenuState::Default);

        self.back_button.grab_focus();

        self
            .scroll_containter
            .set_v_scroll(0);
        
        self.base_mut().show();
    }


    fn do_exit(&mut self) {
        self.base_mut().set_process_unhandled_input(false);
        self.base_mut().hide();
    }
}


#[godot_api]
impl ControlsMenu {
    #[signal]
    pub fn request(request : ControlsMenuRequest);


    fn rust_set_state(&mut self, state : ControlsMenuState) {
        // Set
        let previous_state = std::mem::replace(&mut self.state, state);

        let buttons = self.get_binding_names_and_rebind_buttons();

        match &self.state {
            ControlsMenuState::Default => {
                if let ControlsMenuState::WaitingForInput((_, mut previously_active_button)) = previous_state {
                    previously_active_button.grab_focus();
                }

                for (_, mut button) in buttons {
                    button.set_disabled(false);
                }

                self.refresh();
            },
            ControlsMenuState::WaitingForInput((_, event_button)) => {
                for (_, mut button) in buttons {
                    let is_event_button = &button == event_button;

                    button.set_disabled(!is_event_button);
                    if is_event_button {
                        button.release_focus();
                        button.set_text("Press any button...");
                    }
                }
            },
        }
    }


    #[func]
    fn on_rebind_button_toggled(&mut self, event_name : GString, button : Gd<Button>, toggled : bool) {
        let state = if toggled {
            ControlsMenuState::WaitingForInput((event_name, button))
        } else {
            ControlsMenuState::Default
        };
        
        self.rust_set_state(state);
    }


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

            let key_name = first_associated_input_event.as_text_keycode();
            
            if key_name.is_empty() {
                continue;
            }

            rebind_button.set_text(&key_name);
            rebind_button.set_button_icon(None::<Gd<Texture2D>>.as_ref());
        }
    }


    fn get_binding_names_and_rebind_buttons(&self) -> [(&'static str, Gd<Button>); 9] {
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
