#[cfg(test)]
mod tests {
 use regex_quote_fixer::CharacterClass;
 use regex_quote_fixer::RegexQuoteFixer;
 #[test]
 fn test_from_chars() {
  let rqf = RegexQuoteFixer::from_chars(vec!['?', '(', ')']);
  assert_eq!(true, (rqf.lambda)('?'));
  assert_eq!(true, (rqf.lambda)('('));
  assert_eq!(true, (rqf.lambda)(')'));
  assert_eq!(false, (rqf.lambda)('!'));
 }

 #[test]
 fn test_from_string() {
  let rqf = RegexQuoteFixer::from_string("?()".into());
  assert_eq!(true, (rqf.lambda)('?'));
  assert_eq!(true, (rqf.lambda)('('));
  assert_eq!(true, (rqf.lambda)(')'));
  assert_eq!(false, (rqf.lambda)('!'));
 }

 #[test]
 fn test_from_lambda() {
  let rqf = RegexQuoteFixer::from_lambda(Box::new(|x| x == '_'));
  assert_eq!(true, (rqf.lambda)('_'));
  assert_eq!(false, (rqf.lambda)('!'));
 }

 #[test]
 fn test_fix_regex() {
  let rqf = RegexQuoteFixer::from_lambda(Box::new(|x| x == '?'));
  assert_eq!("", rqf.fix(""));
  assert_eq!("a", rqf.fix("a"));
  assert_eq!("ab", rqf.fix("ab"));
  assert_eq!("\\", rqf.fix("\\"));
  assert_eq!("\\\\", rqf.fix("\\\\"));
  assert_eq!("\\\\\\", rqf.fix("\\\\\\"));
  assert_eq!("\\\\\\\\", rqf.fix("\\\\\\\\"));
  assert_eq!("\\b", rqf.fix("\\b"));
  assert_eq!("a\\", rqf.fix("a\\"));
  assert_eq!("a\\b", rqf.fix("a\\b"));

  assert_eq!("\\?", rqf.fix("?"));
  assert_eq!("?", rqf.fix("\\?"));
  assert_eq!("\\??", rqf.fix("?\\?"));
  assert_eq!("?\\?", rqf.fix("\\??"));

  assert_eq!("\\?\\?\\", rqf.fix("??\\"));
  assert_eq!("??\\", rqf.fix("\\?\\?\\"));
 }

 #[test]
 fn test_quotechar_in_character_class() {
  let rqf = RegexQuoteFixer::for_grep();

  // as used with grep
  let needle = r#"^a\+[]\?]b\+$"#;

  let needle_fixed = rqf.fix(needle);

  assert_eq!(needle_fixed, r#"^a+[]\\?]b+$"#);

  use regex::Regex;

  let regex = Regex::new(&needle_fixed).unwrap();

  assert!(regex.is_match(r#"a]b"#));
  assert!(regex.is_match(r#"aa]bb"#));
  assert!(regex.is_match(r#"a?b"#));
  assert!(regex.is_match(r#"a\b"#));

  let needle2 = rqf.fix(&needle_fixed);

  assert_eq!(needle, needle2);
 }

 #[test]
 fn test_quotechar_in_character_class_1() {
  let mut rqf = RegexQuoteFixer::for_grep();
  rqf.cc = CharacterClass::KeepUnaltered;

  // as used with grep
  let needle = r#"^a\+[]\?]b\+$"#;

  let needle_fixed = rqf.fix(needle);

  assert_eq!(needle_fixed, r#"^a+[]\?]b+$"#);

  use regex::Regex;

  let regex = Regex::new(&needle_fixed).unwrap();

  assert!(regex.is_match(r#"a]b"#));
  assert!(regex.is_match(r#"aa]bb"#));
  assert!(regex.is_match(r#"a?b"#));
  assert!(!regex.is_match(r#"a\b"#));

  let needle2 = rqf.fix(&needle_fixed);

  assert_eq!(needle, needle2);
 }

 #[test]
 fn test_quotechar_in_character_class_ignored_2() {
  let mut rqf = RegexQuoteFixer::for_grep();
  rqf.cc = CharacterClass::Ignore;

  // as used with grep
  let needle = r#"^a\+[]\?]b\+$"#;

  let needle_fixed = rqf.fix(needle);

  assert_eq!(needle_fixed, r#"^a+[]?]b+$"#);

  use regex::Regex;

  let regex = Regex::new(&needle_fixed).unwrap();

  assert!(regex.is_match(r#"a]b"#));
  assert!(regex.is_match(r#"aa]bb"#));
  assert!(regex.is_match(r#"a?b"#));
  assert!(!regex.is_match(r#"a\b"#));

  let needle2 = rqf.fix(&needle_fixed);

  assert_eq!(needle, needle2);
 }

 #[test]
 fn test_free_requoting() {
  let rqf = RegexQuoteFixer {
   lambda: Box::new(|x| x == 'b'),
   quote_char: 'c',
   cc: CharacterClass::Ignore,
  };
  let s1 = "abcccbd";
  let s2 = rqf.fix( s1);
  assert_eq!( s2, "acbccbd");
  let s3 = rqf.fix( &s2);
  assert_eq!( s1, s3);
 }
}
