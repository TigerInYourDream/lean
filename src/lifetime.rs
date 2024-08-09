#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    reminder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            reminder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let reminder = self.reminder.as_mut()?;
        if let Some((delimiter_start, delimiter_end)) = self.delimiter.find_next(reminder) {
            let until_delimiter = &reminder[..delimiter_start];
            *reminder = &reminder[delimiter_end..];
            Some(until_delimiter)
        } else {
            self.reminder.take()
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // character char only have 1 lang
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, self.len_utf8()))
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}
// cnanot compile in earyly version please look at longest_test in test mod
pub fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

#[cfg(test)]
mod test {
    use crate::{longest, until_char, StrSplit};

    #[test]
    fn test() {
        let haystack = "a b c d e";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn tail() {
        let haystack = "a b c d ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
    }

    #[test]
    fn test_util() {
        assert_eq!(until_char("hello world", 'o'), "hell");
    }

    #[test]
    fn test_longest() {
        let string1 = "hello world string 1 long".to_string();
        {
            let string2 = "hello world string 2".to_string();
            let res = longest(&string1, &string2);
            println!("res: {}", res);
        }
    }
}
