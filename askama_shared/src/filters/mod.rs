//! Module for built-in filter functions
//!
//! Contains all the built-in filter functions for use in templates.
//! You can define your own filters; for more information,
//! see the top-level crate documentation.

#[cfg(feature = "serde-json")]
mod json;

#[cfg(feature = "serde-json")]
pub use self::json::json;

use num_traits::Signed;
use std::fmt;

use super::Result;
use escaping::{self, MarkupDisplay};

// This is used by the code generator to decide whether a named filter is part of
// Askama or should refer to a local `filters` module. It should contain all the
// filters shipped with Askama, even the optional ones (since optional inclusion
// in the const vector based on features seems impossible right now).
pub const BUILT_IN_FILTERS: [&str; 18] = [
    "abs",
    "capitalize",
    "center",
    "e",
    "escape",
    "format",
    "join",
    "linebreaks",
    "linebreaksbr",
    "lower",
    "lowercase",
    "safe",
    "trim",
    "truncate",
    "upper",
    "uppercase",
    "wordcount",
    "json", // Optional feature; reserve the name anyway
];

/// Marks a string (or other `Display` type) as safe
///
/// Use this is you want to allow markup in an expression, or if you know
/// that the expression's contents don't need to be escaped.
pub fn safe<D, I>(v: I) -> Result<MarkupDisplay<D>>
where
    D: fmt::Display,
    MarkupDisplay<D>: From<I>,
{
    let res: MarkupDisplay<D> = v.into();
    Ok(res.mark_safe())
}

/// Escapes `&`, `<` and `>` in strings
pub fn escape<D, I>(i: I) -> Result<MarkupDisplay<String>>
where
    D: fmt::Display,
    MarkupDisplay<D>: From<I>,
{
    let md: MarkupDisplay<D> = i.into();
    Ok(MarkupDisplay::Safe(escaping::escape(md.unsafe_string())))
}

/// Alias for the `escape()` filter
pub fn e<D, I>(i: I) -> Result<MarkupDisplay<String>>
where
    D: fmt::Display,
    MarkupDisplay<D>: From<I>,
{
    escape(i)
}

/// Formats arguments according to the specified format
///
/// The first argument to this filter must be a string literal (as in normal
/// Rust). All arguments are passed through to the `format!()`
/// [macro](https://doc.rust-lang.org/stable/std/macro.format.html) by
/// the Askama code generator.
pub fn format() {}

/// Replaces line breaks in plain text with appropriate HTML
///
/// A single newline becomes an HTML line break `<br>` and a new line
/// followed by a blank line becomes a paragraph break `<p>`.
pub fn linebreaks(s: &fmt::Display) -> Result<String> {
    let s = format!("{}", s);
    let linebroken = s.replace("\n\n", "</p><p>").replace("\n", "<br/>");

    Ok(format!("<p>{}</p>", linebroken))
}

/// Converts all newlines in a piece of plain text to HTML line breaks
pub fn linebreaksbr(s: &fmt::Display) -> Result<String> {
    let s = format!("{}", s);
    Ok(s.replace("\n", "<br/>"))
}

/// Converts to lowercase
pub fn lower(s: &fmt::Display) -> Result<String> {
    let s = format!("{}", s);
    Ok(s.to_lowercase())
}

/// Alias for the `lower()` filter
pub fn lowercase(s: &fmt::Display) -> Result<String> {
    lower(s)
}

/// Converts to uppercase
pub fn upper(s: &fmt::Display) -> Result<String> {
    let s = format!("{}", s);
    Ok(s.to_uppercase())
}

/// Alias for the `upper()` filter
pub fn uppercase(s: &fmt::Display) -> Result<String> {
    upper(s)
}

/// Strip leading and trailing whitespace
pub fn trim(s: &fmt::Display) -> Result<String> {
    let s = format!("{}", s);
    Ok(s.trim().to_owned())
}

/// Limit string length, appends '...' if truncated
pub fn truncate(s: &fmt::Display, len: &usize) -> Result<String> {
    let mut s = format!("{}", s);
    if s.len() < *len {
        Ok(s)
    } else {
        s.truncate(*len);
        s.push_str("...");
        Ok(s)
    }
}

/// Joins iterable into a string separated by provided argument
pub fn join<T, I, S>(input: I, separator: S) -> Result<String>
where
    T: fmt::Display,
    I: Iterator<Item = T>,
    S: AsRef<str>,
{
    let separator: &str = separator.as_ref();

    let mut rv = String::new();

    for (num, item) in input.enumerate() {
        if num > 0 {
            rv.push_str(separator);
        }

        rv.push_str(&format!("{}", item));
    }

    Ok(rv)
}

/// Absolute value
pub fn abs<T>(number: T) -> Result<T>
where
    T: Signed,
{
    Ok(number.abs())
}

/// Capitalize a value. The first character will be uppercase, all others lowercase.
pub fn capitalize(s: &fmt::Display) -> Result<String> {
    let mut s = format!("{}", s);

    match s.get_mut(0..1).map(|s| {
        s.make_ascii_uppercase();
        &*s
    }) {
        None => Ok(s),
        _ => {
            let l = s.len();
            match s.get_mut(1..l).map(|s| {
                s.make_ascii_lowercase();
                &*s
            }) {
                _ => Ok(s),
            }
        }
    }
}

/// Centers the value in a field of a given width
pub fn center(s: &fmt::Display, l: usize) -> Result<String> {
    let s = format!("{}", s);
    let len = s.len();

    if l <= len {
        Ok(s)
    } else {
        let p = l - len;
        let q = p / 2;
        let r = p % 2;
        let mut buf = String::with_capacity(l);

        for _ in 0..q {
            buf.push(' ');
        }

        buf.push_str(&s);

        for _ in 0..q + r {
            buf.push(' ');
        }

        Ok(buf)
    }
}

/// Count the words in that string
pub fn wordcount(s: &fmt::Display) -> Result<usize> {
    let s = format!("{}", s);

    Ok(s.split_whitespace().count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linebreaks() {
        assert_eq!(
            linebreaks(&"Foo\nBar Baz").unwrap(),
            "<p>Foo<br/>Bar Baz</p>"
        );
        assert_eq!(
            linebreaks(&"Foo\nBar\n\nBaz").unwrap(),
            "<p>Foo<br/>Bar</p><p>Baz</p>"
        );
    }

    #[test]
    fn test_linebreaksbr() {
        assert_eq!(linebreaksbr(&"Foo\nBar").unwrap(), "Foo<br/>Bar");
        assert_eq!(
            linebreaksbr(&"Foo\nBar\n\nBaz").unwrap(),
            "Foo<br/>Bar<br/><br/>Baz"
        );
    }

    #[test]
    fn test_lower() {
        assert_eq!(lower(&"Foo").unwrap(), "foo");
        assert_eq!(lower(&"FOO").unwrap(), "foo");
        assert_eq!(lower(&"FooBar").unwrap(), "foobar");
        assert_eq!(lower(&"foo").unwrap(), "foo");
    }

    #[test]
    fn test_upper() {
        assert_eq!(upper(&"Foo").unwrap(), "FOO");
        assert_eq!(upper(&"FOO").unwrap(), "FOO");
        assert_eq!(upper(&"FooBar").unwrap(), "FOOBAR");
        assert_eq!(upper(&"foo").unwrap(), "FOO");
    }

    #[test]
    fn test_trim() {
        assert_eq!(trim(&" Hello\tworld\t").unwrap(), "Hello\tworld");
    }

    #[test]
    fn test_join() {
        assert_eq!(
            join((&["hello", "world"]).into_iter(), ", ").unwrap(),
            "hello, world"
        );
        assert_eq!(join((&["hello"]).into_iter(), ", ").unwrap(), "hello");

        let empty: &[&str] = &[];
        assert_eq!(join(empty.into_iter(), ", ").unwrap(), "");

        let input: Vec<String> = vec!["foo".into(), "bar".into(), "bazz".into()];
        assert_eq!(
            join((&input).into_iter(), ":".to_string()).unwrap(),
            "foo:bar:bazz"
        );
        assert_eq!(
            join(input.clone().into_iter(), ":").unwrap(),
            "foo:bar:bazz"
        );
        assert_eq!(
            join(input.clone().into_iter(), ":".to_string()).unwrap(),
            "foo:bar:bazz"
        );

        let input: &[String] = &["foo".into(), "bar".into()];
        assert_eq!(join(input.into_iter(), ":").unwrap(), "foo:bar");
        assert_eq!(join(input.into_iter(), ":".to_string()).unwrap(), "foo:bar");

        let real: String = "blah".into();
        let input: Vec<&str> = vec![&real];
        assert_eq!(join(input.into_iter(), ";").unwrap(), "blah");

        assert_eq!(
            join((&&&&&["foo", "bar"]).into_iter(), ", ").unwrap(),
            "foo, bar"
        );
    }

    #[test]
    fn test_abs() {
        assert_eq!(abs(1).unwrap(), 1);
        assert_eq!(abs(-1).unwrap(), 1);
        assert_eq!(abs(1.0).unwrap(), 1.0);
        assert_eq!(abs(-1.0).unwrap(), 1.0);
        assert_eq!(abs(1.0 as f64).unwrap(), 1.0 as f64);
        assert_eq!(abs(-1.0 as f64).unwrap(), 1.0 as f64);
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize(&"foo").unwrap(), "Foo".to_string());
        assert_eq!(capitalize(&"f").unwrap(), "F".to_string());
        assert_eq!(capitalize(&"fO").unwrap(), "Fo".to_string());
        assert_eq!(capitalize(&"").unwrap(), "".to_string());
        assert_eq!(capitalize(&"FoO").unwrap(), "Foo".to_string());
        assert_eq!(capitalize(&"foO BAR").unwrap(), "Foo bar".to_string());
    }

    #[test]
    fn test_center() {
        assert_eq!(center(&"f", 3).unwrap(), " f ".to_string());
        assert_eq!(center(&"f", 4).unwrap(), " f  ".to_string());
        assert_eq!(center(&"foo", 1).unwrap(), "foo".to_string());
        assert_eq!(center(&"foo bar", 8).unwrap(), "foo bar ".to_string());
    }

    #[test]
    fn test_wordcount() {
        assert_eq!(wordcount(&"").unwrap(), 0);
        assert_eq!(wordcount(&" \n\t").unwrap(), 0);
        assert_eq!(wordcount(&"foo").unwrap(), 1);
        assert_eq!(wordcount(&"foo bar").unwrap(), 2);
    }
}
