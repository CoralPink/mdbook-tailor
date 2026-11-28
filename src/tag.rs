use std::ops::Range;

#[derive(Clone)]
pub enum FenceKind {
    Admonish,
    MediaSlider(Option<String>),
    Other,
}

pub struct FenceFrame {
    pub marker: char,
    pub len: usize,
    pub kind: FenceKind,
    pub open_range: Range<usize>,
}

pub fn fence_open(line: &str) -> Option<(char, usize, &str)> {
    let indent = line.bytes().take_while(|&b| b == b' ').count();

    if indent > 3 {
        return None;
    }

    let rest = &line[indent..];
    let marker = rest.chars().next()?;

    if marker != '`' && marker != '~' {
        return None;
    }

    let len = rest.bytes().take_while(|&b| b == marker as u8).count();

    if len < 3 {
        return None;
    }

    let info = rest[len..].trim();

    // In CommonMark, backquotes cannot be included in the info string of a backquote fence.
    if marker == '`' && info.contains('`') {
        return None;
    }

    Some((marker, len, info))
}

pub fn fence_close(line: &str, marker: char, min_len: usize) -> bool {
    let indent = line.bytes().take_while(|&b| b == b' ').count();

    if indent > 3 {
        return false;
    }

    let rest = &line[indent..];
    let len = rest.bytes().take_while(|&b| b == marker as u8).count();

    len >= min_len && rest[len..].trim().is_empty()
}

pub fn escape_html_attr(value: &str) -> String {
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

pub fn media_slider_open_tag(label: Option<&str>) -> String {
    let mut tag = String::from("<media-slider role=\"group\" aria-roledescription=\"carousel\"");

    if let Some(label) = label {
        tag.push_str(" aria-label=\"");
        tag.push_str(&escape_html_attr(label));
        tag.push('"');
    }

    tag.push('>');
    tag
}

pub fn first_info_word(info: &str) -> &str {
    info.split_ascii_whitespace().next().unwrap_or("")
}

pub fn is_div_wrapper_line(line: &str) -> bool {
    let tag = line.trim();

    if tag == "</div>" {
        return true;
    }

    tag.strip_prefix("<div")
        .and_then(|rest| rest.chars().next())
        .is_some_and(|next| (next.is_ascii_whitespace() || next == '>') && tag.ends_with('>'))
}

pub fn media_slider_label(info: &str) -> Option<String> {
    if first_info_word(info) != "media-slider" {
        return None;
    }

    let attrs = info.strip_prefix("media-slider").unwrap_or("").trim_start();

    let value = attrs.split_once("aria-label=")?.1.trim_start().strip_prefix('"')?;

    let end = value.find('"')?;
    Some(value[..end].to_owned())
}

pub fn prepare_media_slider(content: &str) -> (String, Vec<(Range<usize>, String)>) {
    let mut parser_bytes = content.as_bytes().to_vec();
    let mut blank_ranges = Vec::new();
    let mut replacements = Vec::new();
    let mut stack: Vec<FenceFrame> = Vec::new();
    let mut offset = 0;

    for line in content.split_inclusive('\n') {
        let without_lf = line.strip_suffix('\n').unwrap_or(line);
        let body = without_lf.strip_suffix('\r').unwrap_or(without_lf);
        let line_range = offset..offset + body.len();

        if let Some(frame) = stack.pop_if(|frame| fence_close(body, frame.marker, frame.len)) {
            match frame.kind {
                FenceKind::Admonish => {
                    blank_ranges.push(frame.open_range);
                    blank_ranges.push(line_range);
                }
                FenceKind::MediaSlider(label) => {
                    blank_ranges.push(frame.open_range.clone());
                    blank_ranges.push(line_range.clone());

                    replacements.push((frame.open_range, media_slider_open_tag(label.as_deref())));
                    replacements.push((line_range, "</media-slider>".to_owned()));
                }
                FenceKind::Other => {}
            }

            offset += line.len();
            continue;
        }

        if stack
            .last()
            .is_some_and(|frame| matches!(&frame.kind, FenceKind::Other))
        {
            offset += line.len();
            continue;
        }

        // CommonMark treats a `<div>` block as raw HTML, so Markdown inside it
        // (including images that need measuring) is never emitted as parser events.
        // Hide standalone div wrapper lines while parsing and restore them by offset.
        if stack
            .iter()
            .any(|frame| matches!(&frame.kind, FenceKind::MediaSlider(_)))
        {
            if is_div_wrapper_line(body) {
                parser_bytes[line_range.clone()].fill(b' ');
                replacements.push((line_range, body.to_owned()));
                offset += line.len();
                continue;
            }
        }

        if let Some((marker, len, info)) = fence_open(body) {
            let parent = stack.last().map(|frame| &frame.kind);

            let kind = match parent {
                // Within `admonish`, it recognizes `media-slider` and standard code fences.
                Some(FenceKind::Admonish) => {
                    if first_info_word(info) == "media-slider" {
                        FenceKind::MediaSlider(media_slider_label(info))
                    } else {
                        FenceKind::Other
                    }
                }

                // Code fences within `media-slider` should be protected as regular code.
                Some(FenceKind::MediaSlider(_)) => FenceKind::Other,

                // Do not look for nested syntax within standard code fences.
                Some(FenceKind::Other) => {
                    offset += line.len();
                    continue;
                }

                // Recognize “admonish” and “media-slider” at the top level of the document.
                None if first_info_word(info) == "admonish" => FenceKind::Admonish,
                None if first_info_word(info) == "media-slider" => FenceKind::MediaSlider(media_slider_label(info)),
                None => FenceKind::Other,
            };

            stack.push(FenceFrame {
                marker,
                len,
                kind,
                open_range: line_range,
            });
        }

        offset += line.len();
    }

    // By replacing the marker lines with spaces of the same number of bytes, the parser's offset will match that of the original Markdown.
    for range in blank_ranges {
        parser_bytes[range].fill(b' ');
    }

    (
        String::from_utf8(parser_bytes).expect("replacing ASCII fence lines preserves UTF-8"),
        replacements,
    )
}
