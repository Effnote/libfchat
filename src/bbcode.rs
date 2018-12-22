#[derive(Clone, PartialEq, Debug)]
pub enum Color {
    Black,
    Blue,
    Brown,
    Cyan,
    Gray,
    Green,
    Orange,
    Pink,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Clone, PartialEq, Debug)]
pub enum BBCode {
    Bold(Vec<BBCode>),
    Italics(Vec<BBCode>),
    Underline(Vec<BBCode>),
    Strikethrough(Vec<BBCode>),
    Subscript(Vec<BBCode>),
    Superscript(Vec<BBCode>),
    Color(Color, Vec<BBCode>),
    Url { url: String, text: String },
    Icon { name: String },
    Eicon { name: String },
    User { name: String },
    Noparse(String),
    Channel { channel: String },
    Session { code: String, name: String },
    Text(String),
}

pub fn parse(message: &str) -> Vec<BBCode> {
    use self::BBCode::*;
    vec![Text(String::from(message))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tags() {
        let tags: &[(&str, fn(Vec<BBCode>) -> BBCode)] = &[
            ("b", BBCode::Bold),
            ("i", BBCode::Italics),
            ("u", BBCode::Underline),
            ("s", BBCode::Strikethrough),
            ("sub", BBCode::Subscript),
            ("sup", BBCode::Superscript),
        ];
        for &(tag, bbcode) in tags {
            assert_eq!(
                parse(&format!("[{0}]text[/{0}]", tag)),
                vec![bbcode(vec![BBCode::Text(String::from("text"))])],
            );
        }
    }

    #[test]
    fn unknown_tag() {
        assert_eq!(
            parse("[test]foo[/test][b]bar[/b]"),
            vec![
                BBCode::Text(String::from("[test]foo[/test]")),
                BBCode::Bold(vec![BBCode::Text(String::from("bar"))]),
            ],
        );
    }

    #[test]
    fn autoclose_missing() {
        assert_eq!(
            parse("[b]text"),
            vec![BBCode::Bold(vec![BBCode::Text(String::from("text"))])]
        );
        assert_eq!(
            parse("[b][i]text"),
            vec![BBCode::Bold(vec![BBCode::Italics(vec![BBCode::Text(
                String::from("text")
            )]),]),]
        );
    }

    #[test]
    fn subscript() {
        assert_eq!(
            parse("[sub][b]text[/b][/sub]"),
            vec![BBCode::Subscript(vec![BBCode::Bold(vec![BBCode::Text(
                String::from("text")
            )])]),]
        );
        assert_eq!(
            parse("[sub][sub]text[/sub][/sub]"),
            vec![BBCode::Subscript(vec![BBCode::Text(String::from(
                "[sub]text[/sub]"
            ))]),]
        );
        assert_eq!(
            parse("[sub][sup]text[/sup][/sub]"),
            vec![BBCode::Subscript(vec![BBCode::Text(String::from(
                "[sup]text[/sup]"
            ))]),]
        );
    }

    #[test]
    fn superscript() {
        assert_eq!(
            parse("[sup][b]text[/b][/sup]"),
            vec![BBCode::Superscript(vec![BBCode::Bold(vec![BBCode::Text(
                String::from("text")
            )])]),]
        );
        assert_eq!(
            parse("[sup][sup]text[/sup][/sup]"),
            vec![BBCode::Superscript(vec![BBCode::Text(String::from(
                "[sub]text[/sub]"
            ))]),]
        );
        assert_eq!(
            parse("[sup][sub]text[/sub][/sup]"),
            vec![BBCode::Superscript(vec![BBCode::Text(String::from(
                "[sub]text[/sub]"
            ))]),]
        );
    }

    #[test]
    fn noparse() {
        assert_eq!(
            parse("[noparse][b]test[/b][/noparse]"),
            vec![BBCode::Noparse(String::from("[b]test[/b]"))]
        );
    }

    #[test]
    fn color() {
        assert_eq!(
            parse("[color=red]red[/color]"),
            vec![BBCode::Color(
                Color::Red,
                vec![BBCode::Text(String::from("red"))]
            ),]
        );
        assert_eq!(
            parse("[color=red][b]red[/b][/color]"),
            vec![BBCode::Color(
                Color::Red,
                vec![BBCode::Bold(vec![BBCode::Text(String::from("red"))])],
            ),]
        );
        assert_eq!(
            parse("[color]bad[/color]"),
            vec![BBCode::Text(String::from("bad"))]
        );
        assert_eq!(
            parse("[color]bad[/color]"),
            vec![BBCode::Text(String::from("bad"))]
        );
    }

    #[test]
    fn url() {
        assert_eq!(
            parse("[url=https://www.google.com/]Google[/url]"),
            vec![BBCode::Url {
                url: String::from("https://www.google.com/"),
                text: String::from("Google"),
            },]
        );
        assert_eq!(
            parse("[url]https://www.google.com/[/url]"),
            vec![BBCode::Url {
                url: String::from("https://www.google.com/"),
                text: String::from("https://www.google.com/"),
            },]
        );
        assert_eq!(
            parse("[url=www.google.com/]Google[/url]"),
            vec![BBCode::Url {
                url: String::from("https://www.google.com/"),
                text: String::from("Google"),
            },]
        );
    }
}
