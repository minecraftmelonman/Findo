// -------------------CRATES:-------------------
use jwalk::WalkDir; // file directory crate
use std::path::{Path, PathBuf};

pub fn search_files(target_folder: &Path, cleaned_search: &str) -> Vec<PathBuf> {
    let search_lower = cleaned_search.to_lowercase();
    let search_bytes = search_lower.as_bytes();

    WalkDir::new(target_folder)
        .skip_hidden(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| {
            if entry.file_type().is_file() {
                let name_bytes = entry.file_name().as_encoded_bytes();
                name_bytes
                    .windows(search_bytes.len())
                    .any(|w| w.eq_ignore_ascii_case(search_bytes))
            } else {
                false
            }
        })
        .map(|entry| entry.path())
        .collect()
}