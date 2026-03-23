use godot::{classes::{Button, CenterContainer, Control, IControl}, prelude::*};
use strum::IntoEnumIterator;

use crate::core::{audio::{i_sfx_manager::ISFXManager, sfx_entry::SFXEntry}, run::{i_has_run::IHasRun, run::Run}, ui::{i_state::IState, main_menu::{main_menu_face_button_type::MainMenuFaceButtonType, main_menu_state::MainMenuState}}, utility::node_utility};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct MainMenuFace {
    #[var]
    #[init(node = "%MainContainer")]
    main_container : OnReady<Gd<CenterContainer>>,

    #[var]
    #[init(node = "%StartButton")]
    start_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%OptionsButton")]
    options_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ControlsButton")]
    controls_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ExitButton")]
    exit_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ShowHideUIButton")]
    show_hide_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%AboutButton")]
    about_button : OnReady<Gd<Button>>,

    #[var(get, set = set_showing_ui)]
    #[init(val = true)]
    showing_ui : bool,


    run : Option<Gd<Run>>,


    base : Base<Control>,
}


#[godot_api]
impl IControl for MainMenuFace {
    fn ready(&mut self) {
        let gd = self.to_gd();
        self.run = node_utility::try_find_parent_of_type(gd.upcast());

        for button_type in MainMenuFaceButtonType::iter() {
            let button = self.get_button_from_type(button_type);

            button
                .signals()
                .pressed()
                .connect_other(
                    self,
                    move |me| {
                        me.on_button_pressed(button_type);
                    }
                );
        }
    }
}


#[godot_dyn]
impl IHasRun for MainMenuFace {
    fn get_run(&self) -> Option<Gd<Run>> {
        self.run.clone()
    }
}


#[godot_dyn]
impl IState for MainMenuFace {
    fn enter(&mut self) {
        self.set_showing_ui(true);
        self.start_button.grab_focus();
    }


    fn exit(&mut self) {
        
    }
}


#[godot_api]
impl MainMenuFace {
    #[signal]
    pub fn request_state(main_menu_state : MainMenuState);


    #[func]
    pub fn set_showing_ui(&mut self, show : bool) {
        // Set
        self.showing_ui = show;

        let showing_ui_alpha = self.get_show_hide_button_alpha();
        let mut new_show_hide_color = self.show_hide_button.get_modulate();
        new_show_hide_color.a = showing_ui_alpha;
        self.show_hide_button.set_modulate(new_show_hide_color);

        let mut hideable_controls = self.get_hideable_controls();
        for hideable_control in hideable_controls.iter_mut() {
            hideable_control.set_visible(self.showing_ui);
        }
    }


    #[func]
    fn on_button_pressed(&mut self, button_type : MainMenuFaceButtonType) {
        self.make_click_sound();

        match button_type {
            MainMenuFaceButtonType::Start => {
                self.emit_request_for(MainMenuState::LoadMap);
            },
            MainMenuFaceButtonType::Options => {
                self.emit_request_for(MainMenuState::Options);
            },
            MainMenuFaceButtonType::Controls => {
                self.emit_request_for(MainMenuState::Controls);
            },
            MainMenuFaceButtonType::Exit => {
                self.base().get_tree().unwrap().quit();
            },
            MainMenuFaceButtonType::ShowHide => {
                self.set_showing_ui(!self.showing_ui);
            },
            MainMenuFaceButtonType::About => {
                self.emit_request_for(MainMenuState::About);
            },
        }
    }


    fn emit_request_for(&mut self, state : MainMenuState) {
        self
            .signals()
            .request_state()
            .emit(state);
    }


    fn make_click_sound(&mut self) {
        let mut sfx_opt = self.get_sfx_mananger();
        sfx_opt.play(SFXEntry::Click);
    }


    fn get_hideable_controls(&self) -> [Gd<Control>; 2] {
        let controls = [
            self.main_container.clone().upcast(),
            self.about_button.clone().upcast(),
        ];

        controls
    }


    fn get_show_hide_button_alpha(&self) -> f32 {
        if self.showing_ui {
            1.0
        } else {
            0.5
        }
    }


    fn get_button_from_type(&self, button_type : MainMenuFaceButtonType) -> Gd<Button> {
        match button_type {
            MainMenuFaceButtonType::Start => self.start_button.clone(),
            MainMenuFaceButtonType::Options => self.options_button.clone(),
            MainMenuFaceButtonType::Controls => self.controls_button.clone(),
            MainMenuFaceButtonType::Exit => self.exit_button.clone(),
            MainMenuFaceButtonType::ShowHide => self.show_hide_button.clone(),
            MainMenuFaceButtonType::About => self.about_button.clone(),
        }
    }
}
