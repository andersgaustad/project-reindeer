use godot::{classes::{Button, Control, IControl, RichTextLabel}, prelude::*};

use crate::core::ui::{i_sub_menu_state::ISubMenuState, letter_menu::{letter_menu_inbox_state::LetterMenuInboxState, letter_menu_request::LetterMenuRequest}};


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


    base : Base<Control>,
}


#[godot_api]
impl IControl for LetterMenu {
    fn ready(&mut self) {
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
}


#[godot_dyn]
impl ISubMenuState for LetterMenu {
    fn enter(&mut self) {
        if self.inbox_state == LetterMenuInboxState::NewMail {
            self.inbox_state = LetterMenuInboxState::AllRead;
        }

        self
            .back_button
            .grab_focus();
    }


    fn reset(&mut self) {

    }
}


#[godot_api]
impl LetterMenu {
    #[signal]
    pub fn request(request : LetterMenuRequest);


    #[func]
    fn on_back_pressed(&mut self) {
        self
            .signals()
            .request()
            .emit(LetterMenuRequest::Back);
    }


    pub fn send_mail(&mut self, mail : GString) {
        godot_print!(":?- Got mail!");
        self.letter_text_label.set_text(&mail);
        self.inbox_state = LetterMenuInboxState::NewMail;
    }


    pub fn rust_get_inbox_state(&self) -> LetterMenuInboxState {
        self.inbox_state
    }
}
