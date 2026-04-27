use godot::prelude::*;


pub mod input_map;
pub mod core;
pub mod cfg;


struct ProjectReindeer;

#[gdextension]
unsafe impl ExtensionLibrary for ProjectReindeer {}
