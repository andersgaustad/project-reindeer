use godot::prelude::*;


/// Get first parent (or self) in tree (when traversing bottom up) or None.
/// 
/// Useful for finding the root Run node, but can be used in other cases as well.
pub fn try_find_parent_of_type<T>(node : Gd<Node>) -> Option<Gd<T>>
where T : Inherits<Node>
{
    let cast_result = node.try_cast::<T>();
    match cast_result {
        Ok(run) => {
            Some(run)
        },
        Err(node) => {
            let parent = node.get_parent()?;
            try_find_parent_of_type(parent)
        },
    }
}
