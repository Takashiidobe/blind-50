#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), (0, 1));
    }

    #[test]
    fn test_2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), (1, 2));
    }

    #[test]
    fn test_3() {
        assert_eq!(two_sum(vec![3, 3], 6), (0, 1));
    }
}
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> (i32, i32) {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        if m.contains_key(num) {
            return (m[num], index as i32);
        }
        let loc: i32 = target - num;
        m.insert(loc, index as i32);
    }

    (-1, -1)
}
