use std::collections::HashSet;

macro_rules! tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, contains_duplicate(input));
        }
    )*
    }
}

tests! {
    ex1: (vec![1,2,3,1], true),
    ex2: (vec![1,2,4,3], false),
    ex3: (vec![1], false),
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let num_len = nums.len();
    let s: HashSet<i32> = nums.into_iter().collect();
    s.len() != num_len
}

fn main() {
    assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(contains_duplicate(vec![1, 2, 4, 3]), false);
    assert_eq!(contains_duplicate(vec![1]), false);
}
