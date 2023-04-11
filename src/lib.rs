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

#[cfg(test)]
mod tests {
    use crate::measure;
    use mdbook::book::Book;
    use mdbook::book::{BookItem, Chapter};
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    const CLR_RESET: &str = "\x1b[0m";
    const CLR_R: &str = "\x1b[31m";

    const TEST_DIR: &str = "test/";
    const TEST_MD: &str = "test.md";

    const RESULT_OK: &str = "ok.md";
    const RESULT_NG_OUTPUT: &str = "result-ng.md";

    fn write_chapters_to_files(chap: &Chapter) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(String::from(TEST_DIR) + RESULT_NG_OUTPUT)?;
        file.write_all(chap.content.as_bytes())?;

        Ok(())
    }

    #[test]
    fn test_measure() {
        let mut book = Book::new();

        book.push_item(BookItem::Chapter(Chapter::new(
            "Test Chapter",
            fs::read_to_string(String::from(TEST_DIR) + TEST_MD).unwrap(),
            std::path::Path::new(TEST_MD),
            vec![],
        )));

        match measure(TEST_DIR, book) {
            Ok(book) => {
                for item in book.iter() {
                    if let BookItem::Chapter(chap) = item {
                        let result_ok =
                            fs::read_to_string(String::from(TEST_DIR) + RESULT_OK).unwrap();

                        if chap.content != result_ok {
                            write_chapters_to_files(chap)
                                .unwrap_or_else(|err| panic!("{CLR_R}ERROR{CLR_RESET}: {}", err));

                            panic!("{CLR_R}[FAILED]{CLR_RESET} The conversion was not done correctly.");
                        }
                    }
                }
            }
            Err(err) => {
                panic!("ERROR: {}", err);
            }
        }
    }
}
