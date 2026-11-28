use mdbook_preprocessor::book::{Book, BookItem, Chapter};
use mdbook_tailor::measure;
use pretty_assertions::assert_eq;
use std::{fs, path::Path};

const TEST_DIR: &str = "tests/fixtures/";
const TEST_MD: &str = "test.md";
const EXPECTED_MD: &str = "ok.md";

fn measure_content(content: &str) -> String {
    let mut book = Book::new();
    book.push_item(BookItem::Chapter(Chapter::new(
        "Test Chapter",
        content.to_owned(),
        Path::new(TEST_MD),
        vec![],
    )));

    let book = measure(TEST_DIR, book).expect("measure should succeed");

    book.iter()
        .find_map(|item| match item {
            BookItem::Chapter(chapter) => Some(chapter.content.clone()),
            _ => None,
        })
        .expect("book should contain a chapter")
}

#[test]
fn matches_the_full_book_fixture() {
    let input = fs::read_to_string(format!("{TEST_DIR}{TEST_MD}")).unwrap();
    let expected = fs::read_to_string(format!("{TEST_DIR}{EXPECTED_MD}")).unwrap();

    assert_eq!(measure_content(&input), expected);
}

#[test]
fn preserves_and_escapes_image_title() {
    let output = measure_content(r#"![caption](test1.png 'A "quote" & <tag>')"#);

    assert!(output.contains(
        r#"<img src="test1.png" alt="caption" title="A &quot;quote&quot; &amp; &lt;tag&gt;" width="50" height="28" fetchpriority="high">"#
    ));
}

#[test]
fn omits_empty_image_title() {
    let output = measure_content(r#"![empty title](test1.png "")"#);

    assert!(output.contains(r#"<img src="test1.png" alt="empty title" width="50" height="28" fetchpriority="high">"#));
    assert!(!output.contains(r#"title=""#));
}

#[test]
fn handles_nested_images_and_following_sibling() {
    let input =
        "![before ![middle ![inner](test3.gif) after inner](test2.jpg) after middle](test1.png)![sibling](test2.jpg)";
    let output = measure_content(input);

    assert!(output.contains(
        r#"<img src="test1.png" alt="before middle inner after inner after middle" width="50" height="28" fetchpriority="high"><img src="test2.jpg" alt="sibling" width="59" height="34" loading="lazy">"#
    ));
}

#[test]
fn handles_nested_image_when_outer_image_is_missing() {
    let input = "![unmeasured ![inner](test3.gif)](test11.png)![sibling](test2.jpg)";
    let output = measure_content(input);

    assert!(output.contains(
        r#"![unmeasured ![inner](test3.gif)](test11.png)<img src="test2.jpg" alt="sibling" width="59" height="34" loading="lazy">"#
    ));
}

#[test]
fn preserves_math_in_image_alt() {
    let output = measure_content(r"![inline $x < y$ and display $$a > b & c$$](test1.png)");

    assert!(output.contains(
        r#"<img src="test1.png" alt="inline $x &lt; y$ and display $$a &gt; b &amp; c$$" width="50" height="28" fetchpriority="high">"#
    ));
}

#[test]
fn leaves_image_like_text_in_code_unmodified() {
    let input = "Inline `![literal](test1.png)`\n\n\
                 ```text\n\
                 ![literal](test1.png)\n\
                 ```\n";

    assert_eq!(measure_content(input), input);
}

#[test]
fn converts_reference_style_image() {
    let input = "![logo][logo]\n\n[logo]: test1.png\n";
    let output = measure_content(input);

    assert!(output.contains(r#"<img src="test1.png" alt="logo" width="50" height="28" fetchpriority="high">"#));
    assert!(output.contains("[logo]: test1.png"));
}

#[test]
fn converts_image_inside_attributed_div_in_media_slider() {
    let input = r#"```media-slider aria-label="example2"
![avif](test5.avif)
<div class="ft-definition">
 ![エーブイアイえふ](test5.avif)
 テスト
</div>
```"#;
    let output = measure_content(input);

    assert!(output.contains(r#"<media-slider role="group" aria-roledescription="carousel" aria-label="example2">"#));
    assert!(output.contains("<div class=\"ft-definition\">"));
    assert!(output.contains(r#"<img src="test5.avif" alt="エーブイアイえふ" width="59" height="44" loading="lazy">"#));
    assert!(output.contains("テスト\n</div>\n</media-slider>"));
}
