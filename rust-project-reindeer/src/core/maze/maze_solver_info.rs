use godot::prelude::*;

use crate::core::maze::maze_tile_state::MazeTileState;


#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct MazeSolverInfo {
    #[export(flags = (
        Normal = MazeTileState::Normal.to_godot_flag(),
        Touched = MazeTileState::Touched.to_godot_flag(),
        Committed = MazeTileState::Committed.to_godot_flag(),
        Active = MazeTileState::Active.to_godot_flag()
    ))]
    #[var]
    #[init(val = MazeTileState::Active.to_godot_flag())]
    pub wait_on_state : u32,

    #[export]
    #[var]
    #[init(val = 0.02)]
    pub wait_delay : f64,

    #[export]
    #[var]
    #[init(val = 1000)]
    // Note: Rotation cost of 0 might make spinning in place repeatedly a viable tactic?
    pub rotation_cost : u32,

    base : Base<Resource>,
}
