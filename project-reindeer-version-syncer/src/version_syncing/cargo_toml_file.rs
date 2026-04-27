use std::path::PathBuf;

use crate::version_syncing::i_version_syncing::IVersionSyncing;


pub(in crate) struct CargoTomlFile {
    pub pathbuf : PathBuf,
}


impl IVersionSyncing for CargoTomlFile {
    fn get_path(&self) -> &std::path::Path {
        self.pathbuf.as_path()
    }

    fn line_has_version_tag(&self, line : &str) -> bool {
        let line_vector = line.split(" ").collect::<Vec<_>>();
        
        let triplet_array_opt : Result<[&str; 3], _> = line_vector.try_into();
        let Ok(triplet_array) = triplet_array_opt else {
            return false;
        };

        let matching = 
            triplet_array[0] == "version" &&
            triplet_array[1] == "=";
        
        matching        
    }

    fn replace_line(&self, _existing_line : &str, new_version : &str) -> String {
        format!("version = \"{}\"", new_version)
    }
}
