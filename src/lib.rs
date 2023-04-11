use image::open;
use lazy_static::lazy_static;
use mdbook::{
    book::{Book, BookItem},
    errors::Error,
};
use regex::Regex;
use std::path::Path;

const CLR_RESET: &str = "\x1b[0m";
const CLR_C: &str = "\x1b[36m";
const CLR_M: &str = "\x1b[35m";
const CLR_Y: &str = "\x1b[33m";

lazy_static! {
    static ref TAILOR_RE: Regex =
        Regex::new(r"(?m)^(\s*)!\[(?P<alt>[^]]*)]\((?P<url>[^)]*)\)$")
            .unwrap();
}

pub fn measure(src: &str, mut book: Book) -> Result<Book, Error> {
    book.for_each_mut(|item| {
        if let BookItem::Chapter(chap) = item {
            let mdfile = chap.path.as_ref().map_or("", |p| p.to_str().unwrap_or(""));
            let dir = Path::new(mdfile).parent().unwrap_or_else(|| Path::new(""));

            chap.content = TAILOR_RE
                .replace_all(&chap.content, |caps: &regex::Captures| {
                    let url = caps.name("url").unwrap().as_str();
                    let path = Path::new(&src).join(dir.join(url));

                    match open(&path) {
                        Ok(image) => {
                            format!(
                                "<img src=\"{}\" alt=\"{}\" width=\"{}\" height=\"{}\" loading=\"lazy\">",
                                url,
                                caps.name("alt").unwrap().as_str(),
                                image.width(),
                                image.height(),
                            )
                        }
                        Err(_) => {
                            eprintln!("{CLR_Y}[Warning]{CLR_RESET} Tailor could not find: {CLR_M}{}{CLR_RESET} From {CLR_C}{}{CLR_RESET}",
                                path.display(),
                                mdfile
                            );
                            String::from(caps.get(0).map_or("", |x| x.as_str()))
                        },
                    }
                })
                .to_string();
        }
    });
    Ok(book)
}
