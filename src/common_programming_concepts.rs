pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn nth_fibonacci(f: u32) -> u32 {
    if f == 0 {
        0
    } else if f == 1 {
        1
    } else {
        // use recursion
        nth_fibonacci(f - 1) + nth_fibonacci(f - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(0.0), -17.77777777777778);
        assert_eq!(fahrenheit_to_celsius(68.0), 20.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
        assert_eq!(fahrenheit_to_celsius(-1000.0), -573.3333333333334);
    }

    #[test]
    fn test_nth_fibonacci() {
        assert_eq!(nth_fibonacci(0), 0);
        assert_eq!(nth_fibonacci(1), 1);
        assert_eq!(nth_fibonacci(5), 5);
        assert_eq!(nth_fibonacci(10), 55);
    }
}
