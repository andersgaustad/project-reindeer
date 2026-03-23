use godot::prelude::*;

use crate::core::options::options::Options;


#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct OptionsWrapperNode {
    #[export]
    #[var]
    options : OnEditor<Gd<Options>>,

    base : Base<Node>,
}
