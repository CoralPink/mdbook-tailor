use image::open;
use lazy_static::lazy_static;
use mdbook::{
    book::{Book, BookItem},
    errors::Error,
};
use regex::Regex;
use std::path::Path;

lazy_static! {
    static ref TAILOR_RE: Regex =
        Regex::new(r"(?m)^(\s*)!\[(?P<alt>[^]]*)]\((?P<path>[^)]*)\)$").unwrap();
}

pub fn measure(src: &str, mut book: Book) -> Result<Book, Error> {
    book.for_each_mut(|item| {
        if let BookItem::Chapter(chap) = item {
            let parent = Path::new(chap.path.as_ref().unwrap()).parent();

            let dir = match parent {
                Some(p) => p,
                None => Path::new(""),
            };

            chap.content = TAILOR_RE
                .replace_all(&chap.content, |caps: &regex::Captures| {
                    let path = caps.name("path").unwrap().as_str();
                    let alt = caps.name("alt").unwrap().as_str();

                    let image = open(Path::new(&src).join(dir.join(path))).unwrap();

                    format!(
                        "<img src=\"{}\" alt=\"{}\" width=\"{}\" height=\"{}\" loading=\"lazy\">",
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
