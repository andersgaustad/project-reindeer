use godot::{classes::{Button, Control, FileAccess, IControl, InputEvent, Os, RichTextLabel, TabContainer, TextureButton, file_access::ModeFlags}, prelude::*};

use crate::{core::{audio::{i_sfx_manager::ISFXManager, sfx_entry::SFXEntry}, run::{i_has_run::IHasRun, run::Run}, ui::{i_sub_menu_state::IState, main_menu::about_menu_request::AboutMenuRequest}, utility::node_utility}, input_map::UI_CANCEL};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct AboutMenu {
    #[export(file = "*.txt")]
    #[var(get, set = set_credits_text_file)]
    credits_text_file : GString,


    // Non-exported

    #[var]
    #[init(node = "%TabContainer")]
    tab_container : OnReady<Gd<TabContainer>>,

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


    run : Option<Gd<Run>>,


    base : Base<Control>,
}


#[godot_api]
impl IControl for AboutMenu {
    fn ready(&mut self) {
        let gd = self.to_gd();

        self.run = node_utility::try_find_parent_of_type(gd.upcast());

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

        self
            .tab_container
            .signals()
            .tab_changed()
            .connect_other(
                self,
                Self::on_tab_changed
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

        if event.is_action_pressed(UI_CANCEL) {
            self.
                run_deferred(|me| {
                    me.on_back_pressed();
                }
            );
        }
    }
}


#[godot_dyn]
impl IHasRun for AboutMenu {
    fn get_run(&self) -> Option<Gd<Run>> {
        self.run.clone()
    }
}


#[godot_dyn]
impl IState for AboutMenu {
    fn do_enter(&mut self) {
        self.back_button.grab_focus();

        self.refresh();
    }
    

    fn do_exit(&mut self) {

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
    fn on_tab_changed(&mut self, _tab : i64) {
        self.make_click_sound();
    }


    #[func]
    fn on_back_pressed(&mut self) {
        self.make_click_sound();
        self
            .signals()
            .request()
            .emit(AboutMenuRequest::Back);
    }


    fn handle_text_clicked(&mut self, variant : Variant) {
        let string = variant.stringify();

        let mut os = Os::singleton();
        os.shell_open(&string);
    }


    fn refresh(&mut self) {
        let credits_text = std::mem::take(&mut self.credits_text_file);
        self.set_credits_text_file(credits_text);
    }


    fn make_click_sound(&mut self) {
        let mut sfx = self.get_sfx_mananger();
        sfx.play(SFXEntry::Click);
    }
}
