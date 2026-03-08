use std::path::{Path, PathBuf};

use crate::directory_definitions::{i_directory::IDirectory, project_root::ProjectRoot};


#[derive(Clone)]
pub struct GodotRoot {
    pub path : PathBuf
}


impl IDirectory for GodotRoot {
    fn get_path(&self) -> &Path {
        &self.path
    }
}


impl GodotRoot {
    pub fn from_project_root(project_root : ProjectRoot) -> Self {
        let project_root = project_root.get_path();

        let path = project_root.join("godot/project-reindeer");

        let result = Self {
            path
        };

        result
    }
}