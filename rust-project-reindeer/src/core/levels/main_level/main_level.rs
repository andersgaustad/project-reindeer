use godot::{classes::{AudioStreamPlayer, BoxMesh, BoxShape3D, CollisionShape3D, Input, InputEvent, Material, Mesh, MeshInstance3D, MultiMesh, MultiMeshInstance3D, RandomNumberGenerator, RichTextLabel, ShaderMaterial, StandardMaterial3D, Timer, Tween, base_material_3d::Flags, input::MouseMode, multi_mesh::TransformFormat, object::ConnectFlags}, prelude::*};
use strum::IntoEnumIterator;

use crate::{core::{common::{communicator::Communicator, convex_polygon::ConvexPolygon, coordinate::Coordinate, direction::Direction, i_add_padding::IAddPadding, i_generate_mail::IGenerateMail, padding::Padding}, environment::{enchanced_multi_mesh_instance_3d::EnchancedMultiMeshInstance3D, rock_type::RockType}, levels::{level_run_state::LevelRunState, main_level::pathfinding_state::PathfindingState}, maze::{maze::{Maze, Tile}, maze_solver_info::MazeSolverInfo, maze_tile_state::MazeTileState, path_info::PathInfo, reindeer::Reindeer}, options::option_change::OptionChange, player::Player, props::cabin::Cabin, run::{i_has_run::IHasRun, run::Run}, ui::{i_state::IState, pause_menu::{pause_menu_request::PauseMenuRequest, pause_menu_state_machine::PauseMenuStateMachine}}, utility::{bounding_box_utility, node_utility}}, input_map::UI_CANCEL};


const N_VISIBLE_TREES_IN_LOW_PERFORMANCE_MODE : i32 = 100;


#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct MainLevel {
    #[export_group(name = "Maze")]

    #[export]
    #[var]
    #[init(val = Color::from_html("#343c90").unwrap())]
    color_a : Color,

    #[export]
    #[var]
    #[init(val = Color::from_html("#1b88b6").unwrap())]
    color_b : Color,

    #[export]
    #[var]
    maze_top_left_corner : OnEditor<Gd<Node3D>>,

    #[export]
    #[var]
    maze_solver_info : OnEditor<Gd<MazeSolverInfo>>,

    #[export]
    #[var]
    arrow_shader_material : OnEditor<Gd<ShaderMaterial>>,

    #[export]
    #[var]
    #[init(val = 1.0)]
    arrow_pulse_frequency_factor : f32,

    #[export]
    #[var]
    #[init(val = true)]
    teleport_player_on_maze_set : bool,


    #[export_group(name = "Zones")]

    #[export]
    #[var]
    prop_root : OnEditor<Gd<Node3D>>,

    #[export]
    #[init(val = 30)]
    max_spawn_attempts_per_prop : i32,

    #[export(range=(0.0, 100.0, or_greater))]
    #[init(val = 14.0)]
    forecourt_depth : f32,

    #[export(range=(0.0, 10.0, or_greater))]
    #[init(val = 1.0)]
    maze_to_forecourt_padding : f32,

    #[export]
    clearing_ring : OnEditor<Gd<Padding>>,

    #[export]
    forest_ring : OnEditor<Gd<Padding>>,

    #[export(range=(0.0, 1.0, or_greater))]
    #[init(val = 1.0)]
    trees_per_square_unit : f32,

    #[export]
    #[var]
    #[init(val = 0)]
    outer_forest_rings : i32,

    #[export(range=(0.0, 1.0, or_greater))]
    #[init(val = 0.33)]
    snow_per_square_unit : f32,


    #[export_group(name = "Random")]

    #[export]
    #[var]
    #[init(val = "Reindeer".into())]
    random_seed : GString,


    #[export_group(name = "Mail")]

    #[export]
    #[var]
    #[init(val = default_mail_color_array())]
    mail_notification_colors : Array<Color>,

    #[export]
    #[var]
    #[init(val = 0.5)]
    mail_color_change_interval : f64,


    #[export_group(name = "Maze Parameters")]

    #[export]
    #[var]
    #[init(val = 1000)]
    turning_cost : u32,


    // Non-exported

    #[var]
    #[init(node = "%PauseMenuStateMachine")]
    pause_menu : OnReady<Gd<PauseMenuStateMachine>>,


    #[var]
    #[init(node = "%StartingInLabel")]
    starting_in_label : OnReady<Gd<RichTextLabel>>,

    #[var]
    #[init(node = "%CountdownToStartTimer")]
    countdown_to_start_timer : OnReady<Gd<Timer>>,

    #[var]
    #[init(node = "%MailMessageLabel")]
    mail_message_label : OnReady<Gd<RichTextLabel>>,
    default_mail_message_text : GString,
    mail_message_tween : Option<Gd<Tween>>,


    #[var]
    #[init(node = "%GroundMesh")]
    ground_mesh : OnReady<Gd<MeshInstance3D>>,

    #[var]
    #[init(node = "%Collision")]
    collision : OnReady<Gd<CollisionShape3D>>,

    #[var]
    #[init(node = "%Player")]
    player : OnReady<Gd<Player>>,

    #[var]
    #[init(node = "%Reindeer")]
    maze_reindeer : OnReady<Gd<Reindeer>>,

    #[var]
    #[init(node = "%GhostReindeer")]
    maze_ghost_reindeer : OnReady<Gd<Reindeer>>,

    #[var]
    #[init(node = "%SideForests")]
    side_forest_spawners_root : OnReady<Gd<Node3D>>,

    #[var]
    #[init(node = "%Present")]
    present : OnReady<Gd<Node3D>>,

    #[var]
    #[init(node = "%Cabin")]
    cabin : OnReady<Gd<Cabin>>,

    #[var]
    #[init(node = "%TileSpawner")]
    tile_spawner : OnReady<Gd<MultiMeshInstance3D>>,

    #[var]
    #[init(node = "%RockSmallSpawner")]
    rock_small_spawner : OnReady<Gd<EnchancedMultiMeshInstance3D>>,

    #[var]
    #[init(node = "%RockMediumSpawner")]
    rock_medium_spawner : OnReady<Gd<EnchancedMultiMeshInstance3D>>,

    #[var]
    #[init(node = "%RockLargeSpawner")]
    rock_large_spawner : OnReady<Gd<EnchancedMultiMeshInstance3D>>,

    #[var]
    #[init(node = "%ArrowSpawner")]
    arrow_spawner : OnReady<Gd<EnchancedMultiMeshInstance3D>>,

    #[var]
    #[init(node = "%TreeSpawner")]
    tree_spawner : OnReady<Gd<EnchancedMultiMeshInstance3D>>,

    #[var]
    #[init(node = "%SnowPileSpawner")]
    snow_pile_spawner : OnReady<Gd<MultiMeshInstance3D>>,

    #[var]
    #[init(node = "%SnowFlatSpawner")]
    snow_flats_spawner : OnReady<Gd<MultiMeshInstance3D>>,

    #[var]
    #[init(node = "%SnowBunkerSpawner")]
    snow_bunker_spawner : OnReady<Gd<MultiMeshInstance3D>>,

    #[var]
    #[init(node = "%BackgroundMusicPlayer")]
    background_music_player : OnReady<Gd<AudioStreamPlayer>>,
    default_background_music_player_volume : f32,

    #[var]
    #[init(node = "%MailNotificationSFXPlayer")]
    mail_notification_sfx_player : OnReady<Gd<AudioStreamPlayer>>,
    default_mail_notification_sfx_player_volume : f32,


    #[var(get, set = set_maze)]
    maze : Option<Gd<Maze>>,



    #[var(get, set = set_level_run_state)]
    #[init(val = LevelRunState::Running)]
    level_run_state : LevelRunState,

    #[var(get, set = set_pathfinding_state)]
    #[init(val = PathfindingState::NotStarted)]
    pathfinding_state : PathfindingState,


    rng : Gd<RandomNumberGenerator>,


    run : Option<Gd<Run>>,


    base : Base<Node3D>,
}


#[godot_api]
impl INode3D for MainLevel {
    fn ready(&mut self) {
        let gd = self.to_gd();

        // Run
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

        
        // PauseMenu:
        let pause_menu_state_machine = self.pause_menu.clone();
        let pause_menu_face = pause_menu_state_machine.bind().get_face_pause_menu();

        // MainLevel -> PauseMenu
        
        // on_level_pathfinding_state_update
        self
            .signals()
            .notify_pathfinding_state_update()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_gd(
                &pause_menu_face,
                |mut pause_menu, old_state, new_state| {
                    pause_menu.bind_mut().on_level_pathfinding_state_update(old_state, new_state);
                }
            );


        // PauseMenu -> MainLevel

        // on_pause_menu_button_pressed
            self
                .pause_menu
                .signals()
                .request()
                .builder()
                .flags(ConnectFlags::DEFERRED)
                .connect_other_mut(
                    self,
                    Self::on_pause_menu_request
                );
        
        // Reset arrow count
        // Needed as arrows persist even after main level is freed?
        if let Some(mut arrow_spawner) = self.arrow_spawner.get_multimesh() {
            arrow_spawner.set_instance_count(0);
        }

        self.default_mail_message_text = self.mail_message_label.get_text();
        self.default_background_music_player_volume = self.background_music_player.get_volume_linear();
        self.default_mail_notification_sfx_player_volume = self.background_music_player.get_volume_linear();
        
        self.refresh();
    }


    fn process(&mut self, _delta : f64) {
        if self.pathfinding_state == PathfindingState::Countdown {
            let time_left = self.countdown_to_start_timer.get_time_left();

            let string = format!("Starting in {:>5.2}s", time_left);
            self.starting_in_label.set_text(&string);
        }
    }


    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if self.level_run_state != LevelRunState::Running {
            return;
        }

        // Cancel
        if event.is_action_pressed(UI_CANCEL) {
            let viewport_opt = self.base().get_viewport();
            if let Some(mut viewport) = viewport_opt {
                viewport.set_input_as_handled();
            }
            self.set_level_run_state(LevelRunState::Paused);
            
            return;
        }
    }
}


impl IHasRun for MainLevel {
    fn get_run(&self) -> Option<Gd<Run>> {
        self.run.clone()
    }
}


#[godot_dyn]
impl IState for MainLevel {
    fn enter(&mut self) {
        self.base_mut().set_process_unhandled_input(true);

        self.background_music_player.play();

        self.refresh();
    }


    fn exit(&mut self) {
        self.base_mut().set_process_unhandled_input(false);

        self.background_music_player.stop();
    }
}


#[godot_api]
impl MainLevel {
    #[signal]
    pub fn notify_level_run_state_update(old : LevelRunState, new : LevelRunState);

    #[signal]
    pub fn notify_pathfinding_state_update(old : PathfindingState, new : PathfindingState);

    #[signal]
    pub fn request_exit_to_main_menu();


    #[func]
    pub fn set_level_run_state(&mut self, new_state : LevelRunState) {
        let Some(mut tree) = self.base().get_tree() else {
            return;
        };

        let previous_state = self.level_run_state;

        // Set
        self.level_run_state = new_state;

        let paused = new_state == LevelRunState::Paused;

        let mut pause_menu = self.pause_menu.clone().into_dyn::<dyn IState>();
        if paused {
            pause_menu.dyn_bind_mut().enter();
        } else {
            pause_menu.dyn_bind_mut().exit();
        }

        let mouse_mode = if paused {
            MouseMode::VISIBLE
        } else {
            MouseMode::CAPTURED
        };

        let mut input = Input::singleton();
        input.set_mouse_mode(mouse_mode);

        tree.set_pause(paused);

        // Notify update
        if previous_state != new_state {
            self
                .signals()
                .notify_level_run_state_update()
                .emit(previous_state, new_state);
        }
    }


    #[func]
    pub fn set_pathfinding_state(&mut self, new_state : PathfindingState) {
        let previous_state = self.pathfinding_state;

        // Set
        self.pathfinding_state = new_state;

        match new_state {
            PathfindingState::NotStarted => {
                // Do nothing
            },
            PathfindingState::Countdown => {
                // Only start countdown if we get here from NotStarted
                if previous_state == PathfindingState::NotStarted {
                    self.starting_in_label.show();
                    self.countdown_to_start_timer.start();

                    self
                        .countdown_to_start_timer
                        .signals()
                        .timeout()
                        .connect_other(
                            self,
                            |me| {
                                me.set_pathfinding_state(PathfindingState::InProgress);
                            }
                        );
                    
                }
            },
            PathfindingState::InProgress => {
                self.starting_in_label.hide();
                self.countdown_to_start_timer.stop();
                
                self.run_maze_solver();
            },
            PathfindingState::Done => {
                // Do nothing
            },
        }

        if previous_state != new_state {
            self
                .signals()
                .notify_pathfinding_state_update()
                .emit(previous_state, new_state);
        }
    }


    #[func]
    pub fn set_maze(&mut self, maze_opt : Option<Gd<Maze>>) {
        // Set
        self.maze = maze_opt;

        if !self.base().is_node_ready() {
            return;
        }

        let multimesh_opt = self.tile_spawner.get_multimesh();
        let Some(mut tile_multimesh) = multimesh_opt else {
            return;
        };
        let Some(mut tile_mesh) = tile_multimesh.get_mesh() else {
            return;
        };

        let Some(mut small_rock_multimesh) = self.rock_small_spawner.get_multimesh() else {
            return;
        };
        let Some(mut medium_rock_multimesh) = self.rock_medium_spawner.get_multimesh() else {
            return;
        };
        let Some(mut large_rock_multimesh) = self.rock_large_spawner.get_multimesh() else {
            return;
        };


        if let Some(maze) = self.maze.clone() {
            let bound_maze = maze.bind();
            let dim_x = bound_maze.rust_get_dim_x();
            let dim_y = bound_maze.rust_get_dim_y();

            let mut small_rock_positions = Vec::new();
            let mut medium_rock_positions = Vec::new();
            let mut large_rock_positions = Vec::new();

            let mut material = StandardMaterial3D::new_gd();
            material.set_albedo(Color::WHITE);
            material.set_flag(Flags::ALBEDO_FROM_VERTEX_COLOR, true);

            tile_mesh.surface_set_material(0, &material);

            let dim_x_i32 = dim_x as i32;
            let dim_y_i32 = dim_y as i32;

            let n_tiles = dim_x_i32 * dim_y_i32;

            tile_multimesh.set_instance_count(0);

            tile_multimesh.set_transform_format(TransformFormat::TRANSFORM_3D);
            tile_multimesh.set_use_colors(true);

            tile_multimesh.set_instance_count(n_tiles);

            let tile_size = tile_mesh.get_aabb().size;
            let tile_height = tile_size.y;

            let dim_x_f32 = dim_x as f32;
            let dim_y_f32 = dim_y as f32;

            let bounding_box_size = Vector2::new(
                dim_x_f32 * tile_size.x,
                dim_y_f32 * tile_size.z
            );

            let maze_top_left_corner_center_position = self.maze_top_left_corner.get_position();
            let maze_origin_position = {
                let mut center = maze_top_left_corner_center_position;
                center.x -= tile_size.x / 2.0;
                center.z -= tile_size.z / 2.0;

                center
            };

            let maze_top_left_corner_position_from_above = Vector2::new(maze_origin_position.x, maze_origin_position.z);


            // Zones

            let maze_span = Rect2::new(maze_top_left_corner_position_from_above, bounding_box_size);

            let forecourt_position = Vector2::new(
                maze_top_left_corner_position_from_above.x,
                maze_top_left_corner_position_from_above.y + maze_span.size.y + self.maze_to_forecourt_padding
            );
            let forecourt_size = Vector2::new(
                maze_span.size.x,
                self.forecourt_depth
            );
            let forecourt_span = Rect2::new(forecourt_position, forecourt_size);

            let inner_span = Rect2::from_corners(
                maze_span.position,
                forecourt_span.end()
            );

            let clearing_span = inner_span.grow_with_padding(self.clearing_ring.clone());
            let forest_span = clearing_span.grow_with_padding(self.forest_ring.clone());


            // Spawn (or move) cabin

            let cabin_aabb = bounding_box_utility::get_node_aabb_ex()
                .node(Some(self.cabin.clone().upcast()))
                .build()
                .done();

            let position = cabin_aabb.position;
            let center_3d = cabin_aabb.center();
            let center_2d = Vector2::new(center_3d.x, center_3d.z);
            let end = cabin_aabb.end();

            let top_down_start = Vector2::new(position.x, position.z);
            let top_down_end = Vector2::new(end.x, end.z);

            let top_right = Vector2::new(top_down_end.x, top_down_start.y);
            let bottom_left = Vector2::new(top_down_start.x, top_down_end.y);

            let width = cabin_aabb.size.x;
            let height = cabin_aabb.size.y;

            let corners = [
                top_down_start,
                top_right,
                top_down_end,
                bottom_left,
            ];
            

            let cabin_polygon_opt = (|| {
                let extent = (width + height) * std::f32::consts::SQRT_2 / 4.0;

                // Else, should be able to spawn

                let middle_of_forecourt = forecourt_span.center();

                let mut point = Vector2::new(extent, extent) + forecourt_position;
                let mut rotation = std::f32::consts::FRAC_PI_4;

                let flip_against_y = self.rng.randi() % 2 == 0;
                let flip_against_x = self.rng.randi() % 2 == 0;

                if flip_against_y {
                    let mut relative = point - middle_of_forecourt;
                    relative.x *= -1.0;
                    point = relative + middle_of_forecourt;

                    rotation *= -1.0;
                }

                if flip_against_x {
                    let mut relative = point - middle_of_forecourt;
                    relative.y *= -1.0;
                    point = relative + middle_of_forecourt;

                    rotation = std::f32::consts::PI - rotation;
                }

                let cabin_position = Vector3::new(
                    point.x,
                    maze_top_left_corner_center_position.y,
                    point.y
                );
                
                let cabin = &mut self.cabin;
                cabin.set_position(cabin_position);
                cabin.set_rotation(Vector3::new(0.0, rotation, 0.0));
                cabin.show();

                let rotated_corners = corners.map(|position| {
                    (position - center_2d).rotated(rotation) + point
                });

                let polygon = ConvexPolygon::try_from_points(rotated_corners.into_iter().collect());
                polygon
            })();

            let mut spawned_props = Vec::new();

            #[cfg(debug_assertions)]
            {
                if let Some(cabin_polygon) = cabin_polygon_opt {
                    spawned_props.push(cabin_polygon);
                    godot_print!("Spawned cabin!");

                } else {
                    godot_print!("Failed spawning cabin");
                }
            }
            

            // Spawn snow props

            let props = self.prop_root.get_children();
            for prop in props.iter_shared() {
                let Ok(mut prop_3d) = prop.try_cast::<Node3D>() else {
                    continue;
                };
                let aabb = bounding_box_utility::get_node_aabb_ex()
                    .node(Some(prop_3d.clone()))
                    .build()
                    .done();

                let center_3d = aabb.center();
                let center_2d = Vector2::new(center_3d.x, center_3d.z);
                let position_2d = Vector2::new(aabb.position.x, aabb.position.z);
                let size_2d = Vector2::new(aabb.size.x, aabb.size.z);
                let end_2d = position_2d + size_2d;

                let corners = [
                    position_2d,
                    Vector2::new(end_2d.x, position_2d.y),
                    end_2d,
                    Vector2::new(position_2d.x, end_2d.y),
                ];

                // Attempt k times for each prop
                for _ in 0..self.max_spawn_attempts_per_prop {
                    let x = self.rng.randf_range(0.0, forecourt_size.x);
                    let y = self.rng.randf_range(0.0, forecourt_size.y);
                    let rotation = self.rng.randf_range(0.0, std::f32::consts::TAU);

                    let point = Vector2::new(x, y) + forecourt_position;
                    let rotated_corners = corners
                        .into_iter()
                        .map(|corner| {
                            (corner - center_2d).rotated(rotation) + point
                        })
                        .collect::<Vec<_>>();

                    let all_inside_forecourt = rotated_corners
                        .iter()
                        .all(|corner| {
                            forecourt_span.contains_point(*corner)
                        });
                    
                    if !all_inside_forecourt {
                        continue;
                    }

                    let Some(polygon) = ConvexPolygon::try_from_points(rotated_corners) else {
                        continue;
                    };

                    // Note - this causes O(n^2) time
                    let any_overlap = spawned_props
                        .iter()
                        .any(|existing_polygon| {
                            existing_polygon.overlaps_with(&polygon)
                        });
                    
                    if any_overlap {
                        continue;
                    }

                    // Else, spawn is safe
                    let position_3d = Vector3::new(
                        point.x,
                        maze_top_left_corner_center_position.y,
                        point.y
                    );

                    prop_3d.set_position(position_3d);
                    prop_3d.set_rotation(Vector3::new(0.0, rotation, 0.0));
                    prop_3d.show();

                    spawned_props.push(polygon);
                    break;
                }
            }

            // Spawn snow
            let n_expected_snow_piles = (self.snow_per_square_unit * forecourt_size.length()) as i32;

            const N_BARS : usize = 3 - 1;
            let mut bars = Vec::with_capacity(N_BARS);
            for _ in 0..N_BARS {
                let bar_index = self.rng.randi_range(0, n_expected_snow_piles);
                bars.push(bar_index);
            }

            bars.sort();

            
            let n_snow_piles = bars[0];
            let n_snow_flats = bars[1] - n_snow_piles;
            let n_snow_bunkers = n_expected_snow_piles - n_snow_piles - n_snow_flats;

            let spawners_and_spawn_amouns = [
                (self.snow_pile_spawner.clone(), n_snow_piles),
                (self.snow_flats_spawner.clone(), n_snow_flats),
                (self.snow_bunker_spawner.clone(), n_snow_bunkers),
            ];

            for (spawner, amount) in spawners_and_spawn_amouns {
                let Some(mut multimesh) = spawner.get_multimesh() else {
                    continue;
                };

                let Some(mesh) = multimesh.get_mesh() else {
                    continue;
                };

                let aabb = mesh.get_aabb();
                let center_3d = aabb.center();
                let center_2d = Vector2::new(center_3d.x, center_3d.z);
                let position_2d = Vector2::new(aabb.position.x, aabb.position.y);
                let size_2d = Vector2::new(aabb.size.x, aabb.size.y);
                let end_2d = position_2d + size_2d;

                let corners = [
                    position_2d,
                    Vector2::new(end_2d.x, position_2d.y),
                    end_2d,
                    Vector2::new(position_2d.x, end_2d.y),
                ];

                multimesh.set_instance_count(0);
                multimesh.set_transform_format(TransformFormat::TRANSFORM_3D);

                let mut transforms = Vec::new();

                for _ in 0..amount {
                    let x = self.rng.randf_range(0.0, forecourt_size.x);
                    let y = self.rng.randf_range(0.0, forecourt_size.y);
                    let rotation = self.rng.randf_range(0.0, std::f32::consts::TAU);

                    let point = Vector2::new(x, y) + forecourt_position;
                    let position_3d = Vector3::new(
                        point.x,
                        maze_top_left_corner_center_position.y,
                        point.y
                    );

                    let rotated_corners = corners
                        .iter()
                        .map(|corner| {
                            (*corner - center_2d).rotated(rotation) + point
                        })
                        .collect::<Vec<_>>();

                    let all_inside_forecourt = rotated_corners
                        .iter()
                        .all(|corner| {
                            forecourt_span.contains_point(*corner)
                        });
                    
                    if !all_inside_forecourt {
                        continue;
                    }

                    let Some(polygon) = ConvexPolygon::try_from_points(rotated_corners) else {
                        continue;
                    };

                    let any_collision = spawned_props
                        .iter()
                        .any(|existing_polygon| {
                            polygon.overlaps_with(existing_polygon)
                        });
                    
                    if any_collision {
                        continue;
                    }

                    // Ok to spawn
                    let basis = Basis::from_axis_angle(Vector3::UP, rotation);
                    let transform = Transform3D::new(basis, position_3d);

                    spawned_props.push(polygon);
                    transforms.push(transform);
                }

                let length = transforms.len() as i32;
                multimesh.set_instance_count(length);
                for (i, transform) in transforms.into_iter().enumerate() {
                    let i = i as i32;

                    multimesh.set_instance_transform(i, transform);
                }
            }


            // Create maze

            let mut x = 0;
            let mut y = 0;
            for i in 0..n_tiles {
                if x >= dim_x_i32 {
                    x = 0;
                    y += 1;
                }

                // Base

                let position = Self::get_tile_position_from_cached(x, y, &tile_mesh);

                let pos_x = position.coordinates.x;
                let pos_y = position.coordinates.y;

                let vector = Vector3::new(pos_x, 0.0, pos_y) + maze_top_left_corner_center_position;
                let basis = Basis::default();
                let transform = Transform3D::new(basis, vector);

                let color = if (x + y) % 2 == 0 { self.color_a } else { self.color_b };

                tile_multimesh.set_instance_transform(i, transform);
                tile_multimesh.set_instance_color(i, color);

                // Wall?

                let array = bound_maze.rust_get_array();
                let tile_opt = (|| {
                    let index_usize = usize::try_from(i).ok()?;
                    let tile = array.get(index_usize);

                    tile
                })();

                if let Some(tile) = tile_opt {
                    if tile == &Tile::Wall {
                        let rock_type = RockType::get_random(self.rng.clone());
                        let rock_array = match rock_type {
                            RockType::Small => &mut small_rock_positions,
                            RockType::Medium => &mut medium_rock_positions,
                            RockType::Large => &mut large_rock_positions,
                        };

                        rock_array.push(vector);
                    }
                };

                x += 1;
            }

            // Initialize rock spawners
            let rock_multimeshes_and_positions = [
                (&mut self.rock_small_spawner, &mut small_rock_multimesh, small_rock_positions),
                (&mut self.rock_medium_spawner, &mut medium_rock_multimesh, medium_rock_positions),
                (&mut self.rock_large_spawner, &mut large_rock_multimesh, large_rock_positions),
            ];

            // Spawn all rocks
            for (spawner, multimesh, positions) in rock_multimeshes_and_positions {
                let n_rocks_of_type = positions.len();
                multimesh.set_instance_count(0);
                multimesh.set_transform_format(TransformFormat::TRANSFORM_3D);
                multimesh.set_instance_count(n_rocks_of_type as i32);

                for (i, mut position) in positions.into_iter().enumerate() {
                    position.y += tile_height;

                    let transform = spawner.bind().create_object_transform(position, self.rng.clone());

                    multimesh.set_instance_transform(i as i32, transform);
                }
            }

            // Reindeer

            let reindeer_start = maze.bind().rust_get_reindeer_start_coordinate().clone();
            let x = reindeer_start.x as i32;
            let y = reindeer_start.y as i32;

            let position_info = Self::get_tile_position_from_cached(
                x,
                y,
                &tile_mesh
            );

            let position = Vector3::new(
                position_info.coordinates.x,
                position_info.height,
                position_info.coordinates.y

            ) + maze_top_left_corner_center_position;

            let reindeer = &mut self.maze_reindeer;
            reindeer.set_position(position);
            reindeer.bind_mut().set_reindeer_rotation(Direction::North);
            reindeer.show();


            // Present

            let present_coordinate = maze.bind().rust_get_end_coordinate().clone();
            let x = present_coordinate.x as i32;
            let y = present_coordinate.y as i32;

            let position_info = Self::get_tile_position_from_cached(
                x,
                y,
                &tile_mesh
            );

            let position = Vector3::new(
                position_info.coordinates.x,
                position_info.height,
                position_info.coordinates.y

            ) + maze_top_left_corner_center_position;

            let present = &mut self.present;
            present.set_position(position);
            present.show();


            // Teleport player if enabled
            if self.teleport_player_on_maze_set {
                let forecourt_center = forecourt_span.center();
                let mut player_position = self.player.get_position();
                
                player_position.x = forecourt_center.x;
                player_position.z = forecourt_center.y;

                self.player.set_position(player_position);
            }


            // Create surrounding forest
            // Note that this is done here after everything else as we want the same seed to generate the same cabin and prop variants regardless of forest size

            let forest_position = &forest_span.position;
            let forest_size = &forest_span.size;
            let n_tree_spawn_attempts = (forest_size.x * &forest_size.y * self.trees_per_square_unit) as usize;

            let tree_multimesh_opt =self.tree_spawner.get_multimesh();

            let aabb_tree_height = (|| {
                let height = tree_multimesh_opt.as_ref()?.get_mesh()?.get_aabb().size.y;
                Some(height)

            })().unwrap_or(1.0);
            let forest_custom_aabb = Aabb::new(
                Vector3::new(
                    forest_position.x,
                    maze_top_left_corner_center_position.y,
                    forest_position.y
                ),
                Vector3::new(
                    forest_size.x,
                    aabb_tree_height,
                    forest_size.y
                )
            );

            self.tree_spawner.set_custom_aabb(forest_custom_aabb);

            let Some(mut tree_multimesh) = tree_multimesh_opt else {
                godot_error!("No tree multimesh found??");
                return;
            };

            // Get tree positions

            let mut tree_spawn_positions = Vec::with_capacity(n_tree_spawn_attempts);

            for _ in 0..n_tree_spawn_attempts {
                let x = self.rng.randf_range(0.0, forest_size.x);
                let y = self.rng.randf_range(0.0, forest_size.y);

                let local_point = Vector2::new(x, y);

                let point = local_point + forest_span.position;

                // Only add if this point is in the forest but not in clearing
                if clearing_span.contains_point(point) {
                    continue;
                }

                // Else
                tree_spawn_positions.push(point);
            }

            // Spawn main forest

            let bound_tree_spawner = self.tree_spawner.bind();

            let n_trees = tree_spawn_positions.len() as i32;
            tree_multimesh.set_instance_count(n_trees);
            for (i, top_down_position) in tree_spawn_positions.into_iter().enumerate() {
                let i = i as i32;
                let position = Vector3::new(
                    top_down_position.x,
                    maze_top_left_corner_center_position.y,
                    top_down_position.y
                );

                let transform = bound_tree_spawner.create_object_transform(position, self.rng.clone());
                tree_multimesh.set_instance_transform(i, transform);
            }
            drop(bound_tree_spawner);


            // Spawn side forests

            // Delete old side forests
            for mut old_side_forest in self.side_forest_spawners_root.get_children().iter_shared() {
                old_side_forest.queue_free();
            }

            let forest_rings = self.outer_forest_rings;

            let super_forest_position = *forest_position - (*forest_size * (forest_rings as f32));
            let super_forest_size = (forest_rings * 2 + 1) as f32 * *forest_size;

            let super_forest_span = Rect2::new(super_forest_position, super_forest_size);

            self.set_ground_dimensions(super_forest_span);

            // Spawn new forests
            for x_major in -forest_rings..=forest_rings {
                for y_major in -forest_rings..=forest_rings {
                    if x_major == 0 && y_major == 0 {
                        continue;
                    }

                    let x_major = x_major as f32;
                    let y_major = y_major as f32;

                    let x = x_major * forest_size.x;
                    let y = y_major * forest_size.y;

                    let side_forest_position = *forest_position + Vector2::new(x, y);

                    let tree_mesh_opt = tree_multimesh.get_mesh();
                    let mut tree_multimesh = MultiMesh::new_gd();

                    tree_multimesh.set_instance_count(0);
                    tree_multimesh.set_transform_format(TransformFormat::TRANSFORM_3D);
                    tree_multimesh.set_mesh(tree_mesh_opt.as_ref());

                    let side_forest_spawner_opt = (|| {
                        self.tree_spawner.duplicate()?.try_cast::<EnchancedMultiMeshInstance3D>().ok()
                    })();

                    let Some(mut forest_spawner) = side_forest_spawner_opt else {
                        continue;
                    };

                    forest_spawner.set_multimesh(&tree_multimesh);

                    let mut side_forest_custom_aabb = forest_custom_aabb.clone();
                    side_forest_custom_aabb.position.x = side_forest_position.x;
                    side_forest_custom_aabb.position.z = side_forest_position.y;

                    forest_spawner.set_custom_aabb(side_forest_custom_aabb);


                    // Spawn
                    self.side_forest_spawners_root.add_child(&forest_spawner);

                    let n_tree_spawn_attempts = n_tree_spawn_attempts as i32;
                    tree_multimesh.set_instance_count(n_tree_spawn_attempts);

                    for i in 0..n_tree_spawn_attempts {
                        let x = self.rng.randf_range(0.0, forest_size.x) + side_forest_position.x;
                        let y = self.rng.randf_range(0.0, forest_size.y) + side_forest_position.y;

                        let position_3d = Vector3::new(
                            x,
                            maze_top_left_corner_center_position.y,
                            y
                        );

                        let transform = forest_spawner.bind().create_object_transform(
                            position_3d,
                            self.rng.clone()
                        );

                        tree_multimesh.set_instance_transform(i, transform);
                    }
                }
            }
            
        } else {
            // Reset
            let multimeshes = [
                &mut tile_multimesh,
                &mut small_rock_multimesh,
                &mut medium_rock_multimesh,
                &mut large_rock_multimesh,
            ];

            for multimesh in multimeshes {
                multimesh.set_instance_count(0);
            }

            self.maze_reindeer.hide();
            self.present.hide();
        }

        // Finally, refresh options to apply low performance mode
        // Not using refresh() here as that would lead to an unfortunate infinite loop
        self.on_low_performance_mode_change();
    }


    fn set_ground_dimensions(&mut self, span : Rect2) {
        // Mesh

        let mut box_mesh = (|| {
            self
                .ground_mesh
                .get_mesh()?
                .try_cast::<BoxMesh>()
                .ok()

        })().unwrap_or_default();

        let span_size = span.size;

        let mut mesh_size = box_mesh.get_size();
        mesh_size.x = span_size.x;
        mesh_size.z = span_size.y;

        box_mesh.set_size(mesh_size);
        self.ground_mesh.set_mesh(&box_mesh);

        let top_down_position = span.position + (span_size / 2.0);
        let mut ground_position = self.ground_mesh.get_position();
        ground_position.x = top_down_position.x;
        ground_position.z = top_down_position.y;

        self.ground_mesh.set_position(ground_position);


        // Collision, copied from mesh

        let mut box_collision = (|| {
            self
                .collision
                .get_shape()?
                .try_cast::<BoxShape3D>()
                .ok()

        })().unwrap_or_default();

        box_collision.set_size(mesh_size);

        self.collision.set_position(ground_position);
    }


    #[func]
    fn on_options_changed(&mut self, options_change : OptionChange) {
        match options_change {
            OptionChange::LowPerformanceMode => self.on_low_performance_mode_change(),
            OptionChange::VolumeChange => self.on_volume_change(),
        }
    }


    #[func]
    fn on_low_performance_mode_change(&mut self) {
        let Some(options) = self.get_options() else {
            return;
        };

        let low_performance_mode = options.bind().get_low_performance_mode();

        let mut forest_spawners = vec![self.tree_spawner.clone().upcast::<MultiMeshInstance3D>()];
        let side_forests = self.side_forest_spawners_root.get_children();
        for child in side_forests.iter_shared() {
            let as_multimesh_instance_result = child.try_cast::<MultiMeshInstance3D>();
            if let Ok(as_multimesh_instance) = as_multimesh_instance_result {
                forest_spawners.push(as_multimesh_instance);
            }
        }

        let desired_visible_count = self.get_number_of_visible_trees();
        for spawner in forest_spawners {
            let Some(mut tree_multimesh) = spawner.get_multimesh() else {
                continue;
            };

            let current_tree_count = tree_multimesh.get_instance_count();
            if desired_visible_count <= current_tree_count {
                tree_multimesh.set_visible_instance_count(desired_visible_count);
            }
        }

        self.cabin.bind_mut().toggle_effects(!low_performance_mode);
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
            (self.mail_notification_sfx_player.clone(), self.default_mail_notification_sfx_player_volume)
        ];

        let bound_options = options.bind();
        let music_volume_factor = bound_options.get_music_volume();
        let sfx_volume_factor = bound_options.get_sfx_volume();
        drop(bound_options);

        let components_and_default_factors = [
            (&mut music, music_volume_factor),
            (&mut sfx, sfx_volume_factor) 
        ];

        for (item, volume_factor) in components_and_default_factors {
            for (component, default_factor) in item {
                let volume = volume_factor * *default_factor;
                component.set_volume_linear(volume);
            }
        }
    }


    #[func]
    fn on_pause_menu_request(&mut self, request : PauseMenuRequest) {
        if self.level_run_state != LevelRunState::Paused {
            return;
        }

        match request {
            PauseMenuRequest::Start => {
                self.set_level_run_state(LevelRunState::Running);
                self.set_pathfinding_state(PathfindingState::Countdown);
            },
            PauseMenuRequest::Resume => {
                self.set_level_run_state(LevelRunState::Running);
            },
            PauseMenuRequest::ToMainMenu => {
                // TODO Ask for confirmation
                self.set_level_run_state(LevelRunState::Running);
                self
                    .signals()
                    .request_exit_to_main_menu()
                    .emit();
            },
        }
    }


    #[func]
    fn on_maze_update_idx(&mut self, idx : i32, state : MazeTileState, direction : Direction, acknowledger : Gd<Communicator>) {
        let Some(mut multimesh) = self.tile_spawner.get_multimesh() else {
            godot_error!("Expected multimesh!");
            return;
        };

        match state {
            MazeTileState::Normal => {
                multimesh.set_instance_color(idx, Color::WHITE);
            },
            MazeTileState::Touched => {
                multimesh.set_instance_color(idx, Color::LIGHT_YELLOW);
            },
            MazeTileState::Committed => {
                multimesh.set_instance_color(idx, Color::ORANGE);
            },
            MazeTileState::Active => {
                multimesh.set_instance_color(idx, Color::GREEN);

                if let Some(maze) = self.maze.clone() {
                    if let Ok(idx_usize) = usize::try_from(idx) {
                        let dim_x = maze.bind().rust_get_dim_x();
                        let coordinate_opt = Coordinate::try_from_value(idx_usize, dim_x);
                        if let Some(coordinate) = coordinate_opt {
                            let x = coordinate.x as i32;
                            let y = coordinate.y as i32;
                            let tile_center_position_opt = self.get_tile_position(x, y);
                            if let Some(tile_center_position) = tile_center_position_opt {
                                let TileCenterPosition {
                                    coordinates : position,
                                    height
                                } = tile_center_position;

                                let position_3d = Vector3::new(
                                    position.x,
                                    height,
                                    position.y

                                ) + self.maze_top_left_corner.get_position();

                                // Move ghost reindeer
                                let ghost_reindeer = &mut self.maze_ghost_reindeer;
                                ghost_reindeer.set_position(position_3d);
                                ghost_reindeer.bind_mut().set_reindeer_rotation(direction);
                                ghost_reindeer.show();
                            }
                        }
                    }
                }
            },
        }

        let delay = self.maze_solver_info.bind().wait_delay;
        let mut scene_tree = self.base().get_tree().expect("Failed getting scene tree??");
        let timer = scene_tree.create_timer(delay).expect("Failed creating timer??");

        timer
            .signals()
            .timeout()
            .builder()
            .flags(ConnectFlags::DEFERRED | ConnectFlags::ONE_SHOT)
            .connect_other_gd(
                &acknowledger,
                |ack| {
                    ack.signals().done().emit();
                }
            );
    }


    #[func]
    fn on_maze_commit_found_path(&mut self, path_info_opt : Option<Gd<PathInfo>>) {
        self.set_pathfinding_state(PathfindingState::Done);

        self.maze_ghost_reindeer.hide();

        let mail = path_info_opt.generate_mail();
        self.send_mail_to_letter_menu(mail);

        if let Some(path_info) = &path_info_opt {
            let bound = path_info.bind();
            let paths = bound.rust_get_paths();

            let empty = Vec::new();
            let first_path = paths.first().unwrap_or(&empty);

            let n_arrows = first_path.len().checked_sub(1).unwrap_or(0);

            let arrow_pulse_frequency = self.arrow_pulse_frequency_factor / (n_arrows as f32);


            // Configure arrow spawner

            self.arrow_spawner.set_material_overlay(Some(&self.arrow_shader_material.clone().upcast::<Material>()));

            let Some(mut arrow_multimesh) = self.arrow_spawner.get_multimesh() else {
                godot_error!("Could not find Arrow Spawner MultiMesh!");
                return;
            };

            arrow_multimesh.set_use_custom_data(true);
            arrow_multimesh.set_instance_count(0);

            arrow_multimesh.set_transform_format(TransformFormat::TRANSFORM_3D);
            arrow_multimesh.set_use_colors(true);

            arrow_multimesh.set_instance_count(n_arrows.try_into().unwrap());

            let mut material = StandardMaterial3D::new_gd();
            material.set_albedo(Color::WHITE);
            material.set_flag(Flags::ALBEDO_FROM_VERTEX_COLOR, true);

            let Some(mut arrow_mesh) = arrow_multimesh.get_mesh() else {
                godot_error!("Could not get Arrow Mesh!");
                return;
            };

            arrow_mesh.surface_set_material(0, &material);

            let maze_top_left_corner_offset = self.maze_top_left_corner.get_position();

            for i in 0..n_arrows {
                let Some(current) = first_path.get(i) else {
                    break;
                };

                let Some(next) = first_path.get(i + 1) else {
                    break;
                };

                let x = current.x;
                let y = current.y;

                let Some(tile_position) = self.get_tile_position(
                    x.try_into().unwrap(),
                    y.try_into().unwrap()
                
                ) else {
                    continue;
                };

                let position_coordinates = tile_position.coordinates;
                let tile_height = tile_position.height;

                let position = Vector3::new(
                    position_coordinates.x,
                    tile_height,
                    position_coordinates.y

                ) + maze_top_left_corner_offset;

                let mut transform = self.arrow_spawner.bind().create_object_transform(position, self.rng.clone());

                let direction_from_current_to_next_opt = next.try_get_direction_of_other(current);
                if let Some(direction_from_current_to_next) = direction_from_current_to_next_opt {
                    let alignment = self.arrow_spawner.bind().rust_get_mesh_directional_alignment();
                    let rotations = alignment.counter_clockwise_rotations_to(&direction_from_current_to_next);

                    let rotations_in_radians = std::f32::consts::FRAC_PI_2 * (rotations as f32);

                    let new_basis = transform.basis.rotated(Vector3::UP, rotations_in_radians);
                    transform.basis = new_basis;
                }

                let i_i32 = i32::try_from(i).unwrap();

                let mut custom_data = Color::BLACK;
                custom_data.r = ((n_arrows - i) as f32) * arrow_pulse_frequency;

                arrow_multimesh.set_instance_transform(i_i32, transform);
                arrow_multimesh.set_instance_custom_data(i_i32, custom_data);
            }
        };
    }


    #[func]
    fn on_mail_tween_finished(&mut self) {
        let tween_opt = std::mem::take(&mut self.mail_message_tween);
        if let Some(mut tween) = tween_opt {
            tween.kill();
        }

        self.mail_message_label.hide();
    }


    fn refresh(&mut self) {
        let seed = self.random_seed.hash_u32();
        let seed_u64 = u64::from(seed);
        self.rng.set_seed(seed_u64);

        self.maze_reindeer.hide();
        self.maze_ghost_reindeer.hide();

        let level_run_state = self.level_run_state.clone();
        self.set_level_run_state(level_run_state);

        let pathfinding_state = self.pathfinding_state.clone();
        self.set_pathfinding_state(pathfinding_state);

        let maze = std::mem::take(&mut self.maze);
        self.set_maze(maze);

        for possible_option_change in OptionChange::iter() {
            self.on_options_changed(possible_option_change);
        }
    }


    #[func]
    fn run_maze_solver(&mut self) {
        let Some(mut maze) = self.maze.clone() else {
            godot_warn!("Tried running maze solver without maze? Returning...");
            return;
        };

        let maze_solver_info = self.maze_solver_info.clone();
        let mut handle = maze.bind_mut().find_paths(maze_solver_info, self.turning_cost);

        
        // Level -> Communicator

        self
            .signals()
            .notify_level_run_state_update()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_gd(
                &handle,
                |mut handle, _, new_state| {
                    let paused = match new_state {
                        LevelRunState::Running => false,
                        LevelRunState::Paused => true,
                    };

                    handle.bind_mut().set_paused(paused);
                }
            );
        
        let paused_right_now = self.level_run_state.is_paused();
        handle.bind_mut().set_paused(paused_right_now);


        // Communicator -> Level
        
        handle
            .signals()
            .update_idx()
            .builder()
            .connect_other_mut(self, Self::on_maze_update_idx);

        handle
            .signals()
            .commit_finished()
            .builder()
            .connect_other_mut(self, Self::on_maze_commit_found_path);


        handle.bind_mut().start();
    }


    fn get_tile_position(&self, x : i32, y : i32) -> Option<TileCenterPosition> {
        let multimesh = self.tile_spawner.get_multimesh()?;
        let mesh = multimesh.get_mesh()?;

        let position = Self::get_tile_position_from_cached(
            x,
            y,
            &mesh
        );

        Some(position)
    }


    fn get_tile_position_from_cached(
        x : i32,
        y : i32,
        mesh : &Gd<Mesh>

    ) -> TileCenterPosition {
        let size = mesh.get_aabb().size;
        let size_x = size.x;
        let height = size.y;
        let size_y = size.z;

        let coordinates_and_sizes_and_offsets = [
            (x, size_x),
            (y, size_y),
        ];

        let coordinate_array : [f32; 2] = coordinates_and_sizes_and_offsets.map(|(coordinate, size)| {
            (coordinate as f32) * size
        });

        let coordinates = Vector2::from_array(coordinate_array);

        let result = TileCenterPosition {
            coordinates,
            height,
        };

        result
    }


    fn get_number_of_visible_trees(&self) -> i32 {
        let low_performance_mode = self
            .get_options()
            .map_or(false, |options| {
                options.bind().get_low_performance_mode()
            });
        
        let visible_trees = if low_performance_mode {
            N_VISIBLE_TREES_IN_LOW_PERFORMANCE_MODE
        } else {
            -1
        };

        visible_trees
    }


    fn send_mail_to_letter_menu(&mut self, mail : GString) {
        self.mail_notification_sfx_player.play();
        self.pause_menu.bind_mut().send_mail_to_letter_menu(mail);

        let tween_opt = self.base_mut().create_tween();
        let old_tween_opt = std::mem::replace(&mut self.mail_message_tween, tween_opt);
        if let Some(mut old_tween) = old_tween_opt {
            old_tween.stop();
        }

        let tween_opt = self.mail_message_tween.clone();
        if let Some(mut tween) = tween_opt {
            self.mail_message_label.show();

            let interval = self.mail_color_change_interval;

            let colors = &self.mail_notification_colors;

            for color in colors.iter_shared() {
                let color_string = color.to_html();
                let new_string = format!(
                    "[color=#{}]{}[/color]",
                    &color_string,
                    &self.default_mail_message_text,
                );

                let mut mail_label = self.mail_message_label.clone();

                tween.tween_callback(
                    &Callable::from_fn(
                        &format!("set_mail_color_{}", &color_string),
                        move |_| {
                            mail_label.set_text(&new_string);
                        }
                    )
                );
                tween.tween_interval(interval);
            }

            tween
                .signals()
                .finished()
                .builder()
                .flags(ConnectFlags::DEFERRED | ConnectFlags::ONE_SHOT)
                .connect_other_mut(
                    self,
                    Self::on_mail_tween_finished
                );
        }
    }
}


// Utility

struct TileCenterPosition {
    coordinates : Vector2,
    height : f32,
}

fn default_mail_color_array() -> Array<Color> {
    // Using colors suggested by:
    // https://www.schemecolor.com/retro-disco-colors.php
    const COLOR_ARRAY : [&str; 6] = [
        "FFFFFF",
        "A414D9",
        "FF802B",
        "F9E105",
        "34C7A5",
        "5D50CE",
    ];

    let rust_colors = COLOR_ARRAY
        .map(|color_code| {
            Color::from_html(color_code).unwrap()
        });
    
    let colors = Array::from_iter(rust_colors);
    colors
}
