use godot::{classes::{Button, Control, IControl, InputEvent, RichTextLabel}, prelude::*};

use crate::{core::{audio::{i_sfx_manager::ISFXManager, sfx_entry::SFXEntry}, run::{i_has_run::IHasRun, run::Run}, ui::{i_state::IState, letter_menu::{letter_menu_inbox_state::LetterMenuInboxState, letter_menu_request::LetterMenuRequest}}, utility::node_utility}, input_map::UI_CANCEL};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct LetterMenu {
    #[var]
    #[init(node = "%LetterTextLabel")]
    letter_text_label : OnReady<Gd<RichTextLabel>>,

    #[var]
    #[init(node = "%BackButton")]
    back_button : OnReady<Gd<Button>>,

    
    #[var]
    #[init(val = LetterMenuInboxState::Empty)]
    inbox_state : LetterMenuInboxState,

    
    run : Option<Gd<Run>>,


    base : Base<Control>,
}


#[godot_api]
impl IControl for LetterMenu {
    fn ready(&mut self) {
        let gd = self.to_gd();

        self.run = node_utility::try_find_parent_of_type(gd.upcast());

        // back_button
        self
            .back_button
            .signals()
            .pressed()
            .connect_other(
                self,
                Self::on_back_pressed
            );
    }


    fn unhandled_input(&mut self, event : Gd<InputEvent>) {
        if event.is_action_pressed(UI_CANCEL) {
            self.on_back_pressed();
            return;
        }
    }
}


impl IHasRun for LetterMenu {
    fn get_run(&self) -> Option<Gd<Run>> {
        self.run.clone()
    }
}


#[godot_dyn]
impl IState for LetterMenu {
    fn enter(&mut self) {
        self.base_mut().set_process_unhandled_input(true);

        if self.inbox_state == LetterMenuInboxState::NewMail {
            self.inbox_state = LetterMenuInboxState::AllRead;
        }

        self
            .back_button
            .grab_focus();
        
        self.base_mut().show();
    }


    fn exit(&mut self) {
        self.base_mut().set_process_unhandled_input(false);

        self.base_mut().hide();
    }
}


#[godot_api]
impl LetterMenu {
    #[signal]
    pub fn request(request : LetterMenuRequest);


    #[func]
    fn on_back_pressed(&mut self) {
        self.make_click_sound();
        self
            .signals()
            .request()
            .emit(LetterMenuRequest::Back);
    }


    pub fn send_mail(&mut self, mail : GString) {
        self.letter_text_label.set_text(&mail);
        self.inbox_state = LetterMenuInboxState::NewMail;
    }


    fn make_click_sound(&mut self) {
        let mut sfx = self.get_sfx_mananger();
        sfx.play(SFXEntry::Click);
    }


    pub fn rust_get_inbox_state(&self) -> LetterMenuInboxState {
        self.inbox_state
    }
}
