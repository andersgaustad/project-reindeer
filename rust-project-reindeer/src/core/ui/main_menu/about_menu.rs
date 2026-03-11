use godot::{classes::{Button, Control, FileAccess, IControl, InputEvent, Os, RichTextLabel, TextureButton, file_access::ModeFlags}, prelude::*};

use crate::{core::ui::{i_sub_menu_state::ISubMenuState, main_menu::{about_menu_icon_button_type::AboutMenuIconButtonType, about_menu_request::AboutMenuRequest}}, input_map::CANCEL};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct AboutMenu {
    #[export(file = "*.txt")]
    #[var(get, set = set_credits_text_file)]
    credits_text_file : GString,


    // Non-exported

    #[var]
    #[init(node = "%AboutText")]
    about_text_field : OnReady<Gd<RichTextLabel>>,

    #[var]
    #[init(node = "%GodotIcon")]
    godot_icon : OnReady<Gd<TextureButton>>,

    #[var]
    #[init(node = "%GodotRustIcon")]
    godot_rust_icon : OnReady<Gd<TextureButton>>,

    #[var]
    #[init(node = "%RustIcon")]
    rust_icon : OnReady<Gd<TextureButton>>,

    #[var]
    #[init(node = "%CreditsText")]
    credit_text_field : OnReady<Gd<RichTextLabel>>,

    #[var]
    #[init(node = "%BackButton")]
    back_button : OnReady<Gd<Button>>,


    base : Base<Control>,
}


#[godot_api]
impl IControl for AboutMenu {
    fn ready(&mut self) {
        // about_text_field
        self
            .about_text_field
            .signals()
            .meta_clicked()
            .connect_other(
                self,
                Self::on_about_text_meta_clicked
            );

        // credit_text_field
        self
            .credit_text_field
            .signals()
            .meta_clicked()
            .connect_other(
                self,
                Self::on_credit_text_meta_clicked
            );
        
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

        if event.is_action_pressed(CANCEL) {
            self.
                run_deferred(|me| {
                    me.on_back_pressed();
                }
            );
        }
    }
}


#[godot_dyn]
impl ISubMenuState for AboutMenu {
    fn enter(&mut self) {
        self.back_button.grab_focus();
    }

    fn reset(&mut self) {

    }
}


#[godot_api]
impl AboutMenu {
    #[signal]
    pub fn request(request : AboutMenuRequest);


    #[func]
    fn set_credits_text_file(&mut self, credits : GString) {
        // Set
        self.credits_text_file = credits;

        if !self.base().is_node_ready() {
            return;
        }

        let file_access_opt = FileAccess::open(&self.credits_text_file, ModeFlags::READ);
        let Some(file_access) = file_access_opt else {
            godot_error!("Failed opening '{}'!", &self.credits_text_file);
            return;
        };

        let content = file_access.get_as_text(); 
        self.credit_text_field.set_text(&content);
    }


    #[func]
    fn on_about_text_meta_clicked(&mut self, variant : Variant) {
        self.handle_text_clicked(variant);
    }


    #[func]
    fn on_credit_text_meta_clicked(&mut self, variant : Variant) {
        self.handle_text_clicked(variant);
    }


    #[func]
    fn on_back_pressed(&mut self) {
        self
            .signals()
            .request()
            .emit(AboutMenuRequest::Back);
    }


    fn handle_text_clicked(&mut self, variant : Variant) {
        let string = variant.stringify();
        println!(":?- Clicked: {}", &string);

        let mut os = Os::singleton();
        os.shell_open(&string);
    }


    fn refresh(&mut self) {
        let credits_text = std::mem::take(&mut self.credits_text_file);
        self.set_credits_text_file(credits_text);
    }
}
