use godot::{classes::{Button, Engine, HBoxContainer, IHBoxContainer, InputEventJoypadButton, InputEventJoypadMotion, InputEventKey, InputMap, Label, Texture2D}, prelude::*};


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


    base : Base<HBoxContainer>,
}


#[godot_api]
impl IHBoxContainer for RebindControlRow {
    fn ready(&mut self) {
        self.refresh();
    }
}


#[godot_api]
impl RebindControlRow {
    #[signal]
    pub fn notify_ready_for_input();


    #[func]
    pub fn set_label_name(&mut self, label_name : GString) {
        // Set
        self.label_name = label_name.clone();

        if !self.base().is_node_ready() {
            return;
        }

        self.action_name_label.set_text(&label_name);

        godot_print!(":?- Set label name to {}", &label_name);
    }


    #[func]
    pub fn set_input_action_name(&mut self, input_action_name : GString) {
        // Set
        self.input_action_name = input_action_name.clone();

        if !self.base().is_node_ready() {
            return;
        }
        
        godot_print!(":?- Set input name to {}", &input_action_name);
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

        let binding_names = bindings
            .iter_shared()
            .filter_map(|event| {
                let mut event = event;

                // Key event
                let as_key_event_result = event.try_cast::<InputEventKey>();
                match as_key_event_result {
                    Ok(ok) => {
                        let name = ok.as_text_keycode();
                        return Some((name, self.keyboard_icon.clone()));
                    },
                    Err(returned) => {
                        event = returned;
                    },
                }

                // Controller button
                let as_controller_button_event_result = event.try_cast::<InputEventJoypadButton>();
                match as_controller_button_event_result {
                    Ok(ok) => {
                        let name = ok.as_text();
                        let simplified = simplify_controller_text(name);
                        return Some((simplified, self.controller_icon.clone()));
                    },
                    Err(returned) => {
                        event = returned;
                    },
                }

                let as_controller_axis_event_result = event.try_cast::<InputEventJoypadMotion>();
                match as_controller_axis_event_result {
                    Ok(ok) => {
                        let name = ok.as_text();
                        let simplified = simplify_controller_text(name);
                        return Some((simplified, self.controller_icon.clone()));
                    },
                    Err(_returned) => {
                        
                    },
                }

                None
            })
            .take(2);

        let mut buttons = self.get_rebind_buttons();
        for button in buttons.iter_mut() {
            button.set_text(&self.unassigned_text);
            button.set_button_icon(self.unassigned_icon.as_ref());
        }

        let name_and_button = binding_names.zip(buttons.iter_mut());
        for ((name, button_icon), button) in name_and_button {
            button.set_text(&name);
            button.set_button_icon(button_icon.as_ref());
        }
    }


    fn get_rebind_buttons(&self) -> [Gd<Button>; 2] {
        [
            self.rebind_button_1.clone(),
            self.rebind_button_2.clone(),
        ]
    }
}


// Utility

fn simplify_controller_text(gstring : GString) -> GString {
    gstring
        .split("(")
        .get(1)
        .unwrap_or_default()
        .split(",")
        .get(0)
        .unwrap_or_default()
}
