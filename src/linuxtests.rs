#[cfg(target_os = "linux")]
#[cfg(test)]
mod tests {
 use std::{
  os::fd::FromRawFd,
  process::{Command, ExitStatus, Stdio},
 };

 fn rungrep(needle: &str, haystack: &str) -> bool {
  let mut haystack = haystack.to_string();
  haystack.push('\n');
  let mut grep = Command::new("grep")
   .stdin(Stdio::piped())
   .stdout(Stdio::piped())
   .arg(needle)
   .spawn()
   .unwrap();

  use std::io::Write;

  let _ = grep
   .stdin
   .as_ref()
   .unwrap()
   .write(haystack.as_bytes())
   .unwrap();

  grep.wait().unwrap().success()
 }

 fn runboth(needle: &str, haystack: &str) -> bool {
  let condition1 = rungrep(needle, haystack);
  let condition2 = runregex(needle, haystack);
  assert_eq!(condition1, condition2);

  condition1
 }

 use regex::Regex;

 use crate::RegexQuoteFixer;

 fn runregex(needle: &str, haystack: &str) -> bool {
  let regex = Regex::new(needle).unwrap();
  regex.is_match(haystack)
 }

 fn compare(needle: &str, haystack: &str) -> bool {
  let rqf = RegexQuoteFixer::for_grep();
  let needle2 = rqf.fix(needle);
  let needle2 = needle2.as_str();

  let condition1 = rungrep(needle, haystack) == runregex(needle2, haystack);
  assert!(condition1);
  let condition2 = rungrep(needle2, haystack) == runregex(needle, haystack);
  assert!(condition2);

  condition1 && condition2
 }

 #[test]
 fn test_grep() {
  assert!(rungrep(".", "abc"));
  assert!(!rungrep("x", "abc"));
 }

 #[test]
 fn test_regex() {
  assert!(runregex(".", "abc"));
  assert!(!runregex("x", "abc"));
 }

 #[test]
 fn test_basics() {
  assert!(rungrep("a\\?", ""));
  assert!(runregex("a?", ""));
  assert!(rungrep("ab\\?c", "ac"));
  assert!(runregex("ab?c", "ac"));
  assert!(rungrep("ab\\?c", "abc"));
  assert!(runregex("ab?c", "abc"));
  assert!(!rungrep("ab\\?c", "abbc"));
  assert!(!runregex("ab?c", "abbc"));

  assert!(runboth("\\\\", "\\"));

  assert!(runboth("a[bc]d", "abd"));
  assert!(runboth("a[bc]d", "acd"));

  assert!(runboth("a[bc]d", " abd "));
  assert!(runboth("a[bc]d", " acd "));

  assert!(runboth("^a[bc]d$", "abd"));
  assert!(runboth("^a[bc]d$", "acd"));

  assert!(!runboth("^a[bc]d$", " abd"));
  assert!(!runboth("^a[bc]d$", " acd"));

  assert!(!runboth("^a[bc]d$", "abd "));
  assert!(!runboth("^a[bc]d$", "acd "));

  assert!(runboth("a\\[bc]d", "a[bc]d"));
  assert!(runboth("a\\[bc\\]d", "a[bc]d"));

  assert!(runboth("a\\[bc\\]d", "a[bc]d"));

  assert!(runboth("ab*d", "ad"));
  assert!(runboth("ab*d", "abd"));
  assert!(runboth("ab*d", "abbd"));

  assert!(!rungrep("ab\\+d", "ad"));
  assert!(!runregex("ab+d", "ad"));

  assert!(rungrep("ab\\+d", "abd"));
  assert!(runregex("ab+d", "abd"));
  assert!(rungrep("ab\\+d", "abbd"));
  assert!(runregex("ab+d", "abbd"));

  assert!(!runregex("ab{2,3}d", "abd"));
  assert!(!rungrep("ab\\{2,3\\}d", "abd"));
  assert!(runregex("ab{2,3}d", "abbd"));
  assert!(rungrep("ab\\{2,3\\}d", "abbd"));
  assert!(runregex("ab{2,3}d", "abbbd"));
  assert!(rungrep("ab\\{2,3\\}d", "abbbd"));
 }

 #[test]
 fn test_comparison() {
  assert!(compare("a?", ""));

  assert!(compare("ab?c", "ac"));
  assert!(compare("ab?c", "abc"));
  assert!(compare("ab?c", "abbc"));

  assert!(compare("ab+c", "ac"));
  assert!(compare("ab+c", "abc"));
  assert!(compare("ab+c", "abbc"));

  assert!(compare("ab{2,3}d", "ad"));
  assert!(compare("ab{2,3}d", "abd"));
  assert!(compare("ab{2,3}d", "abbd"));
  assert!(compare("ab{2,3}d", "abbbd"));
  assert!(compare("ab{2,3}d", "abbbbd"));
 }

 #[test]
 fn test_character_classes() {
  assert!(runboth("[{}]", "{"));
  assert!(runboth("[()]", ")"));
  assert!(runboth("[(?)]", "?"));
  assert!(runboth("^[[:alnum:]]*$", "xyz"));
  assert!(runboth("^[][:alnum:]]*$", "]"));
  assert!(runboth("^[][:alnum:]]*$", "]xyz]"));
  assert!(compare("[{}]", "{"));
  assert!(compare("[{}]", "\\"));
  assert!(compare("[()]", "\\"));
  assert!(compare("[?]", "\\"));
  assert!(compare("[+]", "\\"));
 }
}
