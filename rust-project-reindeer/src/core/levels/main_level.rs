use godot::{classes::{IStaticBody3D, Mesh, MultiMesh, MultiMeshInstance3D, StandardMaterial3D, StaticBody3D, base_material_3d::Flags, multi_mesh::TransformFormat}, prelude::*};


#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
pub struct MainLevel {
    #[export]
    #[var]
    dim_x : i32,

    #[export]
    #[var]
    dim_y : i32,

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
        self.update_with_current_dimensions();
    }
}


#[godot_api]
impl MainLevel {
    #[func]
    pub fn update_with_current_dimensions(&mut self) {
        let multimesh_opt = self.tile_spawner.get_multimesh();
        let Some(mut multimesh) = multimesh_opt else {
            return;
        };
        let Some(mut mesh) = multimesh.get_mesh() else {
            return;
        };

        let offset = self.get_top_corner_offset_from_cached(&mesh);

        let x_offset = offset.x;
        let y_offset = offset.y;

        let mut material = StandardMaterial3D::new_gd();
        material.set_albedo(Color::WHITE);
        material.set_flag(Flags::ALBEDO_FROM_VERTEX_COLOR, true);

        mesh.surface_set_material(0, &material);

        let n_tiles = self.dim_x * self.dim_y;

        multimesh.set_transform_format(TransformFormat::TRANSFORM_3D);
        multimesh.set_use_colors(true);

        multimesh.set_instance_count(n_tiles);

        let mut x = 0;
        let mut y = 0;
        for i in 0..n_tiles {
            if x >= self.dim_x {
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
    }


    #[func]
    pub fn update_with_dimensions(&mut self, dim_x : i32, dim_y : i32) {
        self.dim_x = dim_x;
        self.dim_y = dim_y;

        self.update_with_current_dimensions();
    }


    fn get_top_corner_offset(&self) -> Option<TopCornerOffsetInfoFull> {
        let multimesh = self.tile_spawner.get_multimesh()?;
        let mesh = multimesh.get_mesh()?;
        let offset = self.get_top_corner_offset_from_cached(&mesh);

        let result = TopCornerOffsetInfoFull {
            multimesh,
            mesh,
            offset,
        };

        Some(result)
    }


    fn get_top_corner_offset_from_cached(&self, mesh : &Gd<Mesh>) -> Vector2 {
        let aabb = mesh.get_aabb();
        let tile_size = aabb.size;

        let x_size = tile_size.x;
        let y_size = tile_size.z;

        let x_offset = - (self.dim_x - 1) as f32 * x_size / 2.0;
        let y_offset = - (self.dim_y - 1) as f32 * y_size / 2.0;

        let result = Vector2::new(x_offset, y_offset);

        result
    }
}


// Utility

struct TopCornerOffsetInfoFull {
    multimesh : Gd<MultiMesh>,
    mesh : Gd<Mesh>,

    offset : Vector2,
}

