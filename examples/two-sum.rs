macro_rules! two_sum_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, target, expected) = $value;
            assert_eq!(expected, two_sum(input, target));
        }
    )*
    }
}

two_sum_tests! {
    example_0: (vec![2, 7, 11, 15], 9, vec![0, 1]),
    example_1: (vec![3, 2, 4], 6, vec![1, 2]),
    example_2: (vec![3, 3], 6, vec![0, 1]),
}

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        if m.contains_key(num) {
            return vec![m[num], index as i32];
        }
        let loc: i32 = target - num;
        m.insert(loc, index as i32);
    }

    vec![-1, -1]
}

fn main() {
    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", two_sum(vec![3, 2, 4], 6));
    println!("{:?}", two_sum(vec![3, 3], 6));
}
