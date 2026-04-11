use godot::{classes::{Button, Control, IControl, InputEvent, Texture2D}, prelude::*};
use strum::IntoEnumIterator;

use crate::{cfg, core::{audio::{i_sfx_manager::ISFXManager, sfx_entry::SFXEntry}, levels::main_level::pathfinding_state::PathfindingState, run::{i_has_run::IHasRun, run::Run}, ui::{buttons::button_state_info::ButtonStateInfo, i_state::IState, letter_menu::{letter_menu::LetterMenu, letter_menu_inbox_state::LetterMenuInboxState}, pause_menu::{pause_menu_button_type::PauseMenuButtonType, pause_menu_face_request::PauseMenuFaceRequest}}, utility::node_utility}, input_map::UI_CANCEL};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct PauseMenuFace {
    #[export]
    #[var]
    new_mail_icon : OnEditor<Gd<Texture2D>>,


    // Non-exported

    #[var]
    #[init(node = "%StartButton")]
    start_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ResumeButton")]
    resume_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%MailButton")]
    mail_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%OptionsButton")]
    options_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ControlsButton")]
    controls_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%MainMenuButton")]
    main_menu_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ExitButton")]
    exit_button : OnReady<Gd<Button>>,


    #[var(get, set = set_letter_menu)]
    letter_menu : Option<Gd<LetterMenu>>,


    run : Option<Gd<Run>>,


    base : Base<Control>,
}


#[godot_api]
impl IControl for PauseMenuFace {
    fn ready(&mut self) {
        let gd = self.to_gd();

        self.run = node_utility::try_find_parent_of_type(gd.upcast());

        // Signals

        for button_type in PauseMenuButtonType::iter() {
            let button = self.get_button_and_pressed_callback_from_type(button_type);
            button
                .signals()
                .pressed()
                .connect_other(
                    self,
                    move |me|{
                        me.on_button_pressed(button_type);
                    }
                );
        }

        // Web build?
        if cfg::is_web_build() {
            let exit_button = &mut self.exit_button;
            exit_button.set_disabled(true);
            exit_button.set_tooltip_text(cfg::WEB_BUILD_BUTTON_DISABLED_TOOLTIP);
        }

        self.update_mail_button();
    }


    fn unhandled_input(&mut self, event : Gd<InputEvent>) {
        if event.is_action_pressed(UI_CANCEL) {
            self.on_button_pressed(PauseMenuButtonType::Resume);
        }
    }
}


#[godot_dyn]
impl IHasRun for PauseMenuFace {
    fn get_run(&self) -> Option<Gd<Run>> {
        self.run.clone()
    }
}


#[godot_dyn]
impl IState for PauseMenuFace {
    fn enter(&mut self) {
        self.base_mut().set_process_unhandled_input(true);

        self.resume_button.grab_focus();

        #[cfg(debug_assertions)]
        {
            if self.letter_menu.is_none() {
                godot_warn!("PauseMenuFace has no LetterMenu reference - Mail button will not be updated!");
            }
        }

        self.update_mail_button();
    }

    
    fn exit(&mut self) {
        self.base_mut().set_process_unhandled_input(false);
    }
}


#[godot_api]
impl PauseMenuFace {
    #[signal]
    pub fn request(request : PauseMenuFaceRequest);


    #[func]
    pub fn set_letter_menu(&mut self, letter_menu_option : Option<Gd<LetterMenu>>) {
        // Set
        self.letter_menu = letter_menu_option;

        self.update_mail_button();
    }


    #[func]
    fn on_button_pressed(&mut self, button_type : PauseMenuButtonType) {
        self.make_click_sound();

        match button_type {
            PauseMenuButtonType::Start => {
                self
                    .signals()
                    .request()
                    .emit(PauseMenuFaceRequest::Start);

            },
            PauseMenuButtonType::Resume => {
                self
                    .signals()
                    .request()
                    .emit(PauseMenuFaceRequest::Resume);

            },
            PauseMenuButtonType::Mail => {
                self
                    .signals()
                    .request()
                    .emit(PauseMenuFaceRequest::ToMail);

            },
            PauseMenuButtonType::Options => {
                self
                    .signals()
                    .request()
                    .emit(PauseMenuFaceRequest::ToOptions);

            },
            PauseMenuButtonType::Controls => {
                self
                    .signals()
                    .request()
                    .emit(PauseMenuFaceRequest::ToControls);
            },
            PauseMenuButtonType::MainMenu => {
                self
                    .signals()
                    .request()
                    .emit(PauseMenuFaceRequest::ToMainMenu);

            },
            PauseMenuButtonType::Exit => {
                let Some(mut tree) = self.base().get_tree() else {
                    return;
                };

                if cfg::is_web_build() {
                    godot_warn!("Tried exiting in web build! Ignoring...");
                } else {
                    tree.quit();
                }
            },
        }
    }


    #[func]
    pub fn on_level_pathfinding_state_update(
        &mut self,
        _old_state : PathfindingState,
        new_state : PathfindingState
    ) {
        let default_button_state_info = ButtonStateInfo::default();
        self.set_start_button_state_info(default_button_state_info);

        match new_state {
            PathfindingState::NotStarted => {
                // Do nothing, stick with default
            },
            PathfindingState::Countdown => {

                self.set_start_button_state_info(
                    ButtonStateInfo {
                        clickable : false,
                        tooltip : "Countdown initiated!".into()
                    }
                );
            },
            PathfindingState::InProgress => {
                self.set_start_button_state_info(
                    ButtonStateInfo {
                        clickable : false,
                        tooltip : "Pathfinding is running!".into()
                    }
                );

            },
            PathfindingState::Done => {
                self.set_start_button_state_info(
                    ButtonStateInfo {
                        clickable : false,
                        tooltip : "Pathfinding complete - Exit to Main Menu and reset level to run again.".into()
                    }
                );
            },
        }
    }


    pub fn set_start_button_state_info(&mut self, state_info : ButtonStateInfo) {
        let start_button = &mut self.start_button;
        start_button.set_disabled(!state_info.clickable);
        start_button.set_tooltip_text(&state_info.tooltip);
    }


    fn get_button_and_pressed_callback_from_type(&self, ty : PauseMenuButtonType) -> Gd<Button> {
        match ty {
            PauseMenuButtonType::Start      => self.start_button.clone(),
            PauseMenuButtonType::Resume     => self.resume_button.clone(),
            PauseMenuButtonType::Mail       => self.mail_button.clone(),
            PauseMenuButtonType::Options    => self.options_button.clone(),
            PauseMenuButtonType::Controls   => self.controls_button.clone(),
            PauseMenuButtonType::MainMenu   => self.main_menu_button.clone(),
            PauseMenuButtonType::Exit       => self.exit_button.clone(),
        }
    }


    fn update_mail_button(&mut self) {
        let Some(letter_menu) = self.letter_menu.clone() else {
            return;
        };

        let mail_button = &mut self.mail_button;
        let inbox_state = letter_menu.bind().rust_get_inbox_state();
        match inbox_state {
            LetterMenuInboxState::Empty => {
                mail_button.set_disabled(true);
                mail_button.set_button_icon(None::<Gd<Texture2D>>.as_ref());
            },
            LetterMenuInboxState::NewMail => {
                mail_button.set_disabled(false);
                mail_button.set_button_icon(&self.new_mail_icon.clone());
            },
            LetterMenuInboxState::AllRead => {
                mail_button.set_disabled(false);
                mail_button.set_button_icon(None::<Gd<Texture2D>>.as_ref());
            },
        }
    }


    fn make_click_sound(&mut self) {
        let mut sfx = self.get_sfx_mananger();
        sfx.play(SFXEntry::Click);
    }
}
