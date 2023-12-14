#[cfg(test)]
mod tests {
 use crate::RegexQuoteFixer;
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
}
