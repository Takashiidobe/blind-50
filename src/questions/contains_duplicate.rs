use std::collections::HashSet;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(contains_duplicate(vec![1, 2, 4, 3]), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(contains_duplicate(vec![1]), false);
    }
}

/// Returns `true` if nums contains a duplicate, `false otherwise.`
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let num_len = nums.len();
    let s: HashSet<i32> = nums.into_iter().collect();
    s.len() != num_len
}
