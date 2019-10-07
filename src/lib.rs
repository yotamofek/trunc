use unicode_segmentation::UnicodeSegmentation;

pub trait TruncateToBoundary {
    fn truncate_to_boundary(&self, chars: usize) -> &Self;
    fn truncate_to_byte_offset(&self, count: usize) -> &Self;
    fn slice_indices_at_offset(&self, offset: usize) -> (&Self, usize);
    fn slice_indices_at_boundary(&self, boundary: usize) -> (&Self, usize);
}

pub trait SplitToBoundary {
    fn split_to_offset(&self, offset: usize) -> Vec<&str>;
    fn split_to_boundary(&self, boundary: usize) -> Vec<&str>;
    fn split_all_to_boundary(&self, boundary: usize) -> Vec<&str>;
}

pub trait SplitInplaceToBoundary {
    fn split_to_offset_inplace(&mut self, offset: usize) -> &mut Self;
    fn split_to_boundary_inplace(&mut self, boundary: usize) -> &mut Self;
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

    fn slice_indices_at_boundary(&self, boundary: usize) -> (&Self, usize) {
        if boundary == 0 {
            return (&self[..0], 0);
        }

        let (result, offset) = match self.char_indices().nth(boundary) {
            None => (self, self.len()),
            Some((b, char)) => self.slice_indices_at_offset(b)
        };
        (result, offset)
    }
}


impl SplitToBoundary for str {

    fn split_to_offset(&self, offset: usize) -> Vec<&str> {
        if offset > self.len() {
            return vec!(&self)
        }
        let (head, offset) = self.slice_indices_at_offset(offset);
        vec!(head.trim(), &self[offset..])

    }

    fn split_to_boundary(&self, boundary: usize) -> Vec<&str> {
        let (head, offset) = self.slice_indices_at_boundary(boundary);
        if offset == self.len() {
             return vec!(&self)
        }
        vec!(head.trim(), &self[offset..])
    }

    fn split_all_to_boundary(&self, boundary: usize) -> Vec<&str> {
        let mut offset = 0usize;
        let mut result = Vec::new();
        while offset < self.len() {
            let (head, byteoffset) = self[offset..].slice_indices_at_boundary(boundary);
            if byteoffset == 0  {
                 let (_, b) = self[offset..].slice_indices_at_boundary(boundary+1); {
                    offset = offset+b;
                }
            continue
            }
            else if !(head.trim().as_bytes() == b"") {
                result.push(head);
            }
            offset = offset+byteoffset;
        }
        result
    }
}


impl SplitInplaceToBoundary for Vec<&str> {

    fn split_to_offset_inplace(&mut self, offset: usize) -> &mut Self {
        if let Some(string) = self.pop() {
            let mut new;
            match string  {
                "" | " " => new = vec!("", ""),
                _ => new = string.split_to_boundary(offset)
            }
            self.append(&mut new);
        }
        self
    }

    fn split_to_boundary_inplace(&mut self, offset: usize) -> &mut Self {
        if let Some(string) = self.pop() {
            let mut new = string.split_to_boundary(offset);
            self.append(&mut new);
        }
        self
    }
}

impl SplitToBoundary for Vec<&str> {

    fn split_to_offset(&self, offset: usize) -> Vec<&str> {
        let mut result = self.clone();
        if let Some(string) = result.pop(){
            let mut new = string.split_to_offset(offset);
            result.append(&mut new);
        }
        result
    }

    fn split_to_boundary(&self, boundary: usize) -> Vec<&str> {
        let mut result = self.clone();
        if let Some(string) = result.pop(){
            let mut new = string.split_to_boundary(boundary);
            result.append(&mut new);
        }
        result
    }

    fn split_all_to_boundary(&self, boundary: usize) -> Vec<&str> {
        unimplemented!()
    }
}

// perhaps for future use?
fn sanitize_string_vec(list: Vec<&str>) -> Vec<&str> {
    list.iter().
    filter(|&&x| x.trim().as_bytes() != b"")
    .map(|x| *x)
    .collect()
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
    fn test_split_bytes(){
        let s = "🤚🏾a🤚 ";
        assert_eq!(s.split_to_offset(8), vec!("🤚🏾", "a🤚 "));
        assert_eq!(s.split_to_offset(9), vec!("🤚🏾a", "🤚 "));
    }

    #[test]
    fn test_split_boundary(){
        let s = "🤚🏾a🤚 ";
        assert_eq!(s.split_to_boundary(1), vec!("", "🤚🏾a🤚 "));
        assert_eq!(s.split_to_boundary(2), vec!("🤚🏾", "a🤚 "));
        assert_eq!(s.split_to_boundary(3), vec!("🤚🏾a", "🤚 "));
        assert_eq!(s.split_to_boundary(4), vec!("🤚🏾a🤚", " "));
        assert_eq!(s.split_to_boundary(5), vec!(s));
        assert_eq!(s.split_to_boundary(6), vec!(s));
        assert_eq!(s.split_to_boundary(15), vec!(s));
    }


    #[test]
    fn test_split_all(){
        let mut s = "🤚🏾a🤚 ";
        assert_eq!(s.split_all_to_boundary(1), vec!("a", "🤚"));
        assert_eq!(s.split_all_to_boundary(2), vec!("🤚🏾", "a🤚",));
        assert_eq!(s.split_all_to_boundary(3), vec!("🤚🏾a", "🤚 "));
        assert_eq!(s.split_all_to_boundary(4), vec!("🤚🏾a🤚"));
        assert_eq!(s.split_all_to_boundary(14), vec!("🤚🏾a🤚 "));

        let mut s1 = "🤚🏾a🤚🏾 ";
        assert_eq!(s1.split_all_to_boundary(1), vec!("a"));
        assert_eq!(s1.split_all_to_boundary(2), vec!("🤚🏾", "a", "🤚🏾"));
    }

    #[test]
    fn test_inplace_vector_chaining(){
        let mut s = vec!("🤚🏾a🤚 ", "🤚🤚🤚 ");
        s
                .split_to_boundary_inplace(1)
                .split_to_boundary_inplace(1)
                .split_to_boundary_inplace(1);
        assert_eq!(s, vec!("🤚🏾a🤚 ", "🤚", "🤚", "🤚", " "));

        let mut s1 = vec!("🤚🏾a🤚 ", "🤚🏾 🤚 ");
        s1.split_to_boundary_inplace(1);
        assert_eq!(s1, vec!("🤚🏾a🤚 ", "", "🤚🏾 🤚 "));
        s1.split_to_boundary_inplace(3);
        assert_eq!(s1, vec!("🤚🏾a🤚 ", "", "🤚🏾", "🤚 "));
        let mut s2 = vec!("🤚🏾a🤚 ", "🤚🏾🤚🏾 ");
        s2.split_to_boundary_inplace(2);
        assert_eq!(s2, vec!("🤚🏾a🤚 ","🤚🏾", "🤚🏾 "));
        s2.split_to_boundary_inplace(2);
        assert_eq!(s2, vec!("🤚🏾a🤚 ","🤚🏾", "🤚🏾"," "));
    }
}
