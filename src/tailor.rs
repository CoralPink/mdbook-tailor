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
            let dir = match Path::new(chap.path.as_ref().unwrap()).parent(){
                Some(p) => p,
                None => Path::new(""),
            };

            chap.content = TAILOR_RE
                .replace_all(&chap.content, |caps: &regex::Captures| {
                    let path = caps.name("path").unwrap().as_str();

                    match open(Path::new(&src).join(dir.join(path))) {
                        Ok(image) => {
                            format!(
                                "<img src=\"{}\" alt=\"{}\" width=\"{}\" height=\"{}\" loading=\"lazy\">",
                                path,
                                caps.name("alt").unwrap().as_str(),
                                image.width(),
                                image.height(),
                            )
                        }
                        Err(_) => {
                            eprintln!("Warning: Tailor could not find: {}/{}", dir.display(), path);
                            chap.content.clone()
                        },
                    }
                })
                .to_string();
        }
    });
    Ok(book)
}
