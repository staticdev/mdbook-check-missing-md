use std::collections::HashSet;
use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};

use anyhow::Error;

// Function to parse SUMMARY.md and collect .md files
pub fn get_summary_md_files(summary_path: &Path) -> Result<HashSet<PathBuf>, Error> {
    let file = fs::File::open(summary_path)?;
    let reader = std::io::BufReader::new(file);
    let mut summary_md_files = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(start) = line.find('(') {
            if let Some(end) = line.find(')') {
                let link = &line[start + 1..end];
                let path = summary_path.parent().unwrap().join(link);
                summary_md_files.insert(path.canonicalize()?);
            }
        }
    }

    Ok(summary_md_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    use tempfile::tempdir;

    #[test]
    fn test_get_summary_md_files() {
        let dir = tempdir().unwrap();
        let summary_path = dir.path().join("SUMMARY.md");

        let summary_content = r#"
        * [Introduction](intro.md)
        * [Chapter 1](chapter_1.md)
            * [Section 1](chapter_1/section_1.md)
        "#;

        let mut file = File::create(&summary_path).unwrap();
        file.write_all(summary_content.as_bytes()).unwrap();

        // Create the necessary directories and files so canonicalize works
        fs::create_dir_all(dir.path().join("chapter_1")).unwrap();
        File::create(dir.path().join("intro.md")).unwrap();
        File::create(dir.path().join("chapter_1.md")).unwrap();
        File::create(dir.path().join("chapter_1/section_1.md")).unwrap();

        let md_files = get_summary_md_files(&summary_path).unwrap();

        let expected_files: HashSet<PathBuf> = vec![
            summary_path.parent().unwrap().join("intro.md").canonicalize().unwrap(),
            summary_path.parent().unwrap().join("chapter_1.md").canonicalize().unwrap(),
            summary_path.parent().unwrap().join("chapter_1/section_1.md").canonicalize().unwrap(),
        ]
        .into_iter()
        .collect();

        assert_eq!(md_files, expected_files);
    }
}
