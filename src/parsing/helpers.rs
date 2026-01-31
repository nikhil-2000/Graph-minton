use std::fs;
pub fn entry_to_path(entry_res: Result<fs::DirEntry, std::io::Error>) -> Option<std::path::PathBuf> {
    entry_res.ok().and_then(|entry| {
        let path = entry.path();
        path.is_file().then_some(path)
    })
}

pub fn remove_empty_lines(line: &str) -> Option<String> {
    let trimmed = line.trim();
    (!trimmed.is_empty()).then_some(trimmed.to_string())
}