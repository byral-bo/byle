use std::path::{Path, PathBuf};

/// Find the project root by walking up from `start` looking for by.toml.
pub fn find_project_root(start: &Path) -> Option<PathBuf> {
    let mut current = Some(start.to_path_buf());
    while let Some(dir) = current {
        if dir.join("by.toml").exists() {
            return Some(dir);
        }
        current = dir.parent().map(|p| p.to_path_buf());
    }
    None
}

/// Locate the main source file for a project.
pub fn main_source_path(project_root: &Path) -> Option<PathBuf> {
    // Try app.byx first (root-level UI entry), then src/main.byx, then src/main.by
    let app_byx = project_root.join("app.byx");
    if app_byx.exists() {
        return Some(app_byx);
    }
    let by_path = project_root.join("src").join("main.by");
    if by_path.exists() {
        Some(by_path)
    } else {
        None
    }
}

/// Locate test files in a project.
pub fn test_source_paths(project_root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let tests_dir = project_root.join("tests");
    if tests_dir.is_dir()
        && let Ok(entries) = std::fs::read_dir(&tests_dir)
    {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "by") {
                paths.push(path);
            }
        }
    }
    paths
}
