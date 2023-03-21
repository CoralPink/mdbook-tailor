use crate::tailor_lib::Tailor;

use clap::{Arg, ArgMatches, Command};
use image::open;
use lazy_static::lazy_static;
use mdbook::{
    book::{Book, BookItem},
    errors::Error,
    preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext},
};
use regex::{Regex, RegexBuilder};
use semver::{Version, VersionReq};
use std::path::Path;
use std::{io, process};

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
    }
    else if let Err(e) = handle_preprocessing(&preprocessor) {
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
    }
    else {
        process::exit(1);
    }
}

lazy_static! {
    static ref TAILOR_RE: Regex = RegexBuilder::new(
        r"(?m)(?P<previous_line>^[^|]*\|.*\n)?^(?P<image>!\[(?P<alt>.*)]\((?P<path>[^)]+)\))"
    )
    .multi_line(true)
    .build()
    .unwrap();
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

        fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
            let src = ctx
                .config
                .get("build.src")
                .and_then(|v| v.as_str())
                .unwrap_or("src");

            book.for_each_mut(|item| {
                if let BookItem::Chapter(chap) = item {
                    let parent = Path::new(chap.path.as_ref().unwrap()).parent();

                    let dir = match parent {
                        Some(p) => p,
                        None => Path::new(""),
                    };

                    let mut line_image_count = 0;
                    let mut previous_line_image_count = 0;

                    chap.content = TAILOR_RE
                        .replace_all(&chap.content, |caps: &regex::Captures| {
                            // skip if more than 1 image on line
                            if line_image_count >= 1 {
                                return caps.name("image").unwrap().as_str().to_string();
                            }

                            // skip if the previous line contains an image
                            if let Some(previous_line) = caps.name("previous_line") {
                                let re = Regex::new(r"!\[").unwrap();
                                if re.is_match(previous_line.as_str()) {
                                    previous_line_image_count += 1;
                                }
                            }
                            if previous_line_image_count >= 1 {
                                return caps[0].to_string();
                            }
                            line_image_count += 1;

                            let path = caps.name("path").unwrap().as_str();
                            let alt = caps.name("alt").unwrap().as_str();

                            let image = open(Path::new(&src).join(dir.join(path))).unwrap();

                            format!(
                                "<img src=\"{}\" alt=\"{}\" width=\"{}\" height=\"{}\">",
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
