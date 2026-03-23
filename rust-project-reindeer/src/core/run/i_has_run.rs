use godot::prelude::*;

use crate::core::{audio::sfx_manager::SFXManager, options::{options::Options, options_wrapper_node::OptionsWrapperNode}, run::run::Run};


pub trait IHasRun
{
    fn get_run(&self) -> Option<Gd<Run>>;


    fn get_options_wrapper(&self) -> Option<Gd<OptionsWrapperNode>> {
        get_child_of_type_opt(self.get_run())
    }


    fn get_options(&self) -> Option<Gd<Options>> {
        self
            .get_options_wrapper()?
            .bind()
            .get_options()
    }


    fn get_sfx_mananger(&self) -> Option<Gd<SFXManager>> {
        get_child_of_type_opt(self.get_run())
    }
}


// Utility

fn get_child_of_type<S, T>(node : Gd<S>) -> Option<Gd<T>>
where
S : GodotClass + Inherits<Node>,
T : GodotClass + Inherits<Node>
{
    let name = T::class_id().to_string_name();
    node
        .upcast()
        .get_node_or_null(name.arg())?
        .try_cast::<T>()
        .ok()
}


fn get_child_of_type_opt<S, T>(node_opt : Option<Gd<S>>) -> Option<Gd<T>>
where
S : GodotClass + Inherits<Node>,
T : GodotClass + Inherits<Node>,
{
    let node = node_opt?;
    let child_opt = get_child_of_type(node);
    child_opt
}
