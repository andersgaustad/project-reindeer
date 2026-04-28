use clap::Parser;

use crate::cli::clap_parser::ClapParser;


mod cli;
mod file_with_versions_catalog;
mod version_syncing;


pub fn main() {
    let parser = ClapParser::parse();

    let ClapParser {
        tag,
    } = parser;

    let version_files_to_update = file_with_versions_catalog::get_targets();
    for file in version_files_to_update {
        file.change_version_of_file_at_path(&tag).unwrap_or_else(|e| {
            panic!("Error: {}", e)
        });
    }
}
