use std::{io::{BufRead, BufReader, Write}, path::Path};

use in_place::InPlace;


pub(in crate) trait IVersionSyncing {
    fn change_version_of_file_at_path(&self, version_string : &str) ->Result<(), Box<dyn std::error::Error>> {
        let inp = InPlace::new(&self.get_path()).open()?;
        let reader = BufReader::new(inp.reader());
        let mut writer = inp.writer();

        for line in reader.lines() {
            let mut line = line?;
            let matching_line = self.line_has_version_tag(&line);
            if matching_line {
                line = self.replace_line(&line, version_string);
            }

            writeln!(
                writer,
                "{}",
                line
            )?;
        }

        inp.save()?;

        Ok(())
    }

    fn get_path(&self) -> &Path;

    fn line_has_version_tag(&self, line : &str) -> bool;

    fn replace_line(&self, existing_line : &str, new_version : &str) -> String;
}
