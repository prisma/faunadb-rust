use crate::{expr::Expr, query::Query};

query![
    CaseFold,
    Concat,
    FindStr,
    FindStrRegex,
    LTrim,
    Length,
    LowerCase,
    RTrim,
    Repeat,
    ReplaceStr,
    ReplaceStrRegex,
    Space,
    SubString,
    TitleCase,
    Trim,
    UpperCase
];

/// The CaseFold function returns a normalized string.
///
/// When strings are transformed into their normalized forms,
/// canonical-equivalent strings have precisely the same binary representation.
/// Then, a binary comparison function such as equals can compare two strings
/// for case-insensitive matching.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/casefold)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CaseFold<'a> {
    casefold: Expr<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    normalizer: Option<Normalizer>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Normalizer {
    NFKCCaseFold,
    NFC,
    NFD,
    NFKC,
    NFKD,
}

impl<'a> CaseFold<'a> {
    pub fn new(string: impl Into<Expr<'a>>) -> Self {
        Self {
            casefold: string.into(),
            normalizer: None,
        }
    }

    pub fn normalizer(&mut self, normalizer: Normalizer) -> &mut Self {
        self.normalizer = Some(normalizer);
        self
    }
}

/// The `Concat` function returns a string which has joined a list of strings into
/// a single string.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/concat)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Concat<'a> {
    concat: Expr<'a>,
    separator: Expr<'a>,
}

impl<'a> Concat<'a> {
    pub fn new(concat: impl Into<Expr<'a>>, separator: impl Into<Expr<'a>>) -> Self {
        Self {
            concat: concat.into(),
            separator: separator.into(),
        }
    }
}

/// The `FindStr` function returns the offset position of a string in another
/// string, or `-1` if the string is not found.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/findstr)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FindStr<'a> {
    findstr: Expr<'a>,
    find: Expr<'a>,
    start: Option<Expr<'a>>,
}

impl<'a> FindStr<'a> {
    pub fn new(findstr: impl Into<Expr<'a>>, find: impl Into<Expr<'a>>) -> Self {
        Self {
            findstr: findstr.into(),
            find: find.into(),
            start: None,
        }
    }

    pub fn start(&mut self, start: impl Into<Expr<'a>>) -> &mut Self {
        self.start = Some(start.into());
        self
    }
}

/// The `FindStrRegex` function returns an array of up to 1024 objects
/// describing where the pattern is found in the search string.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/findstrregex)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FindStrRegex<'a> {
    findstrregex: Expr<'a>,
    pattern: Expr<'a>,
    start: Option<Expr<'a>>,
    num_results: Option<Expr<'a>>,
}

impl<'a> FindStrRegex<'a> {
    pub fn new(findstrregex: impl Into<Expr<'a>>, pattern: impl Into<Expr<'a>>) -> Self {
        Self {
            findstrregex: findstrregex.into(),
            pattern: pattern.into(),
            start: None,
            num_results: None,
        }
    }

    pub fn start(&mut self, start: impl Into<Expr<'a>>) -> &mut Self {
        self.start = Some(start.into());
        self
    }

    pub fn num_results(&mut self, num_results: impl Into<Expr<'a>>) -> &mut Self {
        self.num_results = Some(num_results.into());
        self
    }
}

/// The `LTrim` function removes all white spaces, tabs, and new lines from the
/// beginning of a string.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/ltrim)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LTrim<'a> {
    ltrim: Expr<'a>,
}

impl<'a> LTrim<'a> {
    pub fn new(string: impl Into<Expr<'a>>) -> Self {
        Self {
            ltrim: string.into(),
        }
    }
}

/// The `RTrim` function removes all trailing white spaces, tabs, and new lines
/// from the end of a string.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/rtrim)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RTrim<'a> {
    rtrim: Expr<'a>,
}

impl<'a> RTrim<'a> {
    pub fn new(string: impl Into<Expr<'a>>) -> Self {
        Self {
            rtrim: string.into(),
        }
    }
}

/// The `Length` function returns the number of code points in the string.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/length)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Length<'a> {
    length: Expr<'a>,
}

impl<'a> Length<'a> {
    pub fn new(string: impl Into<Expr<'a>>) -> Self {
        Self {
            length: string.into(),
        }
    }
}

/// The `LowerCase` function returns a string in which all uppercase characters
/// have been replaced by their corresponding lowercase characters.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/lowercase)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LowerCase<'a> {
    lowercase: Expr<'a>,
}

impl<'a> LowerCase<'a> {
    pub fn new(string: impl Into<Expr<'a>>) -> Self {
        Self {
            lowercase: string.into(),
        }
    }
}

/// The `Repeat` function returns a string consisting of the value string
/// repeated number times.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/repeat)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Repeat<'a> {
    repeat: Expr<'a>,
    number: Expr<'a>,
}

impl<'a> Repeat<'a> {
    pub fn new(string: impl Into<Expr<'a>>, number: impl Into<Expr<'a>>) -> Self {
        Self {
            repeat: string.into(),
            number: number.into(),
        }
    }
}

/// The `ReplaceStr` function returns a string which has all occurrences of the
/// `find` string replaced with the `replace` string. Punctuation in the `find` string
/// is interpreted literally and not as a pattern.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/replacestr)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReplaceStr<'a> {
    replacestr: Expr<'a>,
    find: Expr<'a>,
    replace: Expr<'a>,
}

impl<'a> ReplaceStr<'a> {
    pub fn new<T, V, W>(string: T, find: V, replace: W) -> Self
    where
        T: Into<Expr<'a>>,
        V: Into<Expr<'a>>,
        W: Into<Expr<'a>>,
    {
        Self {
            replacestr: string.into(),
            find: find.into(),
            replace: replace.into(),
        }
    }
}

/// The `ReplaceStrRegex` function returns a string which has either the first or
/// all occurrences of the pattern replaced with the replace string. The pattern
/// conforms to Java regular expression syntax.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/replacestrregex)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReplaceStrRegex<'a> {
    replacestrregex: Expr<'a>,
    pattern: Expr<'a>,
    replace: Expr<'a>,
    first: bool,
}

impl<'a> ReplaceStrRegex<'a> {
    pub fn new<T, V, W>(string: T, pattern: V, replace: W, first: bool) -> Self
    where
        T: Into<Expr<'a>>,
        V: Into<Expr<'a>>,
        W: Into<Expr<'a>>,
    {
        Self {
            replacestrregex: string.into(),
            pattern: pattern.into(),
            replace: replace.into(),
            first,
        }
    }
}

/// The `Space` function returns a string of the specified number of spaces.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/space)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Space<'a> {
    space: Expr<'a>,
}

impl<'a> Space<'a> {
    pub fn new(count: impl Into<Expr<'a>>) -> Self {
        Self {
            space: count.into(),
        }
    }
}

/// The `SubString` function returns a portion of the `value` string beginning
/// at the character `start` position for `length` characters long.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/substring)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubString<'a> {
    substring: Expr<'a>,
    start: Expr<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<Expr<'a>>,
}

impl<'a> SubString<'a> {
    pub fn new(string: impl Into<Expr<'a>>, start: impl Into<Expr<'a>>) -> Self {
        Self {
            substring: string.into(),
            start: start.into(),
            length: None,
        }
    }

    pub fn length(&mut self, length: impl Into<Expr<'a>>) -> &mut Self {
        self.length = Some(length.into());
        self
    }
}

/// The `TitleCase` function returns a string which has the first letter of each
/// word capitalized.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/titlecase)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TitleCase<'a> {
    titlecase: Expr<'a>,
}

impl<'a> TitleCase<'a> {
    pub fn new(string: impl Into<Expr<'a>>) -> Self {
        Self {
            titlecase: string.into(),
        }
    }
}

/// The `Trim` function returns a string which has both the leading and trailing
/// white spaces, tabs, and new lines removed from the string.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/trim)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trim<'a> {
    trim: Expr<'a>,
}

impl<'a> Trim<'a> {
    pub fn new(string: impl Into<Expr<'a>>) -> Self {
        Self {
            trim: string.into(),
        }
    }
}

/// The `UpperCase` function returns a string which has all lowercase characters
/// in the string replaced by their corresponding uppercase characters.
///
/// Read the
/// [docs](https://docs.fauna.com/fauna/current/reference/queryapi/string/uppercase)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpperCase<'a> {
    uppercase: Expr<'a>,
}

impl<'a> UpperCase<'a> {
    pub fn new(string: impl Into<Expr<'a>>) -> Self {
        Self {
            uppercase: string.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde_json::{self, json};

    #[test]
    fn test_case_fold() {
        let mut fun = CaseFold::new("Hen Wen");
        fun.normalizer(Normalizer::NFD);

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "casefold": "Hen Wen",
            "normalizer": "NFD",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_concat() {
        let fun = Concat::new(Array::from(vec!["Hen", "Wen"]), ",");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "concat": ["Hen", "Wen"],
            "separator": ","
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_find_str() {
        let mut fun = FindStr::new("fire and fireman", "fire");
        fun.start(0);

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "findstr": "fire and fireman",
            "find": "fire",
            "start": 0,
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_find_str_regex() {
        let mut fun = FindStrRegex::new("fire and fireman", "[a-z][A-Z]");
        fun.start(0);
        fun.num_results(4);

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "findstrregex": "fire and fireman",
            "pattern": "[a-z][A-Z]",
            "start": 0,
            "num_results": 4,
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_ltrim() {
        let fun = LTrim::new("     haha");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "ltrim": "     haha",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_length() {
        let fun = Length::new("I'm long");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "length": "I'm long",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_lowercase() {
        let fun = LowerCase::new("I SCREAM AND I YELL AND YOU CAN'T STOP ME");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "lowercase": "I SCREAM AND I YELL AND YOU CAN'T STOP ME",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_rtrim() {
        let fun = RTrim::new("haha      ");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "rtrim": "haha      ",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_repeat() {
        let fun = Repeat::new("All work and no play makes Jack a dull boy.", 100000);

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "repeat": "All work and no play makes Jack a dull boy.",
            "number": 100000,
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_replace_str() {
        let fun = ReplaceStr::new("fire and fireman", "fire", "meow");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "replacestr": "fire and fireman",
            "find": "fire",
            "replace": "meow",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_replace_str_regex() {
        let fun = ReplaceStrRegex::new("fire and fireman", "[a-z][A-Z]", "meow", false);

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "replacestrregex": "fire and fireman",
            "pattern": "[a-z][A-Z]",
            "replace": "meow",
            "first": false,
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_space() {
        let fun = Space::new(4);

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "space": 4,
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_substring() {
        let mut fun = SubString::new("meowmeowcat", 4);
        fun.length(2);

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "substring": "meowmeowcat",
            "start": 4,
            "length": 2,
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_title_case() {
        let fun = TitleCase::new("this is a lousy title");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "titlecase": "this is a lousy title",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_trim() {
        let fun = Trim::new("  cut cut trim it out    ");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "trim": "  cut cut trim it out    ",
        });

        assert_eq!(expected, serialized);
    }

    #[test]
    fn test_uppercase() {
        let fun = UpperCase::new("i have a tiny voice, can you help me?");

        let query = Query::from(fun);
        let serialized = serde_json::to_value(&query).unwrap();

        let expected = json!({
            "uppercase": "i have a tiny voice, can you help me?",
        });

        assert_eq!(expected, serialized);
    }
}
