use godot::{classes::{Control, IControl}, prelude::*};
use strum::{EnumCount, VariantArray};

use crate::core::{maze::maze::Maze, ui::main_menu::{i_main_menu_sub_menu::IMainMenuSubMenu, load_map_menu::LoadMapMenu, main_menu_state::MainMenuState, title_menu::TitleMenu}};


#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct MainMenuStateMachine {
    #[var]
    #[init(node = "%TitleMenu")]
    title_menu : OnReady<Gd<TitleMenu>>,

    #[var]
    #[init(node = "%LoadMapMenu")]
    load_map_menu : OnReady<Gd<LoadMapMenu>>,

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
        
        // Title [start] -> Load map
        self
            .title_menu
            .signals()
            .request_start()
            .connect_other(
                self,
                Self::on_title_requests_start
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
        
        let current_state = self.state;
        self.set_state(current_state);
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
    fn on_title_requests_start(&mut self) {
        self.set_state(MainMenuState::LoadMap);
    }


    #[func]
    fn on_load_map_requests_cancel(&mut self) {
        self.set_state(MainMenuState::Title);
    }


    fn get_submenu_control(&self, state : MainMenuState) -> DynGd<Control, dyn IMainMenuSubMenu> {
        match state {
            MainMenuState::Title => self.title_menu.clone().into_dyn().upcast(),
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
