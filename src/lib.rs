use mdbook_preprocessor::{
    book::{Book, BookItem},
    errors::Error,
};

use regex::Regex;
use std::{mem, path::Path, sync::LazyLock};

const CLR_RESET: &str = "\x1b[0m";
const CLR_C: &str = "\x1b[36m";
const CLR_M: &str = "\x1b[35m";
const CLR_Y: &str = "\x1b[33m";

const IMG_LOADING_LAZY: &str = r#"loading="lazy""#;
const IMG_FETCHPRIORITY_HIGH: &str = r#"fetchpriority="high""#;

static TAILOR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"!\[(?P<alt>[^\]]*)]\((?P<url>[^\)]*)\)").expect("Invalid regex for TAILOR_RE"));

struct WriteBuf {
    buf: String,
    itoa: itoa::Buffer,
}

impl WriteBuf {
    fn new() -> Self {
        Self {
            buf: String::new(),
            itoa: itoa::Buffer::new(),
        }
    }

    fn push_char(&mut self, c: char) {
        self.buf.push(c);
    }

    fn push_str(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    fn push_usize(&mut self, v: usize) {
        self.buf.push_str(self.itoa.format(v));
    }

    fn into_string(self) -> String {
        self.buf
    }
}

fn build_img_tag(buf: &mut WriteBuf, url: &str, alt: &str, width: usize, height: usize, count: u32) {
    buf.push_str("<img src=\"");
    buf.push_str(url);
    buf.push_str("\" alt=\"");
    buf.push_str(alt);
    buf.push_str("\" width=\"");
    buf.push_usize(width);
    buf.push_str("\" height=\"");
    buf.push_usize(height);
    buf.push_str("\"");
    buf.push_char(' ');

    if count == 1 {
        buf.push_str(IMG_FETCHPRIORITY_HIGH);
    } else {
        buf.push_str(IMG_LOADING_LAZY);
    }

    buf.push_char('>');
}

pub fn measure(src: impl AsRef<Path>, mut book: Book) -> Result<Book, Error> {
    let src = src.as_ref();

    book.for_each_mut(|item| {
        if let BookItem::Chapter(chap) = item {
            let mdfile = chap.path.as_ref().map_or(Path::new(""), |p| p.as_path());
            let dir = mdfile.parent().unwrap_or_else(|| Path::new(""));

            let mut count = 0;

            let content = mem::take(&mut chap.content);

            chap.content = TAILOR_RE
                .replace_all(&content, |caps: &regex::Captures| {
                    let url = caps.name("url").unwrap().as_str();
                    let path = src.join(dir).join(url);

                    count += 1;

                    match imagesize::size(&path) {
                        Ok(size) => {
                            let mut out = WriteBuf::new();

                            build_img_tag(
                                &mut out,
                                url,
                                caps.name("alt").unwrap().as_str(),
                                size.width,
                                size.height,
                                count,
                            );

                            out.into_string()
                        }
                        Err(_) => {
                            eprintln!(
                                "{CLR_Y}[Warning]{CLR_RESET} Tailor could not find: {CLR_M}{}{CLR_RESET} From {CLR_C}{}{CLR_RESET}",
                                path.display(),
                                mdfile.display()
                            );
                            caps.get(0).map_or("", |x| x.as_str()).to_string()
                        }
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
    use mdbook_preprocessor::book::{Book, BookItem, Chapter};
    use pretty_assertions::assert_eq;
    use std::{fs, fs::File, io::Write, path::Path};

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
            Path::new(TEST_MD),
            vec![],
        )));

        println!("{CLR_C}[INFO]{CLR_RESET} Depending on the test case, [WARNING] may be displayed.");

        match measure(TEST_DIR, book) {
            Ok(book) => {
                for item in book.iter() {
                    if let BookItem::Chapter(chap) = item {
                        write_chapters_to_files(chap).unwrap_or_else(|err| panic!("{CLR_R}ERROR{CLR_RESET}: {err}"));
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
