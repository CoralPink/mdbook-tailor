use crate::tailor_lib::Tailor;

use clap::{Arg, ArgMatches, Command};
use mdbook_preprocessor::{
    book::Book,
    errors::Result,
    {Preprocessor, PreprocessorContext},
};
use semver::{Version, VersionReq};
use std::{io, process};

const DEFAULT_BOOK_SRC: &str = "src";

mod tailor_lib {
    use super::*;

    pub struct Tailor;

    impl Tailor {
        pub fn new() -> Tailor {
            Tailor
        }
    }

    impl Preprocessor for Tailor {
        fn name(&self) -> &str {
            "tailor-preprocessor"
        }

        fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
            mdbook_tailor::measure(ctx.root.join(DEFAULT_BOOK_SRC.to_string()), book)
        }

        fn supports_renderer(&self, renderer: &str) -> Result<bool> {
            Ok(renderer != "not-supported")
        }
    }
}

pub fn make_app() -> Command {
    Command::new("tailor-preprocessor")
        .about("An mdbook preprocessor which converts expands tailor markers")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.get_one::<String>("renderer").expect("Required argument");

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if pre.supports_renderer(renderer).unwrap() {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<()> {
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook_preprocessor::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, but we're being called from version {}",
            pre.name(),
            mdbook_preprocessor::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn main() {
    let matches = make_app().get_matches();
    let preprocessor = Tailor::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{e}");
        process::exit(1);
    }
}
