use std::{ops::Range, path::Path};

use crate::{buf::WriteBuf, tag::escape_html_attr};

const CLR_RESET: &str = "\x1b[0m";
const CLR_C: &str = "\x1b[36m";
const CLR_M: &str = "\x1b[35m";
const CLR_Y: &str = "\x1b[33m";

const IMG_LOADING_LAZY: &str = r#"loading="lazy""#;
const IMG_FETCHPRIORITY_HIGH: &str = r#"fetchpriority="high""#;

fn build_img_tag(buf: &mut WriteBuf, url: &str, alt: &str, title: &str, width: usize, height: usize, count: u32) {
    buf.push_str("<img src=\"");
    buf.push_str(url);
    buf.push_str("\" alt=\"");
    buf.push_str(alt);

    if !title.is_empty() {
        buf.push_str("\" title=\"");
        buf.push_str(&escape_html_attr(title));
    }

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

pub(crate) fn replace_image(
    src: &Path,
    mdfile: &Path,
    url: &str,
    alt: &str,
    title: &str,
    image_range: Range<usize>,
    count: u32,
    replacements: &mut Vec<(Range<usize>, String)>,
) {
    let dir = mdfile.parent().unwrap_or_else(|| Path::new(""));
    let path = src.join(dir).join(url);

    match imagesize::size(&path) {
        Ok(size) => {
            let mut out = WriteBuf::new();

            build_img_tag(
                &mut out,
                &escape_html_attr(url),
                &escape_html_attr(alt),
                title,
                size.width,
                size.height,
                count,
            );

            replacements.push((image_range, out.into_string()));
        }
        Err(_) => {
            // Images that failed to process will not be added to the replacement list and will remain in Markdown format.
            eprintln!(
                "{CLR_Y}[Warning]{CLR_RESET} Tailor could not find: \
                 {CLR_M}{}{CLR_RESET} From {CLR_C}{}{CLR_RESET}",
                path.display(),
                mdfile.display()
            );
        }
    }
}
