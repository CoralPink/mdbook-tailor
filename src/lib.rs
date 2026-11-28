mod buf;
mod image;
mod tag;

use std::{mem, ops::Range, path::Path};

use mdbook_preprocessor::{
    book::{Book, BookItem},
    errors::Error,
};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

use crate::{image::replace_image, tag::prepare_media_slider};

fn process_image_events<'a, I>(src: &Path, mdfile: &Path, mut events: I, replacements: &mut Vec<(Range<usize>, String)>)
where
    I: Iterator<Item = (Event<'a>, Range<usize>)>,
{
    let mut count = 0;

    while let Some((event, range)) = events.next() {
        let Event::Start(Tag::Image { dest_url, title, .. }) = event else {
            continue;
        };

        let image_range = range;
        let url = dest_url.to_string();

        // Constructing `alt` text from an event's sub-events
        let mut alt = String::new();
        let mut nested_images = 0;

        for (child, _) in events.by_ref() {
            match child {
                Event::Start(Tag::Image { .. }) => {
                    nested_images += 1;
                }
                Event::End(TagEnd::Image) if nested_images == 0 => {
                    break;
                }
                Event::End(TagEnd::Image) => {
                    nested_images -= 1;
                }
                Event::Text(text) | Event::Code(text) => {
                    alt.push_str(&text);
                }
                Event::InlineMath(text) => {
                    alt.push('$');
                    alt.push_str(&text);
                    alt.push('$');
                }
                Event::DisplayMath(text) => {
                    alt.push_str("$$");
                    alt.push_str(&text);
                    alt.push_str("$$");
                }
                Event::SoftBreak | Event::HardBreak => {
                    alt.push(' ');
                }
                _ => {}
            }
        }

        count += 1;
        replace_image(src, mdfile, &url, &alt, &title, image_range, count, replacements);
    }
}

pub fn measure(src: impl AsRef<Path>, mut book: Book) -> Result<Book, Error> {
    book.for_each_mut(|item| {
        if let BookItem::Chapter(chap) = item {
            let content = mem::take(&mut chap.content);
            let (parser_content, mut replacements) = prepare_media_slider(&content);

            let mdfile = chap.path.as_ref().map_or(Path::new(""), |p| p.as_path());
            let parser = Parser::new_ext(&parser_content, Options::all()).into_offset_iter();
            process_image_events(src.as_ref(), mdfile, parser, &mut replacements);

            chap.content = apply_replacements(&content, replacements);
        }
    });

    Ok(book)
}

fn apply_replacements(content: &str, mut replacements: Vec<(Range<usize>, String)>) -> String {
    // Only the specified ranges are replaced.
    replacements.sort_by_key(|(range, _)| range.start);

    let mut output = String::with_capacity(content.len());
    let mut copied_until = 0;

    for (range, replacement) in replacements {
        output.push_str(&content[copied_until..range.start]);
        output.push_str(&replacement);
        copied_until = range.end;
    }

    output.push_str(&content[copied_until..]);
    output
}
