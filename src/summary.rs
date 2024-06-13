use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use anyhow::Error;

// Function to parse SUMMARY.md and collect .md files
pub fn get_summary_md_files(summary_path: &Path) -> Result<HashSet<PathBuf>, Error> {
    let file = File::open(summary_path)?;
    let reader = io::BufReader::new(file);
    let mut summary_md_files = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().starts_with("* [") {
            if let Some(link) = line.split(']').nth(1) {
                let link = link.trim();
                if link.starts_with('(') && link.ends_with(')') {
                    let link = &link[1..link.len() - 1];
                    let path = summary_path.parent().unwrap().join(link);
                    summary_md_files.insert(path);
                }
            }
        }
    }

    Ok(summary_md_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::fs::File;
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

        let md_files = get_summary_md_files(&summary_path).unwrap();

        let expected_files: HashSet<PathBuf> = vec![
                summary_path.parent().unwrap().join("intro.md"),
                summary_path.parent().unwrap().join("chapter_1.md"),
                summary_path.parent().unwrap().join("chapter_1/section_1.md"),
            ]
            .into_iter()
            .collect();

        assert_eq!(md_files, expected_files);
    }
}
