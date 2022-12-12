use std::{fmt::Display, iter::Peekable, ops::Range, str::Chars};

use thiserror::Error;

/* Parsing of entries like: `2021-11-31 21:00 23:00 tag1 prop1=val1. Comment text`. Grammar:
      INPUT -> DATES TAGS COMMENT?
      DATES -> DATETIME ('-' DATETIME | TIME)
      DATETIME -> DATE TIME
      DATE -> \d\d\d\d'-'\d\d'-'\d\d
      TIME -> \d\d':'\d\d
      TAGS -> TAG ('.' TAGS)*
      TAG -> TAGNAME (PROP)*
      PROP_OP -> '='
      PROP -> PROPNAME (PROP_OP? PROPVALUE)?
      COMMENT -> \W \w*
      TAGNAME -> \w+
      PROPNAME -> \w+
      PROPVALUE -> \w+
*/

/// Errors occurred during input tokenizing
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TokenizingError {
    #[error("Error parsing {0}: expected '{1}'")]
    Expected(Token, Char, usize),

    // TODO Better error message
    #[error("Error parsing DateTime: expected either '-[SPACE]' followed by DateTime in format: 'YYYY-DD-MM HH:MM' or Time in format 'HH:MM'")]
    DateOrTimeExpected(usize),

    #[error("Tags were not found")]
    TagsNotFound(usize),
}

/// Associated token to each character of the input entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Token {
    Comment,           // Starts with capital letter, then anything
    Date,              // YYYY-MM-DD
    DateSeparator,     // -
    DateTimeSeparator, // -
    PropertyName,      // Small letters
    PropertyOperator,  // =
    PropertyValue,     // Small letters
    Space,             // Space, tabs, etc.
    TagName,           // Small letters or digits
    TagSeparator,      // .
    Time,              // HH:MM
    TimeSeparator,     // :
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

/// Char type that Tokenizer is using during reading
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Char {
    Any,
    AnyNonSeparator,
    Colon,
    Dash,
    Digit,
    Dot,
    Eq,
    LowercaseOrDigit,
    Quote,
    Space,
    Uppercase,
}

impl Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Char::Any => "any letter",
            Char::AnyNonSeparator => "anything non space or dot",
            Char::Colon => "colon",
            Char::Dash => "dash",
            Char::Digit => "digit",
            Char::Dot => "dot",
            Char::Eq => "equal",
            Char::LowercaseOrDigit => "lowercase letter or digit",
            Char::Quote => "quote",
            Char::Space => "space",
            Char::Uppercase => "uppercase letter",
        })
    }
}

impl Char {
    fn matches(&self, c: &char) -> bool {
        match self {
            Char::Any => true,
            Char::AnyNonSeparator => !c.is_whitespace() && *c != '.',
            Char::Colon => *c == ':',
            Char::Dash => *c == '-',
            Char::Digit => c.is_ascii_digit(),
            Char::Dot => *c == '.',
            Char::Eq => *c == '=',
            Char::LowercaseOrDigit => c.is_lowercase() || c.is_ascii_digit(),
            Char::Quote => *c == '"',
            Char::Space => c.is_ascii_whitespace(),
            Char::Uppercase => c.is_uppercase(),
        }
    }
}

/// Internal result structure. Not all of the values are errors but almost all tokenizer functions
/// returns Result<(), TokenizingResult> for more convenient early returns via `?` operator
enum TokenizingResult {
    DateOrTimeExpected(usize),
    EndOfLine(Option<(Token, Char, usize)>),
    Expected(Token, Char, usize),
    TagsNotFound(usize),
}

/// Purpose of the tokenizer is to read the input entry and associate a `Token` to each input character
/// It's used as an input to `Parser` and also for providing feedback when user is typing
/// Tokens are essentially array of bytes which can be transferred from WebAssembly in the most efficient way
pub struct Tokenizer<'a> {
    /// Input tokens, one for each input character. May be smaller in case of an error
    pub tokens: Vec<Token>,
    /// Set of expected tokens that tokenizer expects next, could be used for autocomplete logic. Multiple Tokens may be expected
    pub expected_next: Vec<Token>,
    /// Error if tokenizer failed to parse the input
    pub error: Option<TokenizingError>,
    input: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str, _: bool) -> Self {
        let mut tokenizer = Self {
            tokens: Vec::with_capacity(128), // Educated guess of usual entry length
            expected_next: vec![],
            error: None,
            input: input.chars().peekable(),
        };
        match tokenizer.tokenize_input() {
            Ok(()) => {}
            Err(TokenizingResult::EndOfLine(next)) => {
                if let Some((token, _, _)) = next {
                    tokenizer.expected_next = vec![token];
                }
                tokenizer.error =
                    next.map(|(token, char, idx)| TokenizingError::Expected(token, char, idx));
            }
            Err(TokenizingResult::Expected(token, char, idx)) => {
                tokenizer.expected_next = vec![token];
                tokenizer.error = Some(TokenizingError::Expected(token, char, idx))
            }
            Err(TokenizingResult::DateOrTimeExpected(idx)) => {
                tokenizer.expected_next = vec![Token::DateTimeSeparator, Token::Time];
                tokenizer.error = Some(TokenizingError::DateOrTimeExpected(idx))
            }
            Err(TokenizingResult::TagsNotFound(idx)) => {
                tokenizer.expected_next = vec![Token::TagName];
                tokenizer.error = Some(TokenizingError::TagsNotFound(idx));
            }
        }
        tokenizer
    }

    fn tokenize_input(&mut self) -> Result<(), TokenizingResult> {
        self.tokenize_datetimes()?;
        self.tokenize_tags()?;
        self.tokenize_comments()
    }

    fn tokenize_datetimes(&mut self) -> Result<(), TokenizingResult> {
        self.tokenize_datetime()?;

        // We are here now ↓ After first read datetime either we have another datetime (prefixed with datetime separator) or just time in short notation
        //  2020-01-11 23:23 - 2022-01:12 00:12 activity
        //  2022-01-11 15:00 18:00 activity
        // If first case we would read [SPACE][DASH][SPACE][DATE][SPACE][TIME], in second [SPACE][TIME]
        self.read_one(Token::Space, Char::Space)?;
        match self.read(Token::DateSeparator, Char::Dash, 1..1, false) {
            Ok(_) => {
                self.read_one(Token::Space, Char::Space)?;
                self.tokenize_datetime()
            }
            Err(TokenizingResult::EndOfLine(_)) => {
                Err(TokenizingResult::DateOrTimeExpected(self.tokens.len()))
            }
            Err(TokenizingResult::Expected(..)) => {
                // TODO If [TIME] parsing failed we may improve error logging by suggesting
                //      [DASH][SPACE][DATE][SPACE][TIME] as an alternative
                self.tokenize_time()
            }
            Err(err) => Err(err),
        }
    }

    fn tokenize_datetime(&mut self) -> Result<(), TokenizingResult> {
        self.tokenize_date()?;
        self.read_one(Token::Space, Char::Space)?;
        self.tokenize_time()
    }

    /// Tokenize date in format: [DIGIT]{4}[DASH]{1}[DIGIT]{2}[DASH]{1}[DIGIT]{2}
    fn tokenize_date(&mut self) -> Result<(), TokenizingResult> {
        self.read(Token::Date, Char::Digit, 4..4, false)?;
        self.read_one(Token::DateSeparator, Char::Dash)?;
        self.read(Token::Date, Char::Digit, 2..2, false)?;
        self.read_one(Token::DateSeparator, Char::Dash)?;
        self.read(Token::Date, Char::Digit, 2..2, false)?;
        Ok(())
    }

    /// Tokenize time in format: [DIGIT]{2}[COLON]{1}[DIGIT]{2}
    fn tokenize_time(&mut self) -> Result<(), TokenizingResult> {
        self.read(Token::Time, Char::Digit, 2..2, false)?;
        self.read_one(Token::TimeSeparator, Char::Colon)?;
        self.read(Token::Time, Char::Digit, 2..2, false)?;
        Ok(())
    }

    fn tokenize_tags(&mut self) -> Result<(), TokenizingResult> {
        let mut tags_exists = false;
        self.expected_next = vec![Token::TagName];
        while self.tokenize_tag()?.is_some() {
            tags_exists = true;
            if self.read(Token::TagSeparator, Char::Dot, 0..1, true)? > 0 {
                self.expected_next = vec![Token::TagName, Token::Comment];
            }
        }
        if !tags_exists {
            return Err(TokenizingResult::TagsNotFound(self.tokens.len()));
        }
        Ok(())
    }

    fn tokenize_tag(&mut self) -> Result<Option<()>, TokenizingResult> {
        match self.read(Token::TagName, Char::LowercaseOrDigit, 1..usize::MAX, true) {
            Ok(read) => {
                if read > 0 {
                    self.expected_next = vec![Token::TagName];
                }
                self.tokenize_properties()?;
                Ok(Some(())) // Successfully read a tag
            }
            Err(TokenizingResult::EndOfLine(_)) | Err(TokenizingResult::Expected(..)) => {
                Ok(None) // EOL or unexpected letters, probably a comment - return
            }
            Err(err) => {
                Err(err) // Forward other
            }
        }
    }

    /// Optional comment that starts with the capital letter and follows until the end
    fn tokenize_comments(&mut self) -> Result<(), TokenizingResult> {
        if self
            .read(Token::Comment, Char::Uppercase, 1..1, true)
            .is_ok()
        {
            self.expected_next = vec![Token::Comment];
            self.read(Token::Comment, Char::Any, 0..usize::MAX, true)?;
        }
        Ok(())
    }

    fn tokenize_properties(&mut self) -> Result<(), TokenizingResult> {
        //               We are here now ↓ Start reading properties
        // 2022-01-11 15:00 18:00 activity foo=bar bar=foo. activity2. Comment about it
        while self.tokenize_property()?.is_some() {
            // Keep reading more properties
        }
        Ok(())
    }

    fn tokenize_property(&mut self) -> Result<Option<()>, TokenizingResult> {
        if self.read(Token::Space, Char::Space, 0..usize::MAX, true)? > 0 {
            self.expected_next = vec![Token::PropertyName];
        }
        match self.read(
            Token::PropertyName,
            Char::LowercaseOrDigit,
            1..usize::MAX,
            true,
        ) {
            Ok(read) => {
                if read > 0 {
                    self.expected_next = vec![Token::PropertyName];
                }
            }
            Err(TokenizingResult::EndOfLine(_)) | Err(TokenizingResult::Expected(..)) => {
                return Ok(None); // EOL or unexpected letters, probably a comment - return
            }
            Err(err) => return Err(err), // Forward anything else
        };
        //                   We are here now ↓ Read optional property operator, followed by property value
        // 2022-01-11 15:00 18:00 activity foo bar=foo. activity2. Comment about it
        let prop_op = self.read(Token::PropertyOperator, Char::Eq, 0..1, true)?;
        let prop_value = self.tokenize_property_value()?;
        if prop_op > 0 && prop_value.is_none() {
            // Property value is expected when operator was used
            return Err(TokenizingResult::Expected(
                Token::PropertyValue,
                Char::AnyNonSeparator,
                self.tokens.len(),
            ));
        }
        Ok(Some(()))
    }

    fn tokenize_property_value(&mut self) -> Result<Option<()>, TokenizingResult> {
        // Property values may be surrounded with quotes
        if self
            .read(Token::PropertyValue, Char::Quote, 1..1, true)
            .is_ok()
        {
            self.expected_next = vec![Token::PropertyValue];
            // Quoted property value, keep reading until another quote
            // TODO Support escaping of the quote
            self.read_until(Token::PropertyValue, Char::Quote)?;
            return Ok(Some(()));
        }

        let mut value_read = None;
        // Prop values could be a float written with dot as a separator. Dot is used as a tag separator,
        // so such floats wouldn't be parsed correctly. To make UX better treat such cases in a special
        // way. It's safe to assume that no tags would start with a digit
        // TODO Kinda ugly, can we make it more pretty?
        if self
            .read(Token::PropertyValue, Char::Digit, 1..usize::MAX, true)
            .is_ok()
        {
            self.expected_next = vec![Token::PropertyValue];
            if self.read_one(Token::PropertyValue, Char::Dot).is_ok() {
                if self
                    .read(Token::PropertyValue, Char::Digit, 1..usize::MAX, false)
                    .is_ok()
                {
                    return Ok(Some(()));
                } else {
                    // Failed to read digits after the dot, so dot was a tag separator instead, recover
                    if let Some(v) = self.tokens.last_mut() {
                        *v = Token::TagSeparator;
                    }
                    return Ok(Some(()));
                }
            } else {
                // We've failed to read the dot, so property is contains of digits, keep reading for other letters
                // Set the flag that value exists
                value_read = Some(())
            }
        } else {
            // Failed to read a float, continue with normal property reading
        }

        // Normal property value, read as is
        match self.read(
            Token::PropertyValue,
            Char::AnyNonSeparator,
            1..usize::MAX,
            true,
        ) {
            Ok(read) => {
                if read > 0 {
                    self.expected_next = vec![Token::PropertyValue];
                }
                Ok(Some(()))
            }
            Err(TokenizingResult::EndOfLine(..)) | Err(TokenizingResult::Expected(..)) => {
                // TODO Should we actually support property names without values? What is the use case? It may complicate searching without any value
                Ok(value_read)
            }
            Err(err) => Err(err),
        }
    }

    fn read(
        &mut self,
        token: Token,
        char: Char,
        range: Range<usize>,
        space_prefix_allowed: bool,
    ) -> Result<usize, TokenizingResult> {
        let mut chars_read = 0;
        let mut space_prefix = space_prefix_allowed;
        loop {
            let next = match self.input.peek() {
                Some(c) => c,
                None => {
                    // We've reached the end of line, check if we've expected anything
                    if chars_read >= range.start {
                        return Ok(chars_read); // It's an optional token, return
                    } else {
                        return Err(TokenizingResult::EndOfLine(Some((
                            token,
                            char,
                            self.tokens.len(),
                        ))));
                    }
                }
            };
            if char.matches(next) {
                self.input.next();
                self.tokens.push(token);
                chars_read += 1;
                if chars_read == range.end {
                    return Ok(chars_read); // Token successfully read - return
                } else {
                    space_prefix = false; // Found a match, not a prefix anymore
                    continue; // Keep reading the token
                }
            } else if space_prefix && Char::Space.matches(next) {
                self.input.next();
                self.tokens.push(Token::Space);
                continue; // Spaces are ignored, continue
            } else {
                if chars_read >= range.start {
                    return Ok(chars_read); // Match failed but it's OK as it's was optional
                }
                return Err(TokenizingResult::Expected(token, char, self.tokens.len()));
            }
        }
    }

    fn read_until(&mut self, token: Token, char: Char) -> Result<(), TokenizingResult> {
        for c in self.input.by_ref() {
            self.tokens.push(token);
            if char.matches(&c) {
                return Ok(());
            }
        }
        Err(TokenizingResult::EndOfLine(Some((
            token,
            char,
            self.tokens.len(),
        ))))
    }

    fn read_one(&mut self, token: Token, char: Char) -> Result<usize, TokenizingResult> {
        self.read(token, char, 1..1, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizing_datetime() {
        // Aliases for easier testing
        let d = Token::Date;
        let ds = Token::DateSeparator;
        let s = Token::Space;
        let t = Token::Time;
        let ts = Token::TimeSeparator;

        let cases = vec![
            (
                "",
                vec![],
                vec![Token::Date],
                Some(TokenizingError::Expected(Token::Date, Char::Digit, 0)),
            ),
            (
                " ",
                vec![],
                vec![Token::Date],
                Some(TokenizingError::Expected(Token::Date, Char::Digit, 0)),
            ),
            (
                "202",
                vec![d, d, d],
                vec![Token::Date],
                Some(TokenizingError::Expected(Token::Date, Char::Digit, 3)),
            ),
            (
                "2028",
                vec![d, d, d, d],
                // TODO Shouldn't it be Token::Date? It would be weird to change autocomplete type just for one character in the middle of the date
                vec![Token::DateSeparator],
                Some(TokenizingError::Expected(
                    Token::DateSeparator,
                    Char::Dash,
                    4,
                )),
            ),
            (
                "2028-",
                vec![d, d, d, d, ds],
                vec![Token::Date],
                Some(TokenizingError::Expected(Token::Date, Char::Digit, 5)),
            ),
            (
                "2 ",
                vec![d],
                vec![Token::Date],
                Some(TokenizingError::Expected(Token::Date, Char::Digit, 1)),
            ),
            (
                "2022-",
                vec![d, d, d, d, ds],
                vec![Token::Date],
                Some(TokenizingError::Expected(Token::Date, Char::Digit, 5)),
            ),
            (
                "2022-01-02 23:11",
                vec![d, d, d, d, ds, d, d, ds, d, d, s, t, t, ts, t, t],
                vec![Token::Space],
                Some(TokenizingError::Expected(Token::Space, Char::Space, 16)),
            ),
            (
                "2022-01-02 23:11 ",
                vec![d, d, d, d, ds, d, d, ds, d, d, s, t, t, ts, t, t, s],
                vec![Token::DateTimeSeparator, Token::Time],
                Some(TokenizingError::DateOrTimeExpected(17)),
            ),
            (
                "2022-01-02 23:11 1",
                vec![d, d, d, d, ds, d, d, ds, d, d, s, t, t, ts, t, t, s, t],
                vec![Token::Time],
                Some(TokenizingError::Expected(Token::Time, Char::Digit, 18)),
            ),
            (
                "2022-01-02 23:11 - 2",
                vec![
                    d, d, d, d, ds, d, d, ds, d, d, s, t, t, ts, t, t, s, ds, s, d,
                ],
                vec![Token::Date],
                Some(TokenizingError::Expected(Token::Date, Char::Digit, 20)),
            ),
            (
                " 2022", // Dates are strict and no extra spaces are allowed
                vec![],
                vec![Token::Date],
                Some(TokenizingError::Expected(Token::Date, Char::Digit, 0)),
            ),
        ];
        for (input, tokens, next, error) in cases {
            let got = Tokenizer::new(input, true);
            assert_eq!(got.error, error, "input |{input}|");
            assert_eq!(got.expected_next, next, "input |{input}|");
            assert_eq!(got.tokens, tokens, "input |{input}|");
        }
    }

    #[test]
    fn tokenizing_tags() {
        // Aliases for easier testing
        let d = Token::Date;
        let ds = Token::DateSeparator;
        let c = Token::Comment;
        let s = Token::Space;
        let t = Token::Time;
        let tn = Token::TagName;
        let pn = Token::PropertyName;
        let po = Token::PropertyOperator;
        let pv = Token::PropertyValue;
        let ts = Token::TagSeparator;
        let time_s = Token::TimeSeparator;

        let datetime_prefix = "2022-01-02 23:11 12:23 ";
        let datetime_tokens = vec![
            d, d, d, d, ds, d, d, ds, d, d, s, t, t, time_s, t, t, s, t, t, time_s, t, t, s,
        ];

        let cases = vec![
            ("a", vec![tn], vec![Token::TagName], None),
            ("a ", vec![tn, s], vec![Token::PropertyName], None),
            (
                "a.",
                vec![tn, ts],
                vec![Token::TagName, Token::Comment],
                None,
            ),
            (
                "a. ",
                vec![tn, ts, s],
                vec![Token::TagName, Token::Comment],
                None,
            ),
            ("a. b", vec![tn, ts, s, tn], vec![Token::TagName], None),
            (
                "a. b  .zz ",
                vec![tn, ts, s, tn, s, s, ts, tn, tn, s],
                vec![Token::PropertyName],
                None,
            ),
            (
                "aa. Cc ",
                vec![tn, tn, ts, s, c, c, c],
                vec![Token::Comment],
                None,
            ),
            (
                "aa. Cc .C\"-.:",
                vec![tn, tn, ts, s, c, c, c, c, c, c, c, c, c],
                vec![Token::Comment],
                None,
            ),
            ("a p", vec![tn, s, pn], vec![Token::PropertyName], None),
            (
                "a p. Cc",
                vec![tn, s, pn, ts, s, c, c],
                vec![Token::Comment],
                None,
            ),
            (
                "a p. b p",
                vec![tn, s, pn, ts, s, tn, s, pn],
                vec![Token::PropertyName],
                None,
            ),
            (
                "a  pp . ",
                vec![tn, s, s, pn, pn, s, ts, s],
                vec![Token::TagName, Token::Comment],
                None,
            ),
            (
                "a p=",
                vec![tn, s, pn, po],
                vec![Token::PropertyValue],
                Some(TokenizingError::Expected(
                    Token::PropertyValue,
                    Char::AnyNonSeparator,
                    27,
                )),
            ),
            (
                "a p=v v",
                vec![tn, s, pn, po, pv, s, pn],
                vec![Token::PropertyName],
                None,
            ),
            (
                "a p=v v c",
                vec![tn, s, pn, po, pv, s, pn, s, pv],
                vec![Token::PropertyValue],
                None,
            ),
            (
                "a p=\"C",
                vec![tn, s, pn, po, pv, pv],
                vec![Token::PropertyValue],
                Some(TokenizingError::Expected(
                    Token::PropertyValue,
                    Char::Quote,
                    29,
                )),
            ),
            (
                "a p=\"C c\"",
                vec![tn, s, pn, po, pv, pv, pv, pv, pv],
                vec![Token::PropertyValue],
                None,
            ),
            (
                "a p=\"C c\" ",
                vec![tn, s, pn, po, pv, pv, pv, pv, pv, s],
                vec![Token::PropertyName],
                None,
            ),
            (
                "a pp=ff. Cc",
                vec![tn, s, pn, pn, po, pv, pv, ts, s, c, c],
                vec![Token::Comment],
                None,
            ),
            (
                "t p=1.2",
                vec![tn, s, pn, po, pv, pv, pv],
                vec![Token::PropertyValue],
                None,
            ),
            (
                "t p=1",
                vec![tn, s, pn, po, pv],
                vec![Token::PropertyValue],
                None,
            ),
            (
                "t p=1.",
                vec![tn, s, pn, po, pv, ts],
                vec![Token::PropertyValue],
                None,
            ),
            (
                "t p=1. C",
                vec![tn, s, pn, po, pv, ts, s, c],
                vec![Token::Comment],
                None,
            ),
            (
                "tag1. Cc",
                vec![tn, tn, tn, tn, ts, s, c, c],
                vec![Token::Comment],
                None,
            ),
            (
                "aa. bb pp=🧠. Ccc",
                vec![tn, tn, ts, s, tn, tn, s, pn, pn, po, pv, ts, s, c, c, c],
                vec![Token::Comment],
                None,
            ),
        ];
        for (input, tokens, next, error) in cases {
            let input = format!("{datetime_prefix}{input}");
            let got = Tokenizer::new(&input, true);
            let (date_tokens, tags_tokens) = got.tokens.split_at(datetime_tokens.len());
            assert_eq!(
                date_tokens, datetime_tokens,
                "wrong date tokens, input {input}"
            );
            assert_eq!(tags_tokens, tokens, "wrong tokens, input {input}");
            assert_eq!(got.error, error, "wrong error, input {input}");
            assert_eq!(got.expected_next, next, "wrong next, input {input}");
        }
    }
}
