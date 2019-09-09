use unicode_segmentation::UnicodeSegmentation;

pub trait TruncateToBoundary {
    fn truncate_to_boundary(&self, chars: usize) -> &Self;
}

impl TruncateToBoundary for str {
    fn truncate_to_boundary(&self, chars: usize) -> &Self {
        if chars == 0 {
            return &self[..0];
        }

        let mut boundary = match self.trim_start().char_indices().nth(chars) {
            None => return self.trim(),
            Some((boundary, _)) => boundary,
        };

        let mut grapheme_iter = self
            .trim_start()
            .grapheme_indices(true)
            .rev()
            .skip_while(move |(n, _)| *n > boundary);

        if let Some((grapheme_boundary, _)) = grapheme_iter.next() {
            boundary = grapheme_boundary;
        }

        &self.trim_start()[..boundary].trim_end()
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
        assert_eq!(s.truncate_to_boundary(12), "🤚🏾a🤚🏾 🤚🏾\t 🤚🏾");
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
        assert_eq!(s.truncate_to_boundary(20), "🤚🏾a🤚 🤚🏾\t 🤚");
    }

    #[test]
    fn truncate_non_split_grapheme_leading_whitespace() {
        let s = "   🤚🏾a🤚 🤚🏾\t 🤚    ";

        assert_eq!(s.truncate_to_boundary(4), "🤚🏾a🤚");
        assert_eq!(s.truncate_to_boundary(5), "🤚🏾a🤚");
        assert_eq!(s.truncate_to_boundary(6), "🤚🏾a🤚");
        assert_eq!(s.truncate_to_boundary(7), "🤚🏾a🤚 🤚🏾");
        assert_eq!(s.truncate_to_boundary(8), "🤚🏾a🤚 🤚🏾");
        assert_eq!(s.truncate_to_boundary(9), "🤚🏾a🤚 🤚🏾");
        assert_eq!(s.truncate_to_boundary(10), "🤚🏾a🤚 🤚🏾\t 🤚");
        assert_eq!(s.truncate_to_boundary(11), "🤚🏾a🤚 🤚🏾\t 🤚");
        assert_eq!(s.truncate_to_boundary(12), "🤚🏾a🤚 🤚🏾\t 🤚");
        assert_eq!(s.truncate_to_boundary(20), "🤚🏾a🤚 🤚🏾\t 🤚");
    }
}
