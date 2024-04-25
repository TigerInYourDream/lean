#[derive(Debug)]
pub struct StrSplit<'heystack, D> {
    reminder: Option<&'heystack str>,
    delimiter: D,
}

impl<'heystack, D> StrSplit<'heystack, D> {
    pub fn new(haystack: &'heystack str, delimiter: D) -> Self {
        Self {
            reminder: Some(haystack),
            delimiter,
        }
    }
}

impl<'heystack, D> Iterator for StrSplit<'heystack, D>
where
    D: Delimiter,
{
    type Item = &'heystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let reminder = self.reminder.as_mut()?;
        if let Some((delimter_start, delimiter_end)) = self.delimiter.find_next(reminder) {
            let until_delimiter = &reminder[..delimter_start];
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
        // charactar char only have 1 lang
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

#[cfg(test)]
mod test {
    use crate::{until_char, StrSplit};

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
}
