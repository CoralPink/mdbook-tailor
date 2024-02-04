use image::open;
use mdbook::{
    book::{Book, BookItem},
    errors::Error,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::path::Path;

const CLR_RESET: &str = "\x1b[0m";
const CLR_C: &str = "\x1b[36m";
const CLR_M: &str = "\x1b[35m";
const CLR_Y: &str = "\x1b[33m";

const IMG_LAZY: &str = "loading=\"lazy\"";
const IMG_ASYNC: &str = "decoding=\"async\"";

static TAILOR_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^(\s*)!\[(?P<alt>[^]]*)]\((?P<url>[^)]*)\)$")
        .expect("Invalid regex for TAILOR_RE")
});

fn format_img_tag(url: &str, alt: &str, width: u32, height: u32, count: u32) -> String {
    let param = if count > 1 { IMG_LAZY } else { IMG_ASYNC };
    format!("<img src=\"{url}\" alt=\"{alt}\" width=\"{width}\" height=\"{height}\" {param}>")
}

pub fn measure(src: &str, mut book: Book) -> Result<Book, Error> {
    book.for_each_mut(|item| {
        if let BookItem::Chapter(chap) = item {
            let mdfile = chap.path.as_ref().map_or("", |p| p.to_str().unwrap_or(""));
            let dir = Path::new(mdfile).parent().unwrap_or_else(|| Path::new(""));

            let mut count = 0;

            chap.content = TAILOR_RE
                .replace_all(&chap.content, |caps: &regex::Captures| {
                    let url = caps.name("url").unwrap().as_str();
                    let path = Path::new(&src).join(dir.join(url));
                    count += 1;

                    match open(&path) {
                        Ok(image) => {
                            format_img_tag(
                                url,
                                caps.name("alt").unwrap().as_str(),
                                image.width(),
                                image.height(),
                                count)
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
    use mdbook::book::{Book, BookItem, Chapter};
    use pretty_assertions::assert_eq;
    use std::{fs, fs::File, io::Write};

    const CLR_RESET: &str = "\x1b[0m";
    const CLR_R: &str = "\x1b[31m";
    const CLR_C: &str = "\x1b[36m";

    const TEST_DIR: &str = "test/";
    const TEST_MD: &str = "test.md";

    const OK_RESULT: &str = "ok.md";
    const OUTPUT_RESULT: &str = "result.md";

    fn write_chapters_to_files(chap: &Chapter) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(String::from(TEST_DIR) + OUTPUT_RESULT)?;
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

        println!(
            "{CLR_C}[INFO]{CLR_RESET} Depending on the test case, [WARNING] may be displayed."
        );

        match measure(TEST_DIR, book) {
            Ok(book) => {
                for item in book.iter() {
                    if let BookItem::Chapter(chap) = item {
                        write_chapters_to_files(chap)
                            .unwrap_or_else(|err| panic!("{CLR_R}ERROR{CLR_RESET}: {err}"));
                        assert_eq!(
                            chap.content,
                            fs::read_to_string(String::from(TEST_DIR) + OK_RESULT).unwrap()
                        );
                    }
                }
            }
            Err(err) => {
                panic!("ERROR: {err}");
            }
        }
    }
}
