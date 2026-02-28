use godot::{classes::{Input, input::MouseMode, object::ConnectFlags}, prelude::*};

use crate::core::{levels::main_level::main_level::MainLevel, maze::maze::Maze, ui::main_menu::{main_menu_state_machine::MainMenuStateMachine, main_menu_state::MainMenuState}};


#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct Run {
    #[export]
    #[var]
    main_level_scene : OnEditor<Gd<PackedScene>>,

    #[var(get, set = set_main_level)]
    main_level : Option<Gd<MainLevel>>,

    #[var]
    #[init(node = "%MainMenu")]
    main_menu_state_machine : OnReady<Gd<MainMenuStateMachine>>,
    

    base : Base<Node>
}


#[godot_api]
impl INode for Run {
    fn ready(&mut self) {
        // Signals
        self
            .main_menu_state_machine
            .signals()
            .request_set_maze()
            .connect_other(
                self,
                Self::on_receive_maze
            );

        let main_level = std::mem::take(&mut self.main_level);
        self.set_main_level(main_level);
    }
}


#[godot_api]
impl Run {
    #[func]
    pub fn set_main_level(&mut self, main_level : Option<Gd<MainLevel>>) {
        // Handle existing
        let existing_level_opt = std::mem::take(&mut self.main_level);
        if let Some(mut existing_level) = existing_level_opt {
            existing_level.queue_free();
        }

        // Setting
        self.main_level = main_level;
        if !self.base().is_node_ready() {
            return;
        }

        let main_menu_visible = self.main_level.is_none();
        self.main_menu_state_machine.set_visible(main_menu_visible);

        match self.main_level.clone() {
            Some(main_level) => {
                // Connect signals

                // MainLevel -> Run

                // request_exit_to_main_menu
                main_level
                    .signals()
                    .request_exit_to_main_menu()
                    .builder()
                    .flags(ConnectFlags::DEFERRED | ConnectFlags::ONE_SHOT)
                    .connect_other_mut(
                        self,
                        |me| {
                            me.set_main_level(None);
                        }
                    );
            },
            None => {
                let mut input = Input::singleton();
                input.set_mouse_mode(MouseMode::VISIBLE);

                // Reset to title
                let main_menu_state_machine = &mut self.main_menu_state_machine;
                main_menu_state_machine.clone().into_dyn().dyn_bind_mut().reset();
                main_menu_state_machine.bind_mut().set_state(MainMenuState::Title);
            },
        }
    }


    #[func]
    fn on_receive_maze(&mut self, maze : Gd<Maze>) {
        if self.main_level.is_some() {
            godot_warn!("Run got maze while level was spawned? Ignoring...");
            return;
        }

        // Else, spawn new

        let main_level_opt = self.main_level_scene.try_instantiate_as::<MainLevel>();
        let Some(mut main_level) = main_level_opt else {
            godot_error!("Failed instantiating main level!! Check that the scene is actually set to MainLevel!");
            return;
        };

        self.base_mut().add_child(&main_level.clone().upcast::<Node>());
        self.set_main_level(Some(main_level.clone()));

        main_level.bind_mut().set_maze(Some(maze));
    }
}
