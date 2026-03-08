use std::path::{Path, PathBuf};

use crate::directory_definitions::i_directory::IDirectory;


pub struct ProjectRoot {
    pub path : PathBuf
}


impl IDirectory for ProjectRoot {
    fn get_path(&self) -> &Path {
        &self.path
    }
}


impl ProjectRoot {
    pub fn from_path_to_this_file(path : PathBuf) -> Self {
        let mut path_ref = path.as_path();

        for _ in 0..3 {
            path_ref = path_ref.parent().unwrap();
        }

        let path = path_ref.to_path_buf();

        let result = Self {
            path
        };

        result
    }
}
