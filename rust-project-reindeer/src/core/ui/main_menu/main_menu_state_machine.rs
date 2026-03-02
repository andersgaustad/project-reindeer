use godot::{classes::{Control, IControl}, prelude::*};
use strum::{EnumCount, VariantArray};

use crate::core::{maze::maze::Maze, ui::{main_menu::{i_main_menu_sub_menu::IMainMenuSubMenu, load_map_menu::LoadMapMenu, main_menu_state::MainMenuState, title_menu::TitleMenu}, options_menu::option_menu::OptionsMenu}};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct MainMenuStateMachine {
    #[var]
    #[init(node = "%TitleMenu")]
    title_menu : OnReady<Gd<TitleMenu>>,

    #[var]
    #[init(node = "%LoadMapMenu")]
    load_map_menu : OnReady<Gd<LoadMapMenu>>,

    #[var]
    #[init(node = "%OptionsMenu")]
    options_menu : OnReady<Gd<OptionsMenu>>,

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
                        .request_set_maze()
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

        // Options [back] -> Title
        self
            .options_menu
            .signals()
            .request_exit()
            .connect_other(
                self,
                Self::on_options_requests_cancel
            );
        
        // Load map [cancel] -> Title
        self
            .load_map_menu
            .signals()
            .request_cancel()
            .connect_other(
                self,
                Self::on_load_map_requests_cancel
            );
        

        self.refresh();
    }
}


#[godot_dyn]
impl IMainMenuSubMenu for MainMenuStateMachine {
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
    pub fn request_set_maze(maze : Gd<Maze>);


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
        active_submenu.show();
    }


    #[func]
    fn on_title_requests_state(&mut self, requested : MainMenuState) {
        self.set_state(requested);
    }


    #[func]
    fn on_options_requests_cancel(&mut self) {
        self.set_state(MainMenuState::Title);
    }


    #[func]
    fn on_load_map_requests_cancel(&mut self) {
        self.set_state(MainMenuState::Title);
    }


    fn refresh(&mut self) {
        let current_state = self.state;
        self.set_state(current_state);
    }


    fn get_submenu_control(&self, state : MainMenuState) -> DynGd<Control, dyn IMainMenuSubMenu> {
        match state {
            MainMenuState::Title => self.title_menu.clone().into_dyn().upcast(),
            MainMenuState::Options => self.options_menu.clone().into_dyn().upcast(),
            MainMenuState::LoadMap => self.load_map_menu.clone().into_dyn().upcast(),
        }
    }


    fn get_all_submenu_controls(&self) -> [DynGd<Control, dyn IMainMenuSubMenu>; MainMenuState::COUNT] {
        let states : &[MainMenuState; MainMenuState::COUNT] = MainMenuState::VARIANTS.try_into().unwrap();

        states.map(|state| {
            self.get_submenu_control(state)
        })
    }
}
