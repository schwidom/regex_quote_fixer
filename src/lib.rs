pub struct RegexQuoteFixer {
 pub lambda: Box<dyn Fn(char) -> bool>,
}

impl RegexQuoteFixer {
 pub fn from_chars(v: Vec<char>) -> Self {
  Self {
   lambda: Box::new(move |x| v.contains(&x)),
  }
 }

 pub fn from_string(s: String) -> Self {
  Self {
   lambda: Box::new(move |x| s.contains(x)),
  }
 }

 pub fn for_grep() -> Self {
  Self::from_string("()?+{}".into())
 }

 pub fn from_lambda(lambda: Box<dyn Fn(char) -> bool>) -> Self {
  Self { lambda }
 }

 pub fn fix(&self, s: &str) -> String {
  // let special_chars = vec!['(', ')', '?'];

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
    ret.push(char);
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

mod linuxtests;
mod tests;
