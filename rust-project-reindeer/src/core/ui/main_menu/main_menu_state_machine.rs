use godot::{classes::{Control, IControl}, prelude::*};
use strum::{EnumCount, VariantArray};

use crate::core::{levels::main_level::main_level_constructor_info::GodotMainLevelConstructorInfo, ui::{controls_menu::{controls_menu::ControlsMenu, controls_menu_request::ControlsMenuRequest}, i_sub_menu_state::ISubMenuState, main_menu::{about_menu::AboutMenu, about_menu_request::AboutMenuRequest, load_map_menu::LoadMapMenu, load_map_menu_request::LoadMapMenuRequest, main_menu::MainMenu, main_menu_state::MainMenuState}, options_menu::{options_menu::OptionsMenu, options_menu_request::OptionsMenuRequest}}};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct MainMenuStateMachine {
    #[var]
    #[init(node = "%TitleMenu")]
    title_menu : OnReady<Gd<MainMenu>>,

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

    #[var(get, set = set_state)]
    #[init(val = MainMenuState::Title)]
    state : MainMenuState,

    base : Base<Control>,
}


#[godot_api]
impl IControl for MainMenuStateMachine {
    fn ready(&mut self) {
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

        self.refresh();
    }
}


#[godot_dyn]
impl ISubMenuState for MainMenuStateMachine {
    fn reset(&mut self) {
        let submenus = self.get_all_submenu_controls();
        for mut submenu in submenus {
            submenu.dyn_bind_mut().reset();
        }
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
            submenu.hide();
        }

        let mut active_submenu = self.get_submenu_control(state);
        active_submenu.dyn_bind_mut().enter();
        active_submenu.show();
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


    fn refresh(&mut self) {
        let current_state = self.state;
        self.set_state(current_state);
    }


    fn get_submenu_control(&self, state : MainMenuState) -> DynGd<Control, dyn ISubMenuState> {
        match state {
            MainMenuState::Title => self.title_menu.clone().into_dyn().upcast(),
            MainMenuState::Options => self.options_menu.clone().into_dyn().upcast(),
            MainMenuState::LoadMap => self.load_map_menu.clone().into_dyn().upcast(),
            MainMenuState::Controls => self.controls_menu.clone().into_dyn().upcast(),
            MainMenuState::About => self.about_menu.clone().into_dyn().upcast(),
        }
    }


    fn get_all_submenu_controls(&self) -> [DynGd<Control, dyn ISubMenuState>; MainMenuState::COUNT] {
        let states : &[MainMenuState; MainMenuState::COUNT] = MainMenuState::VARIANTS.try_into().unwrap();

        states.map(|state| {
            self.get_submenu_control(state)
        })
    }
}
