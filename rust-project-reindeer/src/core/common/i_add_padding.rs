use godot::{classes::class_macros::private::virtuals::Os::Rect2, obj::Gd};

use crate::core::common::padding::Padding;


pub trait IAddPadding {
    fn grow_with_padding(self, padding : Gd<Padding>) -> Self;
}


// Rect2

impl IAddPadding for Rect2 {
    fn grow_with_padding(self, padding : Gd<Padding>) -> Self {
        let bound = padding.bind();
        let left = bound.west_padding;
        let top = bound.north_padding;
        let right = bound.east_padding;
        let bottom = bound.south_padding;

        let result = self.grow_individual(left, top, right, bottom);

        result 
    }
}
