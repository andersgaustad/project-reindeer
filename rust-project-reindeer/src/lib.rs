use godot::prelude::*;


pub mod input_map;
pub mod core;


struct ProjectReindeer;

#[gdextension]
unsafe impl ExtensionLibrary for ProjectReindeer {}
