use std::path::PathBuf;

use crate::version_syncing::i_version_syncing::IVersionSyncing;


pub(in crate) struct ProjectGodotFile {
    pub pathbuf : PathBuf,
}


impl IVersionSyncing for ProjectGodotFile {
    fn get_path(&self) -> &std::path::Path {
        self.pathbuf.as_path()
    }


    fn line_has_version_tag(&self, line : &str) -> bool {
        line.contains("config/version=")        
    }


    fn replace_line(&self, _existing_line : &str, new_version : &str) -> String {
        format!("config/version=\"{}\"", new_version)
    }
}
