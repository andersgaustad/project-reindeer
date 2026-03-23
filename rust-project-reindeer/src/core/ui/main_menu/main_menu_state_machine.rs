use godot::{classes::{AudioStreamPlayer, Control, IControl}, prelude::*};
use strum::{EnumCount, IntoEnumIterator, VariantArray};

use crate::core::{levels::main_level::main_level_constructor_info::GodotMainLevelConstructorInfo, options::option_change::OptionChange, run::{run::Run, i_has_run::IHasRun}, ui::{controls_menu::{controls_menu::ControlsMenu, controls_menu_request::ControlsMenuRequest}, i_state::IState, main_menu::{about_menu::AboutMenu, about_menu_request::AboutMenuRequest, load_map_menu::LoadMapMenu, load_map_menu_request::LoadMapMenuRequest, main_menu_face::MainMenuFace, main_menu_state::MainMenuState}, options_menu::{options_menu::OptionsMenu, options_menu_request::OptionsMenuRequest}}, utility::node_utility};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct MainMenuStateMachine {
    // Exported

    #[export]
    #[var]
    menu_background_level_packed_scene : OnEditor<Gd<PackedScene>>,


    // Non-exported
    #[var]
    #[init(node = "%TitleMenu")]
    title_menu : OnReady<Gd<MainMenuFace>>,

    #[var]
    #[init(node = "%LoadMapMenu")]
    load_map_menu : OnReady<Gd<LoadMapMenu>>,

    #[var]
    #[init(node = "%OptionsMenu")]
    options_menu : OnReady<Gd<OptionsMenu>>,

    #[var]
    #[init(node = "%ControlsMenu")]
    controls_menu : OnReady<Gd<ControlsMenu>>,

    #[var]
    #[init(node = "%AboutMenu")]
    about_menu : OnReady<Gd<AboutMenu>>,


    #[var]
    #[init(node = "%BackgroundMusicPlayer")]
    background_music_player : OnReady<Gd<AudioStreamPlayer>>,
    default_background_music_player_volume : f32,

    #[var(get, set = set_state)]
    #[init(val = MainMenuState::Title)]
    state : MainMenuState,

    #[var(get, set = set_menu_background_level)]
    menu_background_level : Option<Gd<Node>>,


    run : Option<Gd<Run>>,


    base : Base<Control>,
}


#[godot_api]
impl IControl for MainMenuStateMachine {
    fn ready(&mut self) {
        let gd = self.to_gd();

        // Connect signals

        self.run = node_utility::try_find_parent_of_type(gd.upcast());

        let options_opt = self.get_options();
        if let Some(options) = options_opt {
            options
                .signals()
                .option_changed()
                .connect_other(
                    self,
                    Self::on_options_changed
                );
        }

        // Forward request_set_maze
        self
            .load_map_menu
            .signals()
            .notify_maze_created()
            .connect_other(
                self,
                |me, maze| {
                    me
                        .signals()
                        .request_initialize_level()
                        .emit(&maze);
                }
            );
        
        // Title -> Other
        self
            .title_menu
            .signals()
            .request_state()
            .connect_other(
                self,
                Self::on_title_requests_state
            );

        // Options -> Title
        self
            .options_menu
            .signals()
            .request()
            .connect_other(
                self,
                Self::on_options_request
            );
        
        // Load map [cancel] -> Title
        self
            .load_map_menu
            .signals()
            .request()
            .connect_other(
                self,
                Self::on_load_map_requests
            );
        
        // Controls -> Title
        self
            .controls_menu
            .signals()
            .request()
            .connect_other(
                self,
                Self::on_controls_request
            );
        
        // About -> Title
        self
            .about_menu
            .signals()
            .request()
            .connect_other(
                self,
                Self::on_about_requests
            );
        
        self.default_background_music_player_volume = self.background_music_player.get_volume_linear();

        self.refresh();
    }
}


#[godot_dyn]
impl IHasRun for MainMenuStateMachine {
    fn get_run(&self) -> Option<Gd<Run>> {
        self.run.clone()
    }
}


#[godot_dyn]
impl IState for MainMenuStateMachine {
    fn enter(&mut self) {
        self.base_mut().set_process_unhandled_input(true);

        self.toggle_background_level(true);

        self.background_music_player.play();
        self.base_mut().show();
    }


    fn exit(&mut self) {
        self.base_mut().set_process_unhandled_input(false);

        self.toggle_background_level(false);

        self.background_music_player.stop();

        let submenus = self.get_all_submenu_controls();
        for mut submenu in submenus {
            submenu.dyn_bind_mut().exit();
        }
        
        self.base_mut().hide();
    }
}


#[godot_api]
impl MainMenuStateMachine {
    #[signal]
    pub fn request_initialize_level(level_info : Gd<GodotMainLevelConstructorInfo>);


    #[func]
    pub fn set_state(&mut self, state : MainMenuState) {
        self.state = state;
        if !self.base().is_node_ready() {
            return;
        }

        let all_submenus = self.get_all_submenu_controls();

        for mut submenu in all_submenus {
            submenu.dyn_bind_mut().exit();
            submenu.hide();
        }

        let mut active_submenu = self.get_submenu_control(state);
        active_submenu.dyn_bind_mut().enter();
        active_submenu.show();
    }


    #[func]
    pub fn set_menu_background_level(&mut self, background_level : Option<Gd<Node>>) {
        let old_background_level_opt = std::mem::replace(&mut self.menu_background_level, background_level);
        if let Some(mut old_background_level) = old_background_level_opt {
            old_background_level.queue_free();
        }

        if !self.base().is_node_ready() {
            return;
        }

        if let Some(new_background_level_opt) = self.menu_background_level.clone() {
            let mut gd = self.to_gd();
            gd.run_deferred_gd(move |mut me| {
                me.add_child(&new_background_level_opt);
            });
        }
    }


    #[func]
    fn on_title_requests_state(&mut self, requested : MainMenuState) {
        self.set_state(requested);
    }


    #[func]
    fn on_options_request(&mut self, request : OptionsMenuRequest) {
        match request {
            OptionsMenuRequest::Exit => self.set_state(MainMenuState::Title),
        }
    }


    #[func]
    fn on_load_map_requests(&mut self, request : LoadMapMenuRequest) {
        match request {
            LoadMapMenuRequest::Back => self.set_state(MainMenuState::Title),
        }
    }


    #[func]
    fn on_controls_request(&mut self, request : ControlsMenuRequest) {
        match request {
            ControlsMenuRequest::Back => self.set_state(MainMenuState::Title),
        }
    }


    #[func]
    fn on_about_requests(&mut self, request : AboutMenuRequest) {
        match request {
            AboutMenuRequest::Back => self.set_state(MainMenuState::Title),
        }
    }


    #[func]
    fn on_options_changed(&mut self, change : OptionChange) {
        match change {
            OptionChange::LowPerformanceMode => {
                // Do nothing
            },
            OptionChange::VolumeChange => self.on_volume_change(),
        }
    }


    #[func]
    fn on_volume_change(&mut self) {
        let Some(options) = self.get_options() else {
            return;
        };

        let mut music = [
            (self.background_music_player.clone(), self.default_background_music_player_volume)
        ];

        let mut sfx = [
        ];

        let bound_options = options.bind();
        let music_volume_factor = bound_options.get_music_volume();
        let sfx_volume_factor = bound_options.get_sfx_volume();
        drop(bound_options);

        let components_and_default_factors : [(&mut [(Gd<AudioStreamPlayer>, f32)], f32); 2] = [
            (&mut music, music_volume_factor),
            (&mut sfx, sfx_volume_factor) 
        ];

        for (item, volume_factor) in components_and_default_factors {
            for (component, default_factor) in item {
                let volume = volume_factor * *default_factor;
                component.set_volume_linear(volume as f32);
            }
        }
    }


    fn refresh(&mut self) {
        let current_state = self.state;
        self.set_state(current_state);

        let backgroung_level = std::mem::take(&mut self.menu_background_level);
        self.set_menu_background_level(backgroung_level);

        for possible_option_change in OptionChange::iter() {
            self.on_options_changed(possible_option_change);
        }
    }


    fn toggle_background_level(&mut self, active : bool) {
        let mut background_level = None;
        if active {
            background_level = self.menu_background_level_packed_scene.try_instantiate_as();

            #[cfg(debug_assertions)]
            {
                if background_level.is_none() {
                    godot_error!("Failed spawning background level!");
                }
            }
        }

        self.set_menu_background_level(background_level);
    } 


    fn get_submenu_control(&self, state : MainMenuState) -> DynGd<Control, dyn IState> {
        match state {
            MainMenuState::Title => self.title_menu.clone().into_dyn().upcast(),
            MainMenuState::Options => self.options_menu.clone().into_dyn().upcast(),
            MainMenuState::LoadMap => self.load_map_menu.clone().into_dyn().upcast(),
            MainMenuState::Controls => self.controls_menu.clone().into_dyn().upcast(),
            MainMenuState::About => self.about_menu.clone().into_dyn().upcast(),
        }
    }


    fn get_all_submenu_controls(&self) -> [DynGd<Control, dyn IState>; MainMenuState::COUNT] {
        let states : &[MainMenuState; MainMenuState::COUNT] = MainMenuState::VARIANTS.try_into().unwrap();

        states.map(|state| {
            self.get_submenu_control(state)
        })
    }
}
