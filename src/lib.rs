use core::fmt::NumBuffer;
use mdbook_preprocessor::{
    book::{Book, BookItem},
    errors::Error,
};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::{mem, ops::Range, path::Path};

const CLR_RESET: &str = "\x1b[0m";
const CLR_C: &str = "\x1b[36m";
const CLR_M: &str = "\x1b[35m";
const CLR_Y: &str = "\x1b[33m";

const IMG_LOADING_LAZY: &str = r#"loading="lazy""#;
const IMG_FETCHPRIORITY_HIGH: &str = r#"fetchpriority="high""#;

struct WriteBuf {
    buf: String,
    num_buf: NumBuffer<usize>,
}

impl WriteBuf {
    fn new() -> Self {
        Self {
            buf: String::new(),
            num_buf: NumBuffer::new(),
        }
    }

    fn push_char(&mut self, c: char) {
        self.buf.push(c);
    }

    fn push_str(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    fn push_usize(&mut self, v: usize) {
        self.buf.push_str(v.format_into(&mut self.num_buf));
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

fn escape_html_attr(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());

    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            _ => escaped.push(ch),
        }
    }

    escaped
}

pub fn measure(src: impl AsRef<Path>, mut book: Book) -> Result<Book, Error> {
    let src = src.as_ref();

    book.for_each_mut(|item| {
        if let BookItem::Chapter(chap) = item {
            let mdfile = chap.path.as_ref().map_or(Path::new(""), |p| p.as_path());
            let dir = mdfile.parent().unwrap_or_else(|| Path::new(""));
            let content = mem::take(&mut chap.content);

            let mut replacements: Vec<(Range<usize>, String)> = Vec::new();
            let mut count = 0;

            let parser = Parser::new_ext(&content, Options::all()).into_offset_iter();
            let mut events = parser.peekable();

            while let Some((event, range)) = events.next() {
                let Event::Start(Tag::Image { dest_url, .. }) = event else {
                    continue;
                };

                let image_range = range;
                let url = dest_url.to_string();

                // Constructing `alt` text from an event's sub-events
                let mut alt = String::new();
                for (child, _) in events.by_ref() {
                    match child {
                        Event::End(TagEnd::Image) => break,
                        Event::Text(text) | Event::Code(text) => alt.push_str(&text),
                        Event::SoftBreak | Event::HardBreak => alt.push(' '),
                        _ => {}
                    }
                }

                count += 1;

                let path = src.join(dir).join(&url);
                match imagesize::size(&path) {
                    Ok(size) => {
                        let mut out = WriteBuf::new();
                        build_img_tag(
                            &mut out,
                            &escape_html_attr(&url),
                            &escape_html_attr(&alt),
                            size.width,
                            size.height,
                            count,
                        );

                        replacements.push((image_range, out.into_string()));
                    }
                    Err(_) => {
                        // Images that failed to process will not be added to the replacement list and will remain in Markdown format.
                        eprintln!(
                            "{CLR_Y}[Warning]{CLR_RESET} Tailor could not find: {CLR_M}{}{CLR_RESET} From {CLR_C}{}{CLR_RESET}",
                            path.display(),
                            mdfile.display()
                        );
                    }
                }
            }

            // Only the specified range will be replaced.
            let mut output = String::with_capacity(content.len());
            let mut copied_until = 0;

            for (range, replacement) in replacements {
                output.push_str(&content[copied_until..range.start]);
                output.push_str(&replacement);
                copied_until = range.end;
            }

            output.push_str(&content[copied_until..]);
            chap.content = output;
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
