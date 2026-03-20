use godot::{classes::{Button, Control, FileAccess, IControl, Input, InputEvent, Os, RichTextLabel, ScrollContainer, TabContainer, TextureButton, VScrollBar, file_access::ModeFlags, object::ConnectFlags}, prelude::*};

use crate::{core::{audio::{i_sfx_manager::ISFXManager, sfx_entry::SFXEntry}, run::{i_has_run::IHasRun, run::Run}, ui::{i_sub_menu_state::IState, main_menu::about_menu_request::AboutMenuRequest}, utility::node_utility}, input_map::{UI_CANCEL, UI_DOWN, UI_UP}};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct AboutMenu {
    #[export(file = "*.txt")]
    #[var(get, set = set_credits_text_file)]
    credits_text_file : GString,

    #[export]
    #[var]
    #[init(val = 1500.0)]
    pixels_scrolled_per_second : f64,


    // Non-exported

    #[var]
    #[init(node = "%TabContainer")]
    tab_container : OnReady<Gd<TabContainer>>,

    #[var]
    #[init(node = "%AboutText")]
    about_text_field : OnReady<Gd<RichTextLabel>>,

    #[var]
    #[init(node = "%ScrollContainer")]
    scroll_container : OnReady<Gd<ScrollContainer>>,

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

    #[var]
    #[init(node = "%Scroller")]
    scroller : OnReady<Gd<VScrollBar>>,


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
        
        let scroll_container_internal_scroller_opt = self.scroll_container.get_v_scroll_bar();
        if let Some(scroll_container_internal_scroller) = scroll_container_internal_scroller_opt {
            scroll_container_internal_scroller
                .signals()
                .value_changed()
                .builder()
                .flags(ConnectFlags::DEFERRED)
                .connect_other_mut(
                    self,
                    Self::on_scroll_container_scroll_value_changed
                );
            
            scroll_container_internal_scroller
                .signals()
                .changed()
                .connect_other(
                    self,
                    Self::refresh_scroller_dimensions
                );
        }

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
        
        // scroller
        self
            .scroller
            .signals()
            .value_changed()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(
                self,
                Self::on_scroller_value_changed
            );
        
        self
            .scroller
            .signals()
            .gui_input()
            .connect_other(
                self,
                Self::on_scroller_gui_input
            );

        self.refresh();        
    }


    fn process(&mut self, delta : f64) {
        if self.scroller.has_focus() {
            let input = Input::singleton();
            let frame_scroll = input.get_axis(UI_UP, UI_DOWN);
            if !frame_scroll.is_zero_approx() {
                let scroll_delta = f64::from(frame_scroll) * self.pixels_scrolled_per_second * delta;
                let new_scroll = self.scroller.get_value() + scroll_delta;
                self.scroller.set_value(new_scroll);
            }
        }
    }


    fn unhandled_input(&mut self, event : Gd<InputEvent>) {
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
        self.base_mut().set_process(true);
        self.base_mut().set_process_unhandled_input(true);

        self.back_button.grab_focus();

        self.refresh();
    }
    

    fn do_exit(&mut self) {
        self.base_mut().set_process(false);
        self.base_mut().set_process_unhandled_input(false);
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
    fn on_scroll_container_scroll_value_changed(&mut self, value : f64) {
        self.scroller.set_value_no_signal(value);
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


    #[func]
    fn on_scroller_value_changed(&mut self, value : f64) {
        let round = value.round() as i32;
        self.scroll_container.set_v_scroll(round);
    }

    
    #[func]
    fn on_scroller_gui_input(&mut self, event : Gd<InputEvent>) {
        if event.is_action(UI_UP) || event.is_action(UI_DOWN) {
            let viewport_opt = self.base().get_viewport();
            if let Some(mut viewport) = viewport_opt {
                viewport.set_input_as_handled();
            }
        }
    }


    fn handle_text_clicked(&mut self, variant : Variant) {
        let string = variant.stringify();

        let mut os = Os::singleton();
        os.shell_open(&string);
    }


    fn refresh(&mut self) {
        let credits_text = std::mem::take(&mut self.credits_text_file);
        self.set_credits_text_file(credits_text);

        self.refresh_scroller_dimensions();
    }


    fn refresh_scroller_dimensions(&mut self) {
        let scroll_bar_opt = self.scroll_container.get_v_scroll_bar();

        let Some(scroll_bar) = scroll_bar_opt else {
            return;
        };

        let scroll_container_max = scroll_bar.get_max();
        let scroll_container_page = scroll_bar.get_page();

        let scroller_page = self.scroller.get_page();

        let adjusted_max = scroll_container_max + scroller_page - scroll_container_page;

        self.scroller.set_max(adjusted_max);
    }


    fn make_click_sound(&mut self) {
        let mut sfx = self.get_sfx_mananger();
        sfx.play(SFXEntry::Click);
    }
}
