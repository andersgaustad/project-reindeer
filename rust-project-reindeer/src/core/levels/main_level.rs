use godot::{classes::{BoxMesh, BoxShape3D, CollisionShape3D, IStaticBody3D, InputEvent, Mesh, MeshInstance3D, MultiMesh, MultiMeshInstance3D, RandomNumberGenerator, StandardMaterial3D, StaticBody3D, base_material_3d::Flags, multi_mesh::TransformFormat, object::ConnectFlags}, prelude::*};

use crate::{core::{common::{communicator::Communicator, coordinate::Coordinate, direction::Direction, i_add_padding::IAddPadding, padding::Padding, point_and_radius_2d::PointAndRadius2D}, environment::{enchanced_multi_mesh_instance_3d::EnchancedMultiMeshInstance3D, rock_type::RockType}, maze::{maze::{Maze, Tile}, maze_info::MazeInfo, maze_solver_info::MazeSolverInfo, maze_tile_state::MazeTileState, path_info::PathInfo, reindeer::Reindeer}, props::cabin::Cabin}, input_map::DEBUG};


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
    maze_top_left_corner : OnEditor<Gd<Node3D>>,

    #[export]
    #[var]
    maze_solver_info : Option<Gd<MazeSolverInfo>>,

    #[export]
    #[var]
    #[init(val = Color::GREEN)]
    arrow_color : Color,


    #[export_group(name = "Zones")]

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

    #[export(range=(0.0, 100.0, or_greater))]
    #[init(val = 1.0)]
    trees_per_square_unit : f32,


    #[export_group(name = "Random")]

    #[export]
    #[var]
    #[init(val = "Reindeer".into())]
    random_seed : GString,


    // Non-exported

    #[var]
    #[init(node = "%GroundMesh")]
    ground_mesh : OnReady<Gd<MeshInstance3D>>,

    #[var]
    #[init(node = "%Collision")]
    collision : OnReady<Gd<CollisionShape3D>>,

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

            self.set_ground_dimensions(forest_span);


            // Create forest

            let forest_size = &forest_span.size;
            let n_tree_spawn_attempts = (forest_size.x * &forest_size.y * self.trees_per_square_unit) as usize;

            let tree_multimesh_opt =self.tree_spawner.get_multimesh();
            if let Some(mut tree_multimesh) = tree_multimesh_opt {
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

                // Spawn trees
                let bound_tree_spawner = self.tree_spawner.bind();

                // In the extremly unlikely case that usize can't fit into i32, ignore and move on
                let n_trees_opt = i32::try_from(tree_spawn_positions.len()).ok();
                if let Some(n_trees) = n_trees_opt {
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

                }
                drop(bound_tree_spawner);
            }

            // Spawn (or move) cabin
            let cabin_aabb = self.cabin.bind().get_bounding_box();
            let position = cabin_aabb.position;
            let end = cabin_aabb.end();

            let top_down_start = Vector2::new(position.x, position.z);
            let top_down_end = Vector2::new(end.x, end.z);

            let corner_a = Vector2::new(top_down_start.x, top_down_end.y);
            let corner_b = Vector2::new(top_down_end.x, top_down_start.y);

            let corners = [
                top_down_start,
                corner_a,
                corner_b,
                top_down_end
            ];

            let radius = corners
                .iter()
                .map(|corner| {
                    corner.length()
                })
                .max_by(|a, b| {
                    a.total_cmp(b)
                })
                .unwrap();

            let diameter = 2.0 * radius;

            let cabin_position_info_opt = (|| {
                let shortest_side = forecourt_size.x.min(forecourt_size.y);
                if shortest_side < diameter {
                    return None;
                }

                // Else, should be able to spawn

                let intersection_from_position_length = std::f32::consts::SQRT_2 * radius;
                let middle_of_forecourt = forecourt_position + (forecourt_size / 2.0);

                let mut point = Vector2::new(intersection_from_position_length, intersection_from_position_length) + forecourt_position;
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

                let info = PointAndRadius2D::new(point, radius);
                Some(info)
            })();

            if cabin_position_info_opt.is_some() {
                godot_print!("Spawned cabin!");

            } else {
                godot_print!("Failed spawning cabin");
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


            // Finally, set maze info
            
            drop(bound_maze);

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
