# mdbook-check-missing-md

A backend for `mdbook` which will find Markdowns you forgot on SUMMARY.md.

## Installation

First you'll need to install `mdbook-check-missing-md`:

```sh
cargo install mdbook-check-missing-md
```

To use this preprocessor, add it to your `book.toml` file:

```toml
[output.check-missing-md]
```

Then run:

```sh
mdbook build
```

## Development

Build and run tests with:

```sh
cargo build
cargo test
```

## Credits

This plugin was heavily inspired by [mdbook-linkcheck](https://github.com/Michael-F-Bryan/mdbook-linkcheck/tree/master).
