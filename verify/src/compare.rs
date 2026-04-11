/// Relative error metric per VER-04: |x-y| / (1 + max(|x|, |y|))
///
/// This metric avoids division-by-zero when both values are near zero while
/// providing a meaningful relative measure for non-zero values. Matches the
/// formula in the design document Section 16.2.
pub fn relative_error(computed: f64, reference: f64) -> f64 {
    (computed - reference).abs() / (1.0_f64 + computed.abs().max(reference.abs()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_error_identical_values() {
        assert_eq!(relative_error(1.0, 1.0), 0.0);
    }

    #[test]
    fn relative_error_both_zero() {
        assert_eq!(relative_error(0.0, 0.0), 0.0);
    }

    #[test]
    fn relative_error_one_zero() {
        // |0 - 1| / (1 + max(0, 1)) = 1/2 = 0.5
        assert_eq!(relative_error(0.0, 1.0), 0.5);
    }

    #[test]
    fn relative_error_small_difference() {
        let err = relative_error(1.0 + 1e-14, 1.0);
        assert!(err < 1e-13, "expected small error, got {err}");
        assert!(err > 0.0, "expected nonzero error");
    }

    #[test]
    fn relative_error_symmetric() {
        let a = 1.234;
        let b = 1.235;
        assert_eq!(relative_error(a, b), relative_error(b, a));
    }

    #[test]
    fn relative_error_negative_values() {
        // Should work with negative numbers
        let err = relative_error(-1.0, -1.0);
        assert_eq!(err, 0.0);
    }
}
