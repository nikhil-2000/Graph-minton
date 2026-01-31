use std::{collections::HashMap, fs, path::PathBuf};

use crate::parsing::helpers::{entry_to_path, remove_empty_lines};

pub struct AliasesLoadResult {
    pub aliases: HashMap<String, Vec<String>>,
    pub failed_files: Vec<String>,
}

pub fn load_all_aliases(data_source: &str) -> AliasesLoadResult {
    let entries = match fs::read_dir(data_source) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read aliases directory '{}': {}", data_source, e);
            return AliasesLoadResult {
                aliases: HashMap::new(),
                failed_files: vec![data_source.to_string()],
            };
        }
    };

    let paths = entries.filter_map(entry_to_path).collect::<Vec<_>>();

    let (aliases, failed_files): (Vec<_>, Vec<_>) = paths
        .iter()
        .map(|path| load_alias_file(path))
        .map(|result| match result {
            Ok((name, aliases)) => (Some((name, aliases)), None),
            Err(path_str) => {
                eprintln!("Failed to load alias file '{}'", path_str);
                (None, Some(path_str))
            }
        })
        .unzip();

    AliasesLoadResult {
        aliases: aliases.into_iter().flatten().collect(),
        failed_files: failed_files.into_iter().flatten().collect(),
    }
}

fn load_alias_file(path: &PathBuf) -> Result<(String, Vec<String>), String> {
    let main_name = path
        .file_stem()
        .ok_or_else(|| path.to_string_lossy().to_string())?
        .to_string_lossy()
        .to_string();

    match fs::read_to_string(path) {
        Ok(contents) => {
            let aliases = contents.lines().filter_map(remove_empty_lines).collect();
            Ok((main_name, aliases))
        }
        Err(_) => Err(format!("Failed to read file: {}", main_name)),
    }
}
