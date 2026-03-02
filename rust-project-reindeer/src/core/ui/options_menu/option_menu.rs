use godot::{classes::{Button, CheckButton, Control, IControl}, prelude::*};

use crate::core::{options::Options, run::Run, ui::main_menu::i_main_menu_sub_menu::IMainMenuSubMenu, utility::node_utility};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct OptionsMenu {
    #[export]
    #[var(get, set = set_options)]
    options : Option<Gd<Options>>,
 

    // Non-exported:

    // Low-performance

    #[var]
    #[init(node = "%ToggleLowPerformanceModeButton")]
    low_performance_toggle_button : OnReady<Gd<CheckButton>>,

    // Back

    #[var]
    #[init(node = "%BackButton")]
    back_button : OnReady<Gd<Button>>,

    base : Base<Control>
}


#[godot_api]
impl IControl for OptionsMenu {
    fn ready(&mut self) {
        // Signals

        // low_performance_toggle_button 
        self
            .low_performance_toggle_button
            .signals()
            .toggled()
            .connect_other(
                self,
                Self::on_low_performance_mode_toggled
            );
        
        self
            .back_button
            .signals()
            .pressed()
            .connect_other(
                self,
                Self::on_back_pressed
            );


        // Refresh
        self.refresh();        
    }
}


#[godot_dyn]
impl IMainMenuSubMenu for OptionsMenu {
    fn reset(&mut self) {
        self.refresh();
    }
}


#[godot_api]
impl OptionsMenu {
    #[signal]
    pub fn request_exit();


    #[func]
    pub fn set_options(&mut self, options : Option<Gd<Options>>) {
        // Set
        self.options = options;

        if !self.base().is_node_ready() {
            return;
        }
        let options_opt = self.options.clone();

        let good_config = options_opt.is_some();

        let tooltip = if options_opt.is_some() {
            ""
        } else {
            "Could not find Options!"
        };

        let mut low_performance_toggle_button = self.low_performance_toggle_button.clone();
        low_performance_toggle_button.set_disabled(!good_config);
        low_performance_toggle_button.set_tooltip_text(tooltip);

        // Sync
        let Some(options) = options_opt else {
            return;
        };

        let bound_options = options.bind();
        low_performance_toggle_button.set_pressed(bound_options.get_low_performance_mode());

        drop(bound_options);
    }


    #[func]
    fn on_low_performance_mode_toggled(&mut self, toggled : bool) {
        let Some(mut options) = self.options.clone() else {
            return;
        };

        options.bind_mut().set_low_performance_mode(toggled);
    }


    #[func]
    fn on_back_pressed(&mut self) {
        self
            .signals()
            .request_exit()
            .emit();
    }


    fn refresh(&mut self) {
        let options_opt = std::mem::take(&mut self.options);
        let options = if options_opt.is_some() {
            options_opt
        } else {
            // Automatically try to fetch options from tree
            let options = (|| {
                let gd = self.to_gd();
                let run = node_utility::try_find_parent_of_type::<Run>(gd.upcast())?;
                let options = run.bind().get_options();
                options

            })();

            options
        };

        self.set_options(options);
    }
}
