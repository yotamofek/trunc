use unicode_segmentation::UnicodeSegmentation;

pub trait TruncateToBoundary {
    fn truncate_to_boundary(&self, chars: usize) -> &Self;
    fn truncate_to_byte_offset(&self, count: usize) -> &Self;
    fn slice_indices_at_offset(&self, boundary: usize) -> (&Self, usize);
}

pub trait SplitToBoundary {
    fn split_to_offset(&self, offset: usize) -> Vec<&str>;
    fn split_to_boundary(&self, offset: usize) -> Vec<&str>;
    fn split_all_to_boundary(&self, offset: usize) -> Vec<&str>;
}


impl SplitToBoundary for dyn Iterator<Item=&str> {
    fn split_to_offset(&self, offset: usize) -> Vec<&str> {
        unimplemented!()
    }

    fn split_to_boundary(&self, offset: usize) -> Vec<&str> {
        unimplemented!()
    }

    fn split_all_to_boundary(&self, offset: usize) -> Vec<&str> {
        unimplemented!()
    }
}

impl SplitToBoundary for str {

    fn split_to_offset(&self, offset: usize) -> Vec<&str> {
    (head, offset) = self.slice_indices_at_offset();
    vec!(head, &self[offset..])
    }

    fn split_to_boundary(&self, offset: usize) -> Vec<&str> {
        unimplemented!()
    }

    fn split_all_to_boundary(&self, offset: usize) -> Vec<&str> {
        unimplemented!()
    }
}

impl TruncateToBoundary for str {
    /// Truncates a given string to a set numerical boundary.
    /// If the boundary splits a grapheme (e.g., when a character is a resultant mix of more than 1 utf-8 character, like some emojis)
    /// the truncation will scale back to the previous character.
    /// If the truncation ends with white space - this will be trimmed.
    /// Should the truncation boundary exceed the string's size - the original string will return (including whitespace).
    ///
    /// # Examples:
    ///
    ///
    /// ```
    /// use truncrate::*;
    ///
    /// let s = "🤚🏾a🤚🏾 ";
    ///
    /// assert_eq!(s.truncate_to_boundary(1), "");
    /// assert_eq!(s.truncate_to_boundary(2), "🤚🏾");
    /// assert_eq!(s.truncate_to_boundary(3), "🤚🏾a");
    /// assert_eq!(s.truncate_to_boundary(4), "🤚🏾a");
    /// assert_eq!(s.truncate_to_boundary(5), "🤚🏾a🤚🏾");
    /// assert_eq!(s.truncate_to_boundary(10), s);
    ///```
    fn truncate_to_boundary(&self, chars: usize) -> &Self {
        if chars == 0 {
            return &self[..0];
        }

        let result = match self.char_indices().nth(chars) {
            None => self,
            Some((boundary, _)) => self.truncate_to_byte_offset(boundary)
        };
        result
    }
    /// Truncates a given string based on the provided byte-offset.
    /// If the offset splits a grapheme the truncation will scale back to the previous character.
    /// If the truncation ends with white space - this will be trimmed.
    /// Should the offset exceed the strings size - the original string will return (including whitespace).
    /// # Examples:
    ///
    /// ```
    /// use truncrate::*;
    ///
    /// let s = "🤚🏾a🤚 ";
    ///  // where "🤚🏾" = 8 bytes
    /// assert_eq!(s.truncate_to_byte_offset(0), "");
    /// assert_eq!(s.truncate_to_byte_offset(7), "");
    /// assert_eq!(s.truncate_to_byte_offset(8), "🤚🏾");
    /// assert_eq!(s.truncate_to_byte_offset(9), "🤚🏾a");
    /// assert_eq!(s.truncate_to_byte_offset(10), "🤚🏾a");
    /// assert_eq!(s.truncate_to_byte_offset(18), s);
    /// ```
    fn truncate_to_byte_offset(&self, boundary: usize) -> &Self {
        if boundary > self.len() {
            return &self
        }
        let mut grapheme_iter = self
                .grapheme_indices(true)
                .rev()
                .skip_while(move |(n, _)| *n > boundary);
        let mut bytecount = boundary;
        if let Some((grapheme_boundary, _)) = grapheme_iter.next() {
            bytecount = grapheme_boundary;
        }

        &self[..bytecount].trim_end()
    }

    /// The same as 'truncate_to_byte_offset' but returns a tuple with the desired slice along with the byte-offset.
    fn slice_indices_at_offset(&self, boundary: usize) -> (&Self, usize) {
        if boundary > self.len() {
            return (&self, self.len())
        }
        let mut grapheme_iter = self
                .grapheme_indices(true)
                .rev()
                .skip_while(move |(n, _)| *n > boundary);
        let mut bytecount = boundary;
        if let Some((grapheme_boundary, _)) = grapheme_iter.next() {
            bytecount = grapheme_boundary;
        }

        (&self[..bytecount].trim_end(), bytecount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "🤚🏾a🤚🏾 🤚🏾\t 🤚🏾";

        assert_eq!(s.truncate_to_boundary(1), "");
        assert_eq!(s.truncate_to_boundary(2), "🤚🏾");
        assert_eq!(s.truncate_to_boundary(3), "🤚🏾a");
        assert_eq!(s.truncate_to_boundary(4), "🤚🏾a");
        assert_eq!(s.truncate_to_boundary(6), "🤚🏾a🤚🏾");
        assert_eq!(s.truncate_to_boundary(7), "🤚🏾a🤚🏾");
        assert_eq!(s.truncate_to_boundary(8), "🤚🏾a🤚🏾 🤚🏾");
        assert_eq!(s.truncate_to_boundary(9), "🤚🏾a🤚🏾 🤚🏾");
        assert_eq!(s.truncate_to_boundary(10), "🤚🏾a🤚🏾 🤚🏾");
        assert_eq!(s.truncate_to_boundary(11), "🤚🏾a🤚🏾 🤚🏾");
        assert_eq!(s.truncate_to_boundary(12), s);
    }

    #[test]
    fn test_non_trucated_string() {
        let s = "🤚🏾a🤚🏾 🤚🏾  🤚🏾";

        assert_eq!(s.truncate_to_boundary(100), s);
        assert_eq!(s.truncate_to_boundary(s.chars().count()), s);
        assert_eq!(s.truncate_to_boundary(0), "");
    }

    #[test]
    fn truncate_non_split_grapheme() {
        let s = "🤚🏾a🤚 🤚🏾\t 🤚    ";

        assert_eq!(s.truncate_to_boundary(4), "🤚🏾a🤚");
        assert_eq!(s.truncate_to_boundary(5), "🤚🏾a🤚");
        assert_eq!(s.truncate_to_boundary(6), "🤚🏾a🤚");
        assert_eq!(s.truncate_to_boundary(7), "🤚🏾a🤚 🤚🏾");
        assert_eq!(s.truncate_to_boundary(8), "🤚🏾a🤚 🤚🏾");
        assert_eq!(s.truncate_to_boundary(9), "🤚🏾a🤚 🤚🏾");
        assert_eq!(s.truncate_to_boundary(10), "🤚🏾a🤚 🤚🏾\t 🤚");
        assert_eq!(s.truncate_to_boundary(11), "🤚🏾a🤚 🤚🏾\t 🤚");
        assert_eq!(s.truncate_to_boundary(12), "🤚🏾a🤚 🤚🏾\t 🤚");
        assert_eq!(s.truncate_to_boundary(20), s);
    }

    #[test]
    fn truncate_non_split_grapheme_with_whitespace() {
        let s = " 🤚🏾a🤚 🤚🏾\t 🤚    ";

        assert_eq!(s.truncate_to_boundary(5), " 🤚🏾a🤚");
        assert_eq!(s.truncate_to_boundary(6), " 🤚🏾a🤚");
        assert_eq!(s.truncate_to_boundary(7), " 🤚🏾a🤚");
        assert_eq!(s.truncate_to_boundary(8), " 🤚🏾a🤚 🤚🏾");
        assert_eq!(s.truncate_to_boundary(9), " 🤚🏾a🤚 🤚🏾");
        assert_eq!(s.truncate_to_boundary(10), " 🤚🏾a🤚 🤚🏾");
        assert_eq!(s.truncate_to_boundary(11), " 🤚🏾a🤚 🤚🏾\t 🤚");
        assert_eq!(s.truncate_to_boundary(12), " 🤚🏾a🤚 🤚🏾\t 🤚");
        assert_eq!(s.truncate_to_boundary(13), " 🤚🏾a🤚 🤚🏾\t 🤚");
        assert_eq!(s.truncate_to_boundary(21), s);
    }
    #[test]
    fn truncate_to_bytes(){
        let s = "🤚🏾a🤚 ";

        assert_eq!(s.truncate_to_byte_offset(1), "");
        assert_eq!(s.truncate_to_byte_offset(2), "");
        assert_eq!(s.truncate_to_byte_offset(13), "🤚🏾a🤚");
        assert_eq!(s.truncate_to_byte_offset(14), "🤚🏾a🤚");
        assert_eq!(s.truncate_to_byte_offset(18), s);
        assert_eq!(s.truncate_to_byte_offset(100), s);
    }

    #[test]
    fn test_split_offset(){
        let s = "🤚🏾a🤚 ";
        assert_eq!(s.split_to_offset(8), vec!("🤚🏾", "a🤚 "));
    }

    #[test]
    fn test_split_bytes(){
        let s = "🤚🏾a🤚 ";

    }


    #[test]
    fn test_split_all(){
        let s = "🤚🏾a🤚 ";

    }

    #[test]
    fn test_vector_tailsplit_chaining(){
        let s = "🤚🏾a🤚 ";

    }
}
