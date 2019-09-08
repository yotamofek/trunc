use std::iter::repeat;
use unicode_segmentation::UnicodeSegmentation;

pub trait TruncateToBoundary {
    fn truncate_to_boundary(&self, chars: usize) -> &Self;
}

impl TruncateToBoundary for str {
    fn truncate_to_boundary(&self, mut chars: usize) -> &Self {
        let mut boundary = 0;

        let grapheme_indices = self.grapheme_indices(true).zip(
            self.grapheme_indices(true)
                .skip(1)
                .map(Some)
                .chain(repeat(None)),
        );

        for ((_, grapheme), next_grapheme) in grapheme_indices {
            let next = match next_grapheme {
                Some((next, _)) => next,
                None => return &self,
            };

            let grapheme_char_count = grapheme.chars().count();

            chars = match chars.checked_sub(grapheme_char_count) {
                Some(chars) => {
                    if !grapheme.chars().next().unwrap().is_whitespace() {
                        boundary = next;
                    }

                    chars
                }
                None => break,
            };
        }

        &self[..boundary]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "🤚🏾a🤚🏾 🤚🏾  🤚🏾";

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
        assert_eq!(s.truncate_to_boundary(12), "🤚🏾a🤚🏾 🤚🏾  🤚🏾");
    }

    #[test]
    fn test_non_trucated_string() {
        let s = "🤚🏾a🤚🏾 🤚🏾  🤚🏾";

        assert_eq!(s.truncate_to_boundary(100), s);
        assert_eq!(s.truncate_to_boundary(0), "");
    }
}
