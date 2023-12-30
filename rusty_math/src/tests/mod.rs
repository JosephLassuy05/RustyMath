#[cfg(test)]
mod tests {

    use std::vec;

    use crate::{median, range, round};

    #[test]
    fn test_round() {
        assert_eq!(round(2.33333, 0), 2.0);
        assert_eq!(round(2.55555, 0), 3.0);
        assert_eq!(round(2.33333, 3), 2.333);
        assert_eq!(round(2.55555, 3), 2.556);
    }

    #[test]
    fn test_range() {
        assert_eq!(range(vec![1.3, 1.0, 100.0]), 99.0);
    }

    #[test]
    fn test_median() {
        assert_eq!(median(vec![22.0, 29.0, 34.0, 36.0, 39.0, 41.0]), 35.);
        assert_eq!(median(vec![6.0, 11.0, 7.0, 2.0, 14.0, 9.0, 2.0]), 7.);
    }
}
