macro_rules! tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, fib(input));
        }
    )*
    }
}

tests! {
    ex1: (vec![1,2,3,4], vec![24,12,8,6]),
    ex2: (vec![-1,1,0,-3,3], vec![0,0,9,0,0]),
}

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

fn main() {
    println!("{:?}", product_except_self(vec![1, 2, 3, 4]));
    println!("{:?}", product_except_self(vec![-1, 1, 0, -3, 3]));
}
