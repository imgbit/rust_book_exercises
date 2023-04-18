pub fn fahrenheit_to_celsius(f : f64) -> f64 {
    (f - 32.0) * 5.0/9.0
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
}