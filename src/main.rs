use crate::tailor_lib::Tailor;

use clap::{Arg, ArgMatches, Command};
use lazy_static::lazy_static;
use mdbook::{
    book::Book,
    errors::Error,
    preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext},
};
use regex::Regex;
use semver::{Version, VersionReq};
use std::path::Path;
use std::{io, process};

use image::open;

pub fn make_app() -> Command {
    Command::new("tailor-preprocessor")
        .about("An mdbook preprocessor which converts expands tailor markers")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
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

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");

    let supported = pre.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

lazy_static! {
    static ref TAILOR_RE: Regex = Regex::new(r"!\[(?P<alt>.*)]\(\s*(?P<path>.*)\)").unwrap();
}

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
        fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
            book.for_each_mut(|item| {
                if let mdbook::book::BookItem::Chapter(chap) = item {
                    let parent = Path::new(chap.path.as_ref().unwrap()).parent();

                    let dir = match parent {
                        Some(p) => p,
                        None => Path::new(""),
                    };

                    chap.content = TAILOR_RE
                        .replace_all(&chap.content, |caps: &regex::Captures| {
                            let path = caps.name("path").unwrap().as_str();
                            let alt = caps.name("alt").unwrap().as_str();

                            let image = open(Path::new("src").join(dir.join(path))).unwrap();

                            format!(
                                "<img src={} alt={} width={} height={} />",
                                path,
                                alt,
                                image.width(),
                                image.height(),
                            )
                        })
                        .to_string();
                }
            });
            Ok(book)
        }
        fn supports_renderer(&self, renderer: &str) -> bool {
            renderer != "not-supported"
        }
    }
}
