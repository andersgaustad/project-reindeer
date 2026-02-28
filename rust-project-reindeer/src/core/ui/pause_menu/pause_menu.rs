use godot::{classes::{Button, Control, IControl, InputEvent}, prelude::*};

use crate::{core::{levels::main_level::pathfinding_state::PathfindingState, ui::{button_state_info::ButtonStateInfo, pause_menu::pause_menu_button_types::PauseMenuButtonType}}, input_map::CANCEL};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct PauseMenu {
    #[var]
    #[init(node = "%StartButton")]
    start_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ResumeButton")]
    resume_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%OptionsButton")]
    options_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%MainMenuButton")]
    main_menu_button : OnReady<Gd<Button>>,

    #[var]
    #[init(node = "%ExitButton")]
    exit_button : OnReady<Gd<Button>>,


    base : Base<Control>,
}


#[godot_api]
impl IControl for PauseMenu {
    fn unhandled_input(&mut self, event : Gd<InputEvent>) {
        if !self.base().is_visible() {
            return;
        }

        if event.is_action_pressed(CANCEL) {
            self
                .resume_button
                .signals()
                .pressed()
                .emit();
        }
    }
}


#[godot_api]
impl PauseMenu {
    #[signal]
    pub fn button_pressed(button_type : PauseMenuButtonType);


    pub fn set_start_button_state_info(&mut self, state_info : ButtonStateInfo) {
        let start_button = &mut self.start_button;
        start_button.set_disabled(!state_info.clickable);
        start_button.set_tooltip_text(&state_info.tooltip);
    }


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


    pub fn get_button_from_type(&self, ty : PauseMenuButtonType) -> Gd<Button> {
        match ty {
            PauseMenuButtonType::Start => self.start_button.clone(),
            PauseMenuButtonType::Resume => self.resume_button.clone(),
            PauseMenuButtonType::Options => self.options_button.clone(),
            PauseMenuButtonType::MainMenu => self.main_menu_button.clone(),
            PauseMenuButtonType::Exit => self.exit_button.clone(),
        }
    }
}
