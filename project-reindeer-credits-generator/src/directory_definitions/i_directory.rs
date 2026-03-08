use std::path::Path;


pub trait IDirectory {
    fn get_path(&self) -> &Path;
}
