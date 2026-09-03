// -------------------CRATES:-------------------
use jwalk::WalkDir; // file directory crate
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub fn search_files(
    target_folder: &Path,
    cleaned_search: &str,
    ext_search: &str,
    counter: Arc<AtomicUsize>,
) -> Vec<PathBuf> {
    let search_lower = cleaned_search.to_lowercase();
    let ext_lower = ext_search.trim().trim_start_matches('.').to_lowercase();

    WalkDir::new(target_folder)
        .skip_hidden(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .inspect(|_| {
            counter.fetch_add(1, Ordering::Relaxed);
        })
        .filter(|entry| {
            if entry.file_type().is_file() {
                let path = entry.path();

                if !ext_lower.is_empty() {
                    let matches_ext = path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext_str| ext_str.to_lowercase() == ext_lower)
                        .unwrap_or(false);

                    if !matches_ext {
                        return false;
                    }
                }

                let name = entry.file_name().to_string_lossy().to_lowercase();
                name.contains(&search_lower)
            } else {
                false
            }
        })
        .map(|entry| entry.path())
        .collect()
}