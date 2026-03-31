//! Ported from:
//! https://github.com/godotengine/godot-proposals/issues/10032#issuecomment-2576403321
//! https://www.reddit.com/r/godot/comments/18bfn0n/how_to_calculate_node3d_bounding_box/


use derive_builder::Builder;
use godot::{builtin::math::ApproxEq, classes::{Light3D, VisualInstance3D}, prelude::*};


// GetBoundingBoxArgs

#[derive(Builder, Clone)]
#[builder(
    // See
    // https://github.com/colin-kiegel/rust-derive-builder/blob/079fd81949c31ad1de88772dae75a80be1e50b2e/derive_builder/examples/custom_constructor.rs#L64
    build_fn(
        private,
        name = "private_build"
    ),
    default,
    pattern = "owned"
)]
#[must_use]
pub struct GetBoundingBoxArgs {
    node : Option<Gd<Node3D>>,

    ignore_top_level : bool,

    bounds_transform : Transform3D,
}


impl Default for GetBoundingBoxArgs {
    fn default() -> Self {
        Self {
            node : None,
            ignore_top_level : true,
            bounds_transform : Default::default()
        }
    }
}

impl GetBoundingBoxArgs {
    pub fn done(self) -> Aabb {
        // get_node_aabb

        let Some(node) = self.node else {
            return Aabb::default();
        };

        if node.is_queued_for_deletion() {
            return Aabb::default();
        }
        

        let transform = if self.bounds_transform.approx_eq(&Transform3D::default()) {
            node.get_global_transform()
        } else {
            self.bounds_transform
        };

        let top_xform = transform.affine_inverse() * node.get_global_transform();

        let visual_instance_result = node.clone().try_cast::<VisualInstance3D>();
        let mut aabb_box = visual_instance_result
            .ok()
            .map(|visual_instance| {
                visual_instance.get_aabb()
            })
            .unwrap_or_default();

        aabb_box = top_xform * aabb_box;

        for child in node.get_children().iter_shared() {
            let child_3d_result = child.try_cast::<Node3D>();
            let Ok(child_3d) = child_3d_result else {
                continue;
            };

            // Make sure this is NOT a Light3D.
            let child_3d_light_result = child_3d.try_cast::<Light3D>();
            let Err(child_3d) = child_3d_light_result else {
                continue;
            };

            let ignore = self.ignore_top_level && child_3d.is_set_as_top_level();
            if ignore {
                continue;
            }

            let child_box = get_node_aabb_ex()
                .node(Some(child_3d))
                .ignore_top_level(self.ignore_top_level)
                .bounds_transform(transform)
                .build()
                .done();
            
            aabb_box = aabb_box.merge(child_box);
        }

        aabb_box
    }
}


// GetBoundingBoxArgsBuilder

impl GetBoundingBoxArgsBuilder {
    pub fn build(self) -> GetBoundingBoxArgs {
        self
            .private_build()
            .expect("GetBoundingBoxArgs should be infallible!")
    }
}


// get_node_aabb

pub fn get_node_aabb_ex() -> GetBoundingBoxArgsBuilder {
    GetBoundingBoxArgsBuilder::default()
}


pub fn get_node_aabb() -> Aabb {
    get_node_aabb_ex().build().done()
}
