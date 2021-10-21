pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![1; nums.len()];

    (0..nums.len()).fold(1, |mut sum, i| {
        ret[i] = sum;
        sum *= nums[i];
        sum
    });

    (0..nums.len()).rev().fold(1, |mut sum, i| {
        ret[i] *= sum;
        sum *= nums[i];
        sum
    });

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6])
    }
}
