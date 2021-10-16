pub fn random_f64() -> f64 {
    rand::random::<f64>()
}

pub fn random_f64_range(range_min: f64, range_max: f64) -> f64 {
    range_min + (range_max - range_min) * rand::random::<f64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_random_f64() {
        assert!(random_f64() > 0.0);
    }

    #[test]
    fn test_random_f64_range() {
        let number = random_f64_range(0.0, 0.1);
        assert!(number >= 0.0 && number < 0.1);
    }
}
