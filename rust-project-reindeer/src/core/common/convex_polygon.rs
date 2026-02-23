use godot::prelude::*;

use crate::core::common::plane_2d::Plane2D;


// ConvexPolygon

pub struct ConvexPolygon {
    corners : Vec<Vector2>,

    sides : Vec<Plane2D>
}


impl ConvexPolygon {
    pub fn try_from_points(points : Vec<Vector2>) -> Option<Self> {
        let n_points = points.len();

        let mut a_iter = points.iter().cycle();
        let mut b_iter = points.iter().cycle();
        b_iter.next();

        let mut sides = Vec::with_capacity(n_points);
        for _ in 0..n_points {
            let a = a_iter.next().unwrap();
            let b = b_iter.next().unwrap();

            let plane = Plane2D::try_from_points(*a, *b)?;
            sides.push(plane);
        }

        let result = Self {
            corners : points,
            sides,
        };

        Some(result)
    }


    pub fn contains_point(&self, point : Vector2) -> bool {
        self
            .sides
            .iter()
            .all(|plane| {
                plane.distance_to(point) <= 0.0
            })
    }


    pub fn overlaps_with(&self, other : &Self) -> bool {
        let seperating_plane = self.has_seperating_plane_against(other) || other.has_seperating_plane_against(self);
        !seperating_plane
    }


    /// Ported from
    /// https://docs.godotengine.org/en/stable/tutorials/math/vectors_advanced.html
    fn has_seperating_plane_against(&self, other : &Self) -> bool {
        self
            .sides
            .iter()
            .any(|plane| {
                other
                    .corners
                    .iter()
                    .all(|point| {
                        plane.distance_to(*point) >= 0.0
                    })
            })
    }
}
