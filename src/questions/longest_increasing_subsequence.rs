macro_rules! tests {
    ($($name:ident: $value:expr,)*) => {
    #[cfg(test)]
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, longest_increasing_subsequence(input));
        }
    )*
    }
}

tests! {
    ex1: (vec![10,9,2,5,3,7,101,18], 4),
    ex2: (vec![0,1,0,3,2,3], 4),
    ex3: (vec![7,7,7,7,7,7,7], 1),
}

pub fn longest_increasing_subsequence(nums: Vec<i32>) -> i32 {
    let mut table: Vec<i32> = vec![1; nums.len()];

    for i in 1..nums.len() {
        let mut longest = 0;
        for j in 0..i {
            if nums[j] < nums[i] {
                longest = longest.max(table[j]);
            }
        }
        table[i] = longest + 1;
    }

    *table.iter().max().unwrap()
}
