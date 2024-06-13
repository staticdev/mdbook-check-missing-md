use mdbook_check_missing_md::run;
use anyhow::{Context, Error};
use std::{io, path::PathBuf};
use structopt::StructOpt;
use mdbook::{renderer::RenderContext, MDBook};

fn main() -> Result<(), Error> {
    // Parse command-line arguments
    let args = Args::from_args();

    // Determine the RenderContext based on standalone or plugin mode
    let ctx: RenderContext = if args.standalone {
        // Load MDBook for standalone mode
        let md =
            MDBook::load(dunce::canonicalize(&args.root)?).map_err(to_sync)?;
        let destination = md.build_dir_for("check_missing_md");
        RenderContext::new(md.root, md.book, md.config, destination)
    } else {
        serde_json::from_reader(io::stdin())
            .context("Unable to parse RenderContext")?
    };

    // Check for missing .md files not listed in SUMMARY.md
    run(&ctx)?;

    Ok(())
}

#[derive(Debug, Clone, StructOpt)]
struct Args {
    #[structopt(
        short = "s",
        long = "standalone",
        help = "Run standalone (i.e. not as an mdbook plugin)"
    )]
    standalone: bool,
    #[structopt(
        help = "The book to render.",
        parse(from_os_str),
        default_value = "."
    )]
    root: PathBuf,
}

fn to_sync(err: mdbook::errors::Error) -> Error {
    use std::{
        fmt::{self, Display, Formatter},
        sync::Mutex,
    };

    #[derive(Debug)]
    struct Synchronised(Mutex<mdbook::errors::Error>);

    impl Display for Synchronised {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            self.0.lock().expect("lock was poisoned").fmt(f)
        }
    }

    impl std::error::Error for Synchronised {}

    Error::from(Synchronised(Mutex::new(err)))
}
