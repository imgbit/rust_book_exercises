use std::collections::HashMap;

pub fn median(array: &[i32]) -> Option<f32> {
    if array.is_empty() {
        return None;
    }

    let mut vec = Vec::from(&array[..]);
    vec.sort();

    let length = vec.len();
    let middle = length / 2;

    if length % 2 == 0 {
        let median = (vec[middle - 1] + vec[middle]) / 2;
        Some(median as f32)
    } else {
        Some(vec[middle] as f32)
    }
}

pub fn mode(array: &[i32]) -> Option<i32> {
    if array.is_empty() {
        return None;
    }

    let mut hashmap = HashMap::new();
    // if all items occur just once, by default take the first one
    let mut max_value_cnt = 1;
    let mut max_value = &array[0];

    for num in array {
        let count = hashmap.entry(num).or_insert(0);
        *count += 1;

        if max_value_cnt < *count {
            max_value = &num;
            max_value_cnt = *count;
        }
    }

    Some(*max_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median() {
        assert_eq!(median(&[1, 2, 3, 4, 5]).unwrap(), 3 as f32);
        assert_eq!(median(&[-1, 0, 2, -4, 5, 1, 1]).unwrap(), 1 as f32);
        assert_eq!(median(&[1]).unwrap(), 1 as f32);
        assert_eq!(median(&[-1, 1]).unwrap(), 0 as f32);
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn test_mode() {
        assert_eq!(mode(&[1, 1, 3, 4, 5]).unwrap(), 1);
        assert_eq!(mode(&[1, 2, 3, 4, 5]).unwrap(), 1);
        assert_eq!(mode(&[2, 2, 3, 4, 5]).unwrap(), 2);
        assert_eq!(mode(&[2, 2, 2, 1, 1, 1]).unwrap(), 2);
        assert_eq!(mode(&[2, 2, 2, 1, 1, 1, 1]).unwrap(), 1);
        assert_eq!(mode(&[-1, -1, 3, 4, 5]).unwrap(), -1);
        assert_eq!(mode(&[2]).unwrap(), 2);
        assert_eq!(mode(&[2, 2]).unwrap(), 2);
        assert_eq!(mode(&[]), None);
    }
}
