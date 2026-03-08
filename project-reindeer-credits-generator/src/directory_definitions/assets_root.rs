use std::path::{Path, PathBuf};

use crate::directory_definitions::{godot_root::GodotRoot, i_directory::IDirectory};


pub struct AssetsRoot {
    pub path : PathBuf
}


impl IDirectory for AssetsRoot {
    fn get_path(&self) -> &Path {
        &self.path
    }
}


impl AssetsRoot {
    pub fn from_godot_root(root : GodotRoot) -> Self {
        let godot_root = root.get_path();

        let path = godot_root.join("assets");

        let result = Self {
            path
        };

        result
    }
}
