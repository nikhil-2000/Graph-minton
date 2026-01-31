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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_all_aliases_happy_path() {
        let result = load_all_aliases("src/parsing/test_data/aliases");

        // Check that we loaded aliases successfully
        assert!(!result.aliases.is_empty(), "Should load some aliases");
        
        // Check Chetan aliases
        assert!(
            result.aliases.contains_key("Chetan"),
            "Should have Chetan as a key"
        );
        let chetan_aliases = &result.aliases["Chetan"];
        assert_eq!(chetan_aliases.len(), 3, "Chetan should have 3 aliases");
        assert!(chetan_aliases.contains(&"Chet".to_string()));
        assert!(chetan_aliases.contains(&"Chet P".to_string()));
        assert!(chetan_aliases.contains(&"C. Pat".to_string()));

        // Check Nikhil aliases
        assert!(
            result.aliases.contains_key("Nikhil"),
            "Should have Nikhil as a key"
        );
        let nikhil_aliases = &result.aliases["Nikhil"];
        assert_eq!(nikhil_aliases.len(), 2, "Nikhil should have 2 aliases");
        assert!(nikhil_aliases.contains(&"Nik".to_string()));
        assert!(nikhil_aliases.contains(&"Nik P".to_string()));

        // Empty file should load with 0 aliases
        assert!(
            result.aliases.contains_key("Empty"),
            "Should have Empty as a key"
        );
        assert_eq!(
            result.aliases["Empty"].len(),
            0,
            "Empty file should have 0 aliases"
        );

        // No failed files in happy path
        assert!(
            result.failed_files.is_empty(),
            "Should have no failed files"
        );
    }

    #[test]
    fn test_load_all_aliases_nonexistent_directory() {
        let result = load_all_aliases("nonexistent/directory");

        // Should fail to read directory
        assert!(
            result.failed_files.contains(&"nonexistent/directory".to_string()),
            "Should record nonexistent directory as failed"
        );
        assert!(
            result.aliases.is_empty(),
            "Should have no aliases from failed directory"
        );
    }
}
