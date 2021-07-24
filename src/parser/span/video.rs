use parser::Span;
use parser::Span::Video;
use regex::Regex;

pub fn parse_video(text: &str) -> Option<(Span, usize)> {
    lazy_static! {
        static ref VIDEO: Regex =
            Regex::new("^#\\[(?P<text>.*?)\\]\\((?P<url>.*?)\\)")
                .unwrap();
    }

    if VIDEO.is_match(text) {
        let caps = VIDEO.captures(text).unwrap();
        let text = if let Some(mat) = caps.name("text") {
            mat.as_str().to_owned()
        } else {
            "".to_owned()
        };
        let url = if let Some(mat) = caps.name("url") {
            mat.as_str().to_owned()
        } else {
            "".to_owned()
        };
        // TODO correctly get whitespace length between url and title
        let len = text.len() + url.len() + 5;
        return Some((Video(text, url), len));
    }
    None
}

#[test]
fn finds_video() {
    assert_eq!(
        parse_video("#[](example.com) test"),
        Some((Video("".to_owned(), "example.com".to_owned()), 16))
    );

    assert_eq!(
        parse_video("#[an example]() test"),
        Some((Video("an example".to_owned(), "".to_owned()), 15))
    );

    assert_eq!(
        parse_video("#[]() test"),
        Some((Video("".to_owned(), "".to_owned()), 5))
    );

    assert_eq!(
        parse_video("#[an example](example.com) test [a link](example.com)"),
        Some((
            Video("an example".to_owned(), "example.com".to_owned()),
            26
        ))
    );
}

#[test]
fn no_false_positives() {
    assert_eq!(parse_video("#[()] testing things test"), None);
    assert_eq!(parse_video("#()[] testing things test"), None);
}

#[test]
fn no_early_matching() {
    assert_eq!(parse_video("were #[an example](example.com) test"), None);
}
