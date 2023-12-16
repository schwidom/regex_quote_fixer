/*!
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

*/

/// holds a lambda which decides which char has to be quoted / unquoted and does the transformation of regex strings
pub struct RegexQuoteFixer {
 pub lambda: Box<dyn Fn(char) -> bool>,
}

impl RegexQuoteFixer {
 /// creates a lambda which returns true if the given character matches one of the vector
 pub fn from_chars(v: Vec<char>) -> Self {
  Self {
   lambda: Box::new(move |x| v.contains(&x)),
  }
 }

 /// creates a lambda which returns true if the given character matches one of the string
 pub fn from_string(s: String) -> Self {
  Self {
   lambda: Box::new(move |x| s.contains(x)),
  }
 }

 /// creates a RegexQuoteFixer which can translate between grep and the regex crate
 pub fn for_grep() -> Self {
  Self::from_string("()?+{}".into())
 }

 /// creates a RegexQuoteFixer which holds this lambda
 pub fn from_lambda(lambda: Box<dyn Fn(char) -> bool>) -> Self {
  Self { lambda }
 }

 /// translates regexpressions between different regexpression implementations by deciding when to add or remove the metachar '\\' from the regexpression string
 pub fn fix(&self, s: &str) -> String {
  let mut ret = String::new();

  struct CharacterClass {
   nth_char: usize,
   depth: u8,
  }

  let mut inside_character_class = Option::<CharacterClass>::None;
  let mut quote_char = false;

  for char in s.chars() {
   if let Some(cc) = &mut inside_character_class {
    cc.nth_char += 1;
    match char {
     ']' if cc.nth_char != 1 => cc.depth -= 1,
     '[' => cc.depth += 1,
     _ => {}
    }
    if cc.depth == 0 {
     inside_character_class = None;
    }

    // a quote char inside a characterclass has to be quoted for the regex crate
    match (quote_char, '\\' == char) {
     (false, true) => quote_char = true,
     (true, true) => {
      quote_char = false;
      ret.push('\\');
     }
     (true, false) => {
      quote_char = false;
      ret.push('\\');
      ret.push('\\');
      ret.push(char);
     }
     (false, false) => ret.push(char),
    }

    continue;
   }

   if char == '[' {
    if quote_char {
     ret.push('\\');
     ret.push(char);
     quote_char = false;
     continue;
    } else {
     inside_character_class = Some(CharacterClass {
      nth_char: 0,
      depth: 1,
     });
     ret.push(char);
     continue;
    }
   }

   if char == '\\' {
    if quote_char {
     ret.push('\\');
     ret.push('\\');
     quote_char = false;
     continue;
    } else {
     quote_char = true;
     continue;
    }
   }

   if (self.lambda)(char) {
    if quote_char {
     ret.push(char);
     quote_char = false;
     continue;
    } else {
     ret.push('\\');
     ret.push(char);
     continue;
    }
   } else {
    if quote_char {
     ret.push('\\');
     ret.push(char);
     quote_char = false;
     continue;
    } else {
     ret.push(char);
    }
   }
  }

  if quote_char {
   ret.push('\\');
  }

  ret
 }
}
