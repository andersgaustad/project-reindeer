use godot::{classes::{Button, Engine, HBoxContainer, IHBoxContainer, InputEvent, InputEventJoypadButton, InputEventJoypadMotion, InputEventKey, InputMap, Label, Texture2D, object::ConnectFlags}, prelude::*};

use crate::{core::ui::{controls_menu::rebind_control_row_state::RebindControlRowState, i_sub_menu_state::IState}, input_map::UI_CANCEL};


#[derive(GodotClass)]
#[class(init, tool, base=HBoxContainer)]
pub struct RebindControlRow {
    #[export]
    #[var(get, set = set_label_name)]
    #[init(val = "<ACTION_NAME>:".into())]
    label_name : GString,

    #[export]
    #[var(get, set = set_input_action_name)]
    input_action_name : GString,

    #[export]
    #[var(get, set = set_unassigned_text)]
    #[init(val = "<UNASSIGNED>".into())]
    unassigned_text : GString,

    #[export]
    #[var(get, set = set_unassigned_icon)]
    unassigned_icon : Option<Gd<Texture2D>>,

    #[export]
    #[var(get, set = set_keyboard_icon)]
    keyboard_icon : Option<Gd<Texture2D>>,

    #[export]
    #[var(get, set = set_controller_icon)]
    controller_icon : Option<Gd<Texture2D>>,


    // Non-exported

    #[var]
    #[init(node = "%ActionNameLabel")]
    action_name_label : OnReady<Gd<Label>>,

    #[var]
    #[init(node = "%RebindButton1")]
    rebind_button_1 : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%RebindButton2")]
    rebind_button_2 : OnReady<Gd<Button>>,


    // set = rust_set_state
    #[init(val = RebindControlRowState::Default)]
    state : RebindControlRowState,


    base : Base<HBoxContainer>,
}


#[godot_dyn]
impl IState for RebindControlRow {
    fn do_enter(&mut self) {
        self.base_mut().set_process_unhandled_input(true);
    }

    fn do_exit(&mut self) {
        self.base_mut().set_process_unhandled_input(false);
    }
}


#[godot_api]
impl IHBoxContainer for RebindControlRow {
    fn ready(&mut self) {
        let buttons = self.get_rebind_buttons();

        for (i, button) in buttons.iter().enumerate() {
            let button = button.clone();
            let button_gd = button.clone();

            // pressed
            button
                .signals()
                .pressed()
                .builder()
                .flags(ConnectFlags::DEFERRED)
                .connect_other_mut(
                    self,
                    move |me| {
                        me.on_rebind_button_pressed(button_gd.clone(), i as i32);
                    }
                );
        }

        self.refresh();
    }


    fn unhandled_input(&mut self, event : Gd<InputEvent>) {
        match &self.state {
            RebindControlRowState::Default => {
                // Do nothing
            },
            RebindControlRowState::ListeningForInput((_, button_id)) => {
                let button_id = *button_id as usize;

                let mut input_map = InputMap::singleton();
                let mut action_events = input_map
                    .action_get_events(self.input_action_name.arg())
                    .iter_shared()
                    .collect::<Vec<Gd<InputEvent>>>();
                
                let mut committed_new_rebinds_opt = None;
                
                if event.is_action_pressed(UI_CANCEL) {
                    if button_id < action_events.len() {
                        action_events.remove(button_id);
                    }

                    let committed_action_events = std::mem::take(&mut action_events);
                    committed_new_rebinds_opt = Some(committed_action_events);
                }

                if committed_new_rebinds_opt.is_none() {
                    // Check if valid event - don't rebind if input is unrecognized
                    if let Some((event, _, _)) = self.parse_input_event(event) {
                        // Accepted event, figure out where to put it
                        

                        if let Some(existing) = action_events.get_mut(button_id) {
                            *existing = event;
                            
                        } else {
                            action_events.push(event);
                        }

                        committed_new_rebinds_opt = Some(action_events);
                    }
                }


                // Rebind if committed
                if let Some(committed_rebinds) = committed_new_rebinds_opt {
                    let input_action_name = &self.input_action_name;
                    input_map.action_erase_events(input_action_name.arg());

                    for event in committed_rebinds {
                        input_map.action_add_event(input_action_name.arg(), &event);
                    }

                    self.rust_set_state(RebindControlRowState::Default);
                    self.update_ui_with_bindings();
                    self
                        .signals()
                        .notify_finished_rebinding()
                        .emit();
                }
            },
            RebindControlRowState::Overshadowed => {
                // Do nothing
            },
        }
    }
}


#[godot_api]
impl RebindControlRow {
    #[signal]
    pub fn notify_waiting_for_input();

    #[signal]
    pub fn notify_finished_rebinding();


    #[func]
    pub fn set_label_name(&mut self, label_name : GString) {
        // Set
        self.label_name = label_name.clone();

        if !self.base().is_node_ready() {
            return;
        }

        self.action_name_label.set_text(&label_name);
    }


    #[func]
    pub fn set_input_action_name(&mut self, input_action_name : GString) {
        // Set
        self.input_action_name = input_action_name.clone();

        if !self.base().is_node_ready() {
            return;
        }
    }


    #[func]
    pub fn set_unassigned_text(&mut self, text : GString) {
        // Set
        self.unassigned_text = text.clone();

        if !self.base().is_node_ready() {
            return;
        }

        let mut buttons = self.get_rebind_buttons();
        for button in buttons.iter_mut() {
            button.set_text(&text);
        }
    }


    #[func]
    pub fn set_unassigned_icon(&mut self, icon_opt : Option<Gd<Texture2D>>) {
        // Set
        self.unassigned_icon = icon_opt.clone();

        if !self.base().is_node_ready() {
            return;
        }

        let mut buttons = self.get_rebind_buttons();
        for button in buttons.iter_mut() {
            button.set_button_icon(icon_opt.as_ref());
        }
    }


    #[func]
    pub fn set_keyboard_icon(&mut self, icon : Option<Gd<Texture2D>>) {
        // Set
        self.keyboard_icon = icon;

        self.update_ui_with_bindings();
    }


    #[func]
    pub fn set_controller_icon(&mut self, icon : Option<Gd<Texture2D>>) {
        // Set
        self.controller_icon = icon;

        self.update_ui_with_bindings();
    }


    fn set_buttons_disabled(&mut self, disabled : bool) {
        let mut buttons = self.get_rebind_buttons();
        for button in buttons.iter_mut() {
            button.set_disabled(disabled);
        }
    }


    fn rust_set_state(&mut self, state : RebindControlRowState) {
        // Set
        let previous_state = std::mem::replace(&mut self.state, state);

        let state = &self.state;
        let mut disabled = false;

        match state {
            RebindControlRowState::Default => {
                if let RebindControlRowState::ListeningForInput((mut previous_button, _)) = previous_state {
                    previous_button.grab_focus();
                }
            },
            RebindControlRowState::ListeningForInput((event_button, _id)) => {
                let mut event_button = event_button.clone();
                event_button.release_focus();
                event_button.set_text("Press any button...");

                let mut buttons = self.get_rebind_buttons();
                for button in buttons.iter_mut() {
                    let is_event_button = *button == event_button;

                    button.set_disabled(!is_event_button);
                }


                self
                    .signals()
                    .notify_waiting_for_input()
                    .emit();
            },
            RebindControlRowState::Overshadowed => {
                // Explicitly changing disabled here for clarity
                disabled = true;
            }
        }

        self.set_buttons_disabled(disabled);
    }


    #[func]
    pub fn on_become_overshadowed(&mut self) {
        self.rust_set_state(RebindControlRowState::Overshadowed);
    }


    #[func]
    pub fn on_release_overshadowed(&mut self) {
        if self.state == RebindControlRowState::Overshadowed {
            self.rust_set_state(RebindControlRowState::Default);
        }
    }


    #[func]
    fn on_rebind_button_pressed(&mut self, button : Gd<Button>, id : i32) {
        if self.state == RebindControlRowState::Default {
            self.rust_set_state(RebindControlRowState::ListeningForInput((button, id)));
        }
    }


    fn refresh(&mut self) {
        let label_name = std::mem::take(&mut self.label_name);
        self.set_label_name(label_name);

        let input_action_name = std::mem::take(&mut self.input_action_name);
        self.set_input_action_name(input_action_name);

        let unassigned_text = std::mem::take(&mut self.unassigned_text);
        self.set_unassigned_text(unassigned_text);

        let unassigned_icon = std::mem::take(&mut self.unassigned_icon);
        self.set_unassigned_icon(unassigned_icon);

        let keyboard_icon = std::mem::take(&mut self.keyboard_icon);
        self.set_keyboard_icon(keyboard_icon);

        let controller_icon = std::mem::take(&mut self.controller_icon);
        self.set_controller_icon(controller_icon);

        self.update_ui_with_bindings();
    }


    pub fn update_ui_with_bindings(&mut self) {
        let engine = Engine::singleton();
        if !self.base().is_node_ready() || engine.is_editor_hint() {
            return;
        }

        let mut input_map = InputMap::singleton();
        let bindings = input_map.action_get_events(self.input_action_name.arg());

        let events_and_text_and_icons = bindings
            .iter_shared()
            .filter_map(|event| {
                self.parse_input_event(event)
            })
            .take(2);

        let mut buttons = self.get_rebind_buttons();
        for button in buttons.iter_mut() {
            button.set_text(&self.unassigned_text);
            button.set_button_icon(self.unassigned_icon.as_ref());
        }

        let parsed_and_buttons = events_and_text_and_icons.zip(buttons.iter_mut());
        for ((_, button_text, button_icon), button) in parsed_and_buttons {
            button.set_text(&button_text);
            button.set_button_icon(button_icon.as_ref());
        }
    }


    pub fn get_rebind_buttons(&self) -> [Gd<Button>; 2] {
        [
            self.rebind_button_1.clone(),
            self.rebind_button_2.clone(),
        ]
    }


    pub fn get_button(&self, idx : usize) -> Option<Gd<Button>> {
        self.get_rebind_buttons().get(idx).cloned()
    }


    fn parse_input_event(&self, event : Gd<InputEvent>) -> Option<(Gd<InputEvent>, GString, Option<Gd<Texture2D>>)> {
        // Key event
        let as_key_event_result = event.clone().try_cast::<InputEventKey>();
        if let Ok(ok) = as_key_event_result {
            let name = ok.as_text_keycode();
            let icon = self.keyboard_icon.clone();

            return Some((event, name, icon));
        }

        // Controller button
        let as_controller_button_event_result = event.clone().try_cast::<InputEventJoypadButton>();
        if let Ok(ok) = as_controller_button_event_result {
            let name = simplify_controller_text(ok.as_text());
            let icon = self.controller_icon.clone();

            return Some((event, name, icon));
        }
        
        // Controller axis
        let as_controller_axis_event_result = event.clone().try_cast::<InputEventJoypadMotion>();
        if let Ok(ok) = as_controller_axis_event_result {
            let name = simplify_controller_text(ok.as_text());
            let icon = self.controller_icon.clone();

            // Only react if axis value is significant
            const SIGNIFICANT_TRESHOLD : f32 = 0.9;
            let abs_axis_value = ok.get_axis_value().abs();

            if abs_axis_value >= SIGNIFICANT_TRESHOLD {
                return Some((event, name, icon));
            }
        }

        None
    }
}


// Utility

fn simplify_controller_text(gstring : GString) -> GString {
    let part_1 = gstring.split("(").get(1).unwrap_or_default();

    let end_tokens = [
        ",",
        ")"
    ];

    let part_2 = (|| {
        for token in end_tokens {
            let split = part_1.split(token);
            if split.len() > 1 {
                return split.get(0).unwrap_or_default()
            }
        }

        part_1.split(",").get(0).unwrap_or_default()
    })();

    let simplified = part_2;
    simplified
}
