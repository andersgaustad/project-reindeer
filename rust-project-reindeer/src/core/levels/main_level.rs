use godot::{classes::{FileAccess, IStaticBody3D, Mesh, MultiMesh, MultiMeshInstance3D, StandardMaterial3D, StaticBody3D, base_material_3d::Flags, file_access::ModeFlags, multi_mesh::TransformFormat}, prelude::*};

use crate::core::maze::maze::Maze;


#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
pub struct MainLevel {
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

    #[var]
    #[init(node = "%Center")]
    center : OnReady<Gd<Node3D>>,

    #[var]
    #[init(node = "%TileSpawner")]
    tile_spawner : OnReady<Gd<MultiMeshInstance3D>>,

    base : Base<StaticBody3D>,
}


#[godot_api]
impl IStaticBody3D for MainLevel {
    fn ready(&mut self) {
        let maze_file = self.get_maze_file();
        self.set_maze_file(maze_file);
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
        let Some(mut multimesh) = multimesh_opt else {
            return;
        };
        let Some(mut mesh) = multimesh.get_mesh() else {
            return;
        };


        if let Some(maze) = self.maze.clone() {
            let bound_maze = maze.bind();
            let dim_x = bound_maze.rust_get_x();
            let dim_y = bound_maze.rust_get_y();
            drop(bound_maze);

            let offset = self.get_top_corner_offset_from_cached(&mesh, dim_x, dim_y);

            let dim_x = dim_x as i32;
            let dim_y = dim_y as i32;

            let x_offset = offset.x;
            let y_offset = offset.y;

            let mut material = StandardMaterial3D::new_gd();
            material.set_albedo(Color::WHITE);
            material.set_flag(Flags::ALBEDO_FROM_VERTEX_COLOR, true);

            mesh.surface_set_material(0, &material);

            let n_tiles = dim_x * dim_y;

            multimesh.set_transform_format(TransformFormat::TRANSFORM_3D);
            multimesh.set_use_colors(true);

            multimesh.set_instance_count(n_tiles);

            let mut x = 0;
            let mut y = 0;
            for i in 0..n_tiles {
                if x >= dim_x {
                    x = 0;
                    y += 1;
                }

                let pos_x = x as f32 + x_offset;
                let pos_y = y as f32 + y_offset;

                let vector = Vector3::new(pos_x, 1.0, pos_y);
                let basis = Basis::default();
                let transform = Transform3D::new(basis, vector);

                let color = if (x + y) % 2 == 0 { self.color_a } else { self.color_b };

                multimesh.set_instance_transform(i, transform);
                multimesh.set_instance_color(i, color);

                x += 1;
            }

        } else {
            // Reset
            multimesh.set_instance_count(0);
        }
    }


    fn get_top_corner_offset(&self) -> Option<TopCornerOffsetInfoFull> {
        let multimesh = self.tile_spawner.get_multimesh()?;
        let mesh = multimesh.get_mesh()?;
        
        let maze = self.maze.clone()?;
        let bound = maze.bind();
        let dim_x = bound.rust_get_x();
        let dim_y = bound.rust_get_y();
        drop(bound);

        let offset = self.get_top_corner_offset_from_cached(&mesh, dim_x, dim_y);

        let result = TopCornerOffsetInfoFull {
            multimesh,
            mesh,
            maze,

            offset,
        };

        Some(result)
    }


    fn get_top_corner_offset_from_cached(&self, mesh : &Gd<Mesh>, dim_x : usize, dim_y : usize) -> Vector2 {
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
}


// Utility

struct TopCornerOffsetInfoFull {
    multimesh : Gd<MultiMesh>,
    mesh : Gd<Mesh>,
    maze : Gd<Maze>,

    offset : Vector2,
}

