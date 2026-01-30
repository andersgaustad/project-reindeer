use godot::{classes::{FileAccess, IStaticBody3D, InputEvent, Mesh, MultiMesh, MultiMeshInstance3D, RandomNumberGenerator, StandardMaterial3D, StaticBody3D, base_material_3d::Flags, file_access::ModeFlags, multi_mesh::TransformFormat, object::ConnectFlags}, prelude::*};

use crate::{core::{common::{acknowledger::Communicator, direction::Direction}, environment::{rock_spawner::RockSpawner, rock_type::RockType}, maze::{maze::{Maze, Tile}, maze_solver_info::MazeSolverInfo, maze_tile_state::MazeTileState, path_info::PathInfo, reindeer::Reindeer}}, input_map::DEBUG};


#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
pub struct MainLevel {
    #[export_group(name = "Maze")]
    #[export(file = "*.txt")]
    #[var(get, set = set_maze_file)]
    maze_file : GString,

    maze : Option<Gd<Maze>>,

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
    #[init(val = 1.0)]
    maze_floor_height : f32,

    #[export]
    #[var]
    maze_solver_info : Option<Gd<MazeSolverInfo>>,

    #[export_group(name = "Random")]
    #[export]
    #[var]
    #[init(val = "Reindeer".into())]
    random_seed : GString,

    #[var]
    #[init(node = "%Center")]
    center : OnReady<Gd<Node3D>>,

    #[var]
    #[init(node = "%Reindeer")]
    maze_reindeer : OnReady<Gd<Reindeer>>,

    #[var]
    #[init(node = "%Present")]
    present : OnReady<Gd<Node3D>>,

    #[var]
    #[init(node = "%TileSpawner")]
    tile_spawner : OnReady<Gd<MultiMeshInstance3D>>,

    #[var]
    #[init(node = "%RockSmallSpawner")]
    rock_small_spawner : OnReady<Gd<RockSpawner>>,

    #[var]
    #[init(node = "%RockMediumSpawner")]
    rock_medium_spawner : OnReady<Gd<RockSpawner>>,

    #[var]
    #[init(node = "%RockLargeSpawner")]
    rock_large_spawner : OnReady<Gd<RockSpawner>>,

    rng : Gd<RandomNumberGenerator>,

    base : Base<StaticBody3D>,
}


#[godot_api]
impl IStaticBody3D for MainLevel {
    fn ready(&mut self) {
        let seed = self.random_seed.hash_u32();
        let seed_u64 = u64::from(seed);
        self.rng.set_seed(seed_u64);

        let maze_file = self.get_maze_file();
        self.set_maze_file(maze_file);
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
    pub fn set_maze_file(&mut self, value : GString) {
        self.maze_file = value;

        if !self.base().is_node_ready() {
            return;
        }
        
        self.update_maze(None);

        if self.maze_file.is_empty() {
            return;
        }

        // Else
        let Some(file_access) = FileAccess::open(&self.maze_file, ModeFlags::READ) else {
            return;
        };

        let text = file_access.get_as_text().to_string();
        let maze_opt = Maze::try_new_gd_from_str(&text);

        self.update_maze(maze_opt);
    }


    fn update_maze(&mut self, maze_opt : Option<Gd<Maze>>) {
        self.maze = maze_opt;

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

            let offset = Self::get_top_corner_offset_from_mesh(dim_x, dim_y, &tile_mesh);

            let x_offset = offset.x;
            let y_offset = offset.y;

            let mut material = StandardMaterial3D::new_gd();
            material.set_albedo(Color::WHITE);
            material.set_flag(Flags::ALBEDO_FROM_VERTEX_COLOR, true);

            tile_mesh.surface_set_material(0, &material);

            let dim_x_i32 = dim_x as i32;
            let dim_y_i32 = dim_y as i32;

            let n_tiles = dim_x_i32 * dim_y_i32;

            tile_multimesh.set_transform_format(TransformFormat::TRANSFORM_3D);
            tile_multimesh.set_use_colors(true);

            tile_multimesh.set_instance_count(n_tiles);


            let size = tile_mesh.get_aabb().size;
            let size_x = size.x;
            let size_y = size.z;
            let tile_height = size.y;

            let mut x = 0;
            let mut y = 0;
            for i in 0..n_tiles {
                if x >= dim_x_i32 {
                    x = 0;
                    y += 1;
                }

                // Base

                let position = Self::get_tile_position_from_cached(x, y, dim_x, dim_y, &tile_mesh);

                let pos_x = position.coordinates.x;
                let pos_y = position.coordinates.y;

                let vector = Vector3::new(pos_x, self.maze_floor_height, pos_y);
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
                multimesh.set_transform_format(TransformFormat::TRANSFORM_3D);
                multimesh.set_instance_count(n_rocks_of_type as i32);

                for (i, mut position) in positions.into_iter().enumerate() {
                    position.y += tile_height;

                    let transform = spawner.bind().create_rock_transform(position, self.rng.clone());

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
                dim_x,
                dim_y,
                &tile_mesh
            );

            let position = Vector3::new(
                position_info.coordinates.x,
                self.maze_floor_height + position_info.height,
                position_info.coordinates.y
            );

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
                dim_x,
                dim_y,
                &tile_mesh
            );

            let position = Vector3::new(
                position_info.coordinates.x,
                self.maze_floor_height + position_info.height,
                position_info.coordinates.y
            );

            let present = &mut self.present;
            present.set_position(position);
            present.show();

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
    fn run_maze_solver(&mut self) {
        godot_print!("Started maze solver...");
        let Some(mut maze) = self.maze.clone() else {
            return;
        };

        let maze_solver_info = self.get_maze_solver_info_or_default();
        let mut handle = maze.bind_mut().find_paths(maze_solver_info);

        handle
            .signals()
            .update_idx()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(self, Self::on_maze_update_idx);

        handle
            .signals()
            .commit_found_path()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(self, Self::on_maze_commit_found_path);


        handle.bind_mut().start();
    }


    #[func]
    fn on_maze_update_idx(&mut self, idx : i32, state : MazeTileState, direction : Direction, acknowledger : Gd<Communicator>) {
        let Some(mut multimesh) = self.tile_spawner.get_multimesh() else {
            godot_print!("Expected multimesh!");
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
            },
        }

        let delay = self.get_maze_solver_info_or_default().bind().wait_delay;
        let mut scene_tree = self.base().get_tree().expect("Failed getting scene tree??");
        let timer = scene_tree.create_timer(delay).expect("Failed creating timer??");

        timer
            .signals()
            .timeout()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_gd(
                &acknowledger,
                |ack| {
                    ack.signals().done().emit();
                }
            );
    }


    #[func]
    fn on_maze_commit_found_path(&mut self, path_info : Gd<PathInfo>) {
        godot_print!("Found path!");
    }


    fn get_rock_spawner_from_type(&self, rock_type : RockType) -> Gd<RockSpawner> {
        match rock_type {
            RockType::Small => self.get_rock_small_spawner(),
            RockType::Medium => self.get_rock_medium_spawner(),
            RockType::Large => self.get_rock_large_spawner(),
        }
    }


    fn get_top_corner_offset(&self) -> Option<TopCornerOffsetInfoFull> {
        let multimesh = self.tile_spawner.get_multimesh()?;
        let mesh = multimesh.get_mesh()?;
        
        let maze = self.maze.clone()?;
        let bound = maze.bind();
        let dim_x = bound.rust_get_dim_x();
        let dim_y = bound.rust_get_dim_y();
        drop(bound);

        let offset = Self::get_top_corner_offset_from_mesh(dim_x, dim_y, &mesh);

        let result = TopCornerOffsetInfoFull {
            multimesh,
            mesh,
            maze,

            offset,
        };

        Some(result)
    }


    fn get_top_corner_offset_from_mesh(dim_x : usize, dim_y : usize, mesh : &Gd<Mesh>) -> Vector2 {
        let aabb = mesh.get_aabb();

        let tile_size = aabb.size;

        let x_size = tile_size.x;
        let y_size = tile_size.z;

        let dimensions = [
            (dim_x, x_size),
            (dim_y, y_size)
        ];

        let offsets = dimensions.map(|(dimension, size)| {
            - ((dimension.checked_sub(1).unwrap_or(0)) as f32 * size / 2.0)
        });

        let result = Vector2::from_array(offsets);

        result
    }


    fn get_tile_position(&self, x : i32, y : i32) -> Option<TileCenterPosition> {
        let multimesh = self.tile_spawner.get_multimesh()?;
        let mesh = multimesh.get_mesh()?;
        
        let maze = self.maze.clone()?;
        let bound = maze.bind();
        let dim_x = bound.rust_get_dim_x();
        let dim_y = bound.rust_get_dim_y();
        drop(bound);

        let position = Self::get_tile_position_from_cached(
            x,
            y,
            dim_x,
            dim_y,
            &mesh
        );

        Some(position)
    }


    fn get_tile_position_from_cached(
        x : i32,
        y : i32,
        dim_x : usize,
        dim_y : usize,
        mesh : &Gd<Mesh>

    ) -> TileCenterPosition {
        let offset = Self::get_top_corner_offset_from_mesh(dim_x, dim_y, mesh);
        let offset_x = offset.x;
        let offset_y = offset.y;

        let size = mesh.get_aabb().size;
        let size_x = size.x;
        let height = size.y;
        let size_y = size.z;

        let coordinates_and_sizes_and_offsets = [
            (x, size_x, offset_x),
            (y, size_y, offset_y),
        ];

        let coordinate_array : [f32; 2] = coordinates_and_sizes_and_offsets.map(|(coordinate, size, offset)| {
            (coordinate as f32) * size + offset
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
