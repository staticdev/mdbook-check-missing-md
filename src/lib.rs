pub mod md_files;
pub mod summary;

use std::collections::HashSet;

use mdbook::renderer::RenderContext;
use anyhow::Error;

use crate::md_files::collect_md_files;
use crate::summary::get_summary_md_files;

pub fn run(ctx: &RenderContext) -> Result<(), Error> {
    let root = ctx.root.join("src");
    let summary_path = root.join("SUMMARY.md");

    let mut all_md_files = HashSet::new();
    collect_md_files(&root, &mut all_md_files)?;

    // Exclude SUMMARY.md from the set of markdown files
    all_md_files.remove(&summary_path.canonicalize()?);

    let summary_md_files = get_summary_md_files(&summary_path)?;

    let missing_in_summary: HashSet<_> = all_md_files.difference(&summary_md_files).collect();

    if !missing_in_summary.is_empty() {
        eprintln!("Error: The following .md files are not listed in SUMMARY.md:");
        for file in missing_in_summary {
            eprintln!("{}", file.display());
        }
        std::process::exit(1);
    }

    println!("All files are listed in SUMMARY.md");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    
    use tempfile::tempdir;
    use mdbook::renderer::RenderContext;
    use mdbook::book::Book;
    use mdbook::config::Config;
    
    #[test]
    fn test_check_missing() {
        let dir = tempdir().unwrap();
        let src_dir = dir.path().join("src");
        let summary_path = src_dir.join("SUMMARY.md");
    
        // Create files and directories
        fs::create_dir_all(&src_dir).unwrap();
        let md_files = vec!["intro.md", "chapter_1.md", "chapter_1/section_1.md"];
        for file in &md_files {
            let file_path = src_dir.join(file);
            fs::create_dir_all(file_path.parent().unwrap()).unwrap();
            File::create(&file_path).unwrap();
        }
    
        // Create SUMMARY.md
        let summary_content = r#"
        * [Introduction](intro.md)
        * [Chapter 1](chapter_1.md)
            * [Section 1](chapter_1/section_1.md)
        "#;
        let mut summary_file = File::create(&summary_path).unwrap();
        summary_file.write_all(summary_content.as_bytes()).unwrap();
    
        let ctx = RenderContext::new(
            dir.path().to_path_buf(),
            Book::new(),
            Config::default(),
            dir.path().to_path_buf()
        );
    
        let result = run(&ctx);
    
        assert!(result.is_ok());
    }
}