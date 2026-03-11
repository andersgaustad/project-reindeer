use std::{collections::BTreeMap, ffi::OsStr, io::{BufWriter, Write}, path::Path};

use walkdir::WalkDir;

use crate::{asset_info::AssetInfo, directory_definitions::{assets_root::AssetsRoot, credits_out_root::CreditsOutRoot, godot_root::GodotRoot, i_directory::IDirectory, project_root::ProjectRoot}};

mod asset_info;
mod directory_definitions;


pub fn main() {
    let path_to_here = std::fs::canonicalize(file!()).unwrap();

    let project_root = ProjectRoot::from_path_to_this_file(path_to_here);
    let godot_root = GodotRoot::from_project_root(project_root);
    
    let assets_root = AssetsRoot::from_godot_root(godot_root.clone());
    let credits_root = CreditsOutRoot::from_godot_root(godot_root);
    let credits_root_path = credits_root.get_path();

    let assets = get_assets(assets_root, &OsStr::new("asset_info.toml"));
    let sorted_assets = type_sorted_assets(assets.into_iter());

    let credits_md_path = credits_root_path.join("CREDITS.md");
    let credits_txt_path = credits_root_path.join("credits.bbcode.txt");

    write_markdown_credits(&credits_md_path, &sorted_assets).expect("Failed writing MD Credits!");
    write_bbcode_credits(&credits_txt_path, &sorted_assets).expect("Failed writing TXT Credits!");
}


fn get_assets(asset_root : AssetsRoot, asset_info_file_name : &OsStr) -> Vec<AssetInfo> {
    let mut assets = Vec::new();
    let path = asset_root.get_path();

    let walkdir = WalkDir::new(path);
    for entry_result in walkdir {
        let Ok(entry) = entry_result else {
            continue;
        };

        let file_name = entry.file_name();
        if file_name != asset_info_file_name {
            continue;
        };

        let entry_path = entry.path();
        let content_result = std::fs::read_to_string(entry_path);
        let Ok(content) = content_result else {
            continue;
        };

        let asset_info_result = toml::from_str::<AssetInfo>(&content);
        let Ok(asset_info) = asset_info_result else {
            continue;
        };

        assets.push(asset_info);
    }

    assets
}


fn type_sorted_assets(assets : impl Iterator<Item = AssetInfo>) -> BTreeMap<String, Vec<AssetInfo>> {
    let mut map = BTreeMap::new();

    for asset in assets {
        let ty = asset.ty.clone();

        let assets = map.entry(ty).or_insert(Vec::new());
        assets.push(asset);
    }

    for (_, assets) in map.iter_mut() {
        assets.sort_unstable_by(|a, b| {
            let asset_a = &a.name;
            let asset_b = &b.name;

            asset_a.cmp(asset_b)
        });
    }

    map
}


fn write_markdown_credits(out_credits_file : &Path, map : &BTreeMap<String, Vec<AssetInfo>>) -> Result<(), std::io::Error> {
    let file : std::fs::File = std::fs::File::create(out_credits_file)?;
    let mut buffer = BufWriter::new(file);

    writeln!(buffer, "# Credits")?;
    writeln!(buffer, "The following assets are used in this project.")?;
    writeln!(buffer, "Huge thanks to all the creators!")?;
    writeln!(buffer, "\n")?;

    for (header, assets) in map.iter() {
        let capitalized = header
            .char_indices()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            })
            .collect::<String>();

        writeln!(buffer, "## {}\n", capitalized)?;

        for asset_info in assets {
            let AssetInfo {
                name,
                author,
                source,
                license,
                license_source,
                ty : _ty,
                custom_attribution,
            } = asset_info;

            let not_attributed = "*(not attributed)*".to_string();

            let author = if let Some(author) = author {
                format!("**{}**", author)
            } else {
                not_attributed
            };
            

            writeln!(buffer, "**{}**", name)?;
            writeln!(buffer, "- Author: {}", author)?;
            writeln!(buffer, "- Source: **[link]({})**", source)?;
            writeln!(buffer, "- License: **[{}]({})**", license, license_source)?;
            
            if let Some(custom_attribution_override) = custom_attribution {
                writeln!(buffer, "- {}", custom_attribution_override)?;
            };

            writeln!(buffer, "")?;
        }

        writeln!(buffer, "")?;
    }

    buffer.flush()?;

    Ok(())
}


fn write_bbcode_credits(out_credits_file : &Path, map : &BTreeMap<String, Vec<AssetInfo>>) -> Result<(), std::io::Error> {
    let file : std::fs::File = std::fs::File::create(out_credits_file)?;
    let mut buffer = BufWriter::new(file);

    writeln!(buffer, "[p][b]Credits and Acknowledgements[/b][/p]")?;
    writeln!(buffer, "This project would not have been possible wothout the use of assets and creations from other creators.")?;
    writeln!(buffer, "See the bundled Markdown file for a detailed list of assets, authors, and licenses.")?;
    writeln!(buffer, "\n")?;

    for (header, assets) in map.iter() {
        let capitalized = header
            .char_indices()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            })
            .collect::<String>();

        writeln!(buffer, "[b]{}:[/b]\n", capitalized)?;

        for asset in assets.iter() {
            let by_part = if let Some(author) = &asset.author {
                format!(" by {}", author)
            } else {
                String::new()
            };

            writeln!(
                buffer,
                "[i]{}[/i]{} ([url={}]External link[/url])",
                &asset.name,
                by_part,
                &asset.source,
            )?;
        }
        writeln!(buffer, "\n")?;
    }

    Ok(())
}
