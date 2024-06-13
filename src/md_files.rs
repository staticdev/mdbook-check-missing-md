use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Error;

pub fn collect_md_files(dir: &Path, md_files: &mut HashSet<PathBuf>) -> Result<(), Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_md_files(&path, md_files)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            md_files.insert(path);
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;
    use std::fs::{self, File};
    use std::path::PathBuf;
    
    use tempfile::tempdir;
    
    #[test]
    fn test_collect_md_files() {
        let dir = tempdir().unwrap();
        let file1 = dir.path().join("file1.md");
        let file2 = dir.path().join("nested").join("file2.md");
    
        fs::create_dir_all(file2.parent().unwrap()).unwrap();
        File::create(&file1).unwrap();
        File::create(&file2).unwrap();
    
        let mut md_files = HashSet::new();
        collect_md_files(dir.path(), &mut md_files).unwrap();
    
        let expected_files: HashSet<PathBuf> = vec![
            file1, file2
        ]
        .into_iter()
        .collect();
    
        assert_eq!(md_files, expected_files);
    }
}