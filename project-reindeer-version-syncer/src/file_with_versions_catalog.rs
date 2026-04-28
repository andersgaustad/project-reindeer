use std::path::PathBuf;

use crate::version_syncing::{cargo_toml_file::CargoTomlFile, i_version_syncing::IVersionSyncing, project_godot_file::ProjectGodotFile};


pub fn get_targets() -> Vec<Box<dyn IVersionSyncing>> {
    let path_to_here = std::fs::canonicalize(file!()).unwrap();

    let project_root = path_to_here
        // file_with_versions_catalog.rs
        .parent()
        .unwrap()
        // src
        .parent()
        .unwrap()
        // project-reindeer-version-syncer
        .parent()
        .unwrap()
        // project-reindeer = root
        ;
    
    let mut targets = Vec::new();

    const ACCEPTED_PATH_DIVIDER_TOKENS : [&str; 2] = [
        "/",
        "\\"
    ];

    let path_catalog = include_str!("file_with_versions_catalog.txt").lines();
    for path_str in path_catalog {
        if path_str.is_empty() {
            continue;
        }

        let accepted_tokens_iter = ACCEPTED_PATH_DIVIDER_TOKENS.iter();
        let token_to_split_on_opt = (|| {
            for token in accepted_tokens_iter{
                if path_str.contains(token) {
                    return Some(token);
                }
            }

            None
        })();

        let assembled_path = {
            let mut assembled_path = PathBuf::new();
            if let Some(token_to_split_on) = token_to_split_on_opt {
                let split = path_str.split(token_to_split_on);
                for segment in split {
                    assembled_path = assembled_path.join(segment);
                }
                
            } else {
                assembled_path = assembled_path.join(path_str);
            };

            assembled_path
        };

        let joined = project_root.join(assembled_path);
        let boxed = path_to_syncable_file(joined);
        targets.push(boxed);
    }

    targets
}


fn path_to_syncable_file(pathbuf : PathBuf) -> Box<dyn IVersionSyncing> {
    let extension = pathbuf
        .extension()
        .unwrap_or_else(|| {
            panic!(
                "Path '{}' has no file name!",
                pathbuf.display()
            )
        });
    
    // .toml
    if extension == "toml" {
        return Box::new(
            CargoTomlFile { pathbuf }
        );
    }

    // .godot
    if extension == "godot" {
        return Box::new(
            ProjectGodotFile { pathbuf }
        );
    }

    panic!("No match for extension '{}'", extension.display());    
}
