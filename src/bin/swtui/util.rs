use std::cmp::min;

/// Get the minimum and maximum x-coordinates to center a text within the width
/// of a window.
///
/// For example, for a window where x-coordinates 0 to 4 (inclusive) are
/// free to use, a string of length 5 can use coordinates 0 to 4 (inclusive).
///
/// ```
/// let span = center_text(5, (0, 4));
/// assert_eq!(span, Some((0, 4)));
/// ```
///
/// The x-coordinates returned will never exceed the width of the window.
///
/// ```
/// let span = center_text(100, (10, 49));
/// assert_eq!(span, Some((10, 49)));
/// ```
///
/// Returns [`None`] if an invalid case is encountered
/// (like when right_border < left_border).
pub fn center_text(
    text_len: usize,
    (left_border, right_border): (usize, usize)
) -> Option<(usize, usize)> {
    if right_border < left_border {
        return None;
    }
    let mid_x = (right_border+left_border) / 2;
    if text_len <= 1 {
        return Some((mid_x, mid_x));
    }
    let text_left_len = text_len / 2;
    let text_right_len = text_len - text_left_len - 1;
    let min_x = mid_x - min(mid_x-left_border, text_left_len);
    let max_x = mid_x + min(right_border-mid_x, text_right_len);
    Some((min_x, max_x))
}

#[cfg(test)]
mod tests {
    use crate::util::center_text;

    #[test]
    fn test_center_text() {
        assert_eq!(center_text(5, (0, 4)), Some((0, 4)));
        assert_eq!(center_text(0, (0, 4)), Some((2, 2)));
        assert_eq!(center_text(4, (0, 4)), Some((0, 3)));
        assert_eq!(center_text(10, (10, 29)), Some((14, 23)));
        assert_eq!(center_text(100, (0, 4)), Some((0, 4)));
        assert_eq!(center_text(6, (0, 4)), Some((0, 4)));
    }
}
