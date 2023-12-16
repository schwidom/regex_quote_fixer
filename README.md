# regex_quote_fixer
Rewrites grep regexpressions for the use in the regex crate.

This crate allows to translate between regexpressions of different regexpression
implementations by deciding when to add or remove the metachar '\\' from the regexpression
string.

Example:
```rust
use regex_quote_fixer::RegexQuoteFixer;
let rqf = RegexQuoteFixer::for_grep();

// as used with grep
let needle = r#"https\?://\([[:alnum:].]*\.\)\?example\.com/"#;

let needle_fixed = rqf.fix( needle);

assert_eq!( needle_fixed, r#"https?://([[:alnum:].]*\.)?example\.com/"#);

use regex::Regex;

let regex = Regex::new( &needle_fixed).unwrap();

assert!( regex.is_match( r#"https://www.example.com/"#));
assert!( regex.is_match( r#"http://www.example.com/"#));
assert!( regex.is_match( r#"http://example.com/"#));

// and it is also possible to convert regex compatible regexpressions
// to grep regexpressions :

let needle2 = rqf.fix( &needle_fixed);

assert_eq!( needle, needle2);

```

Another Example:

```rust
use regex_quote_fixer::RegexQuoteFixer;
let rqf = RegexQuoteFixer::for_grep();

// as used with grep
let needle = r#"^a\+[]\?]b\+$"#;

let needle_fixed = rqf.fix( needle);

assert_eq!( needle_fixed, r#"^a+[]\\?]b+$"#);

use regex::Regex;

let regex = Regex::new( &needle_fixed).unwrap();

assert!( regex.is_match( r#"a]b"#));
assert!( regex.is_match( r#"aa]bb"#));
assert!( regex.is_match( r#"a?b"#));
assert!( regex.is_match( r#"a\b"#));

// and it is also possible to convert this regex compatible regexpression
// to a grep compatible regexpressions :

let needle2 = rqf.fix( &needle_fixed);

assert_eq!( needle, needle2);

```


