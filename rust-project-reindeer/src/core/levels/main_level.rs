use godot::{classes::{IStaticBody3D, InputEvent, Mesh, MultiMesh, MultiMeshInstance3D, RandomNumberGenerator, StandardMaterial3D, StaticBody3D, base_material_3d::Flags, multi_mesh::TransformFormat, object::ConnectFlags}, prelude::*};

use crate::{core::{common::{communicator::Communicator, coordinate::Coordinate, direction::Direction, padding::Padding}, environment::{enchanced_multi_mesh_instance_3d::EnchancedMultiMeshInstance3D, rock_type::RockType}, maze::{maze::{Maze, Tile}, maze_info::MazeInfo, maze_solver_info::MazeSolverInfo, maze_tile_state::MazeTileState, path_info::PathInfo, reindeer::Reindeer}}, input_map::DEBUG};


#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
pub struct MainLevel {
    #[export_group(name = "Maze")]
    #[export]
    #[var]
    #[init(val = Color::DARK_RED)]
    color_a : Color,

    #[export]
    #[var]
    #[init(val = Color::DARK_GREEN)]
    color_b : Color,

    #[export]
    #[var]
    maze_top_left_corner : Option<Gd<Node3D>>,

    #[export]
    #[var]
    maze_solver_info : Option<Gd<MazeSolverInfo>>,

    #[export]
    #[var]
    #[init(val = Color::GREEN)]
    arrow_color : Color,

    #[export_group(name = "Zones")]
    #[export]
    clearing_ring : Option<Gd<Padding>>,

    #[export]
    forest_ring : Option<Gd<Padding>>,

    #[export_group(name = "Random")]
    #[export]
    #[var]
    #[init(val = "Reindeer".into())]
    random_seed : GString,

    #[var]
    #[init(node = "%Reindeer")]
    maze_reindeer : OnReady<Gd<Reindeer>>,

    #[var]
    #[init(node = "%GhostReindeer")]
    maze_ghost_reindeer : OnReady<Gd<Reindeer>>,

    #[var]
    #[init(node = "%Present")]
    present : OnReady<Gd<Node3D>>,

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

    rng : Gd<RandomNumberGenerator>,

    maze_info : Option<MazeInfo>,

    base : Base<StaticBody3D>,
}


#[godot_api]
impl IStaticBody3D for MainLevel {
    fn ready(&mut self) {
        let seed = self.random_seed.hash_u32();
        let seed_u64 = u64::from(seed);
        self.rng.set_seed(seed_u64);

        self.maze_reindeer.hide();
        self.maze_ghost_reindeer.hide();

        let maze = std::mem::take(&mut self.maze_info).map(|info| info.maze);
        self.set_maze(maze);
    }


    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed(DEBUG) {
            self.run_maze_solver();
            return;
        }
    }
}


#[godot_api]
impl MainLevel {
    #[func]
    pub fn set_maze(&mut self, maze_opt : Option<Gd<Maze>>) {
        self.maze_info = None;

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


        if let Some(maze) = maze_opt.clone() {
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

            let size = tile_mesh.get_aabb().size;
            let tile_height = size.y;

            let dim_x_f32 = dim_x as f32;
            let dim_y_f32 = dim_y as f32;

            let bounding_box_size = Vector2::new(
                dim_x_f32 * size.x,
                dim_y_f32 * size.z
            );

            let maze_top_left_corner_offset = self.get_maze_top_left_corner_position_or_default();

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

                let vector = Vector3::new(pos_x, 0.0, pos_y) + maze_top_left_corner_offset;
                let basis = Basis::default();
                let transform = Transform3D::new(basis, vector);

                let color = if i % 2 == 0 { self.color_a } else { self.color_b };

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

            ) + maze_top_left_corner_offset;

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

            ) + maze_top_left_corner_offset;

            let present = &mut self.present;
            present.set_position(position);
            present.show();

            drop(bound_maze);

            // Finally, set
            let maze_info = MazeInfo {
                maze,
                size : bounding_box_size
            };

            self.maze_info = Some(maze_info);

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

                if let Some(maze_info) = self.maze_info.as_ref() {
                    let maze = maze_info.maze.clone();

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

                                ) + self.get_maze_top_left_corner_position_or_default();

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

        let delay = self.get_maze_solver_info_or_default().bind().wait_delay;
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
    fn on_maze_commit_found_path(&mut self, path_info : Gd<PathInfo>) {
        godot_print!("Path commited!");
        self.maze_ghost_reindeer.hide();

        let bound = path_info.bind();
        let paths = bound.rust_get_paths();

        let Some(first_path) = paths.first() else {
            godot_print!("TODO: No paths found!");
            return;
        };

        let n_arrows = first_path.len().checked_sub(1).unwrap_or(0);

        // Configure arrow spawner

        let Some(mut arrow_multimesh) = self.arrow_spawner.get_multimesh() else {
            godot_error!("Could not find Arrow Spawner MultiMesh!");
            return;
        };

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

        let maze_top_left_corner_offset = self.get_maze_top_left_corner_position_or_default();

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

            arrow_multimesh.set_instance_transform(i_i32, transform);
            arrow_multimesh.set_instance_color(i_i32, self.arrow_color);
        }

        godot_print!("Found path!");
    }


    #[func]
    fn run_maze_solver(&mut self) {
        let Some(maze_info) = self.maze_info.as_ref() else {
            godot_warn!("Tried running maze solver without maze? Returning...");
            return;
        };
        let mut maze = maze_info.maze.clone();

        let maze_solver_info = self.get_maze_solver_info_or_default();
        let mut handle = maze.bind_mut().find_paths(maze_solver_info);

        handle
            .signals()
            .update_idx()
            .builder()
            .connect_other_mut(self, Self::on_maze_update_idx);

        handle
            .signals()
            .commit_found_path()
            .builder()
            .connect_other_mut(self, Self::on_maze_commit_found_path);


        handle.bind_mut().start();
    }


    pub fn get_maze_top_left_corner_position_or_default(&self) -> Vector3 {
        self
            .maze_top_left_corner
            .as_ref()
            .map(|node_3d| node_3d.get_position())
            .unwrap_or_default()
    }


    fn get_rock_spawner_from_type(&self, rock_type : RockType) -> Gd<EnchancedMultiMeshInstance3D> {
        match rock_type {
            RockType::Small => self.get_rock_small_spawner(),
            RockType::Medium => self.get_rock_medium_spawner(),
            RockType::Large => self.get_rock_large_spawner(),
        }
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


    pub fn get_maze_solver_info_or_default(&self) -> Gd<MazeSolverInfo> {
        self.maze_solver_info.clone().unwrap_or_default()
    }


    pub fn get_clearing_ring_padding_or_default(&self) -> Gd<Padding> {
        self.clearing_ring.clone().unwrap_or_default()
    }


    pub fn get_forest_ring_padding_or_default(&self) -> Gd<Padding> {
        self.forest_ring.clone().unwrap_or_default()
    }
}


// Utility

struct TopCornerOffsetInfoFull {
    multimesh : Gd<MultiMesh>,
    mesh : Gd<Mesh>,
    maze : Gd<Maze>,

    offset : Vector2,
}


struct TileCenterPosition {
    coordinates : Vector2,
    height : f32,
}
