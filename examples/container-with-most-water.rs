macro_rules! tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, max_area(input));
        }
    )*
    }
}

tests! {
    ex1: (vec![1, 1], 1),
    ex2: (vec![1,2,1], 2),
    ex3: (vec![4,3,2,1,4], 16),
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = height.len() - 1;

    let mut max_so_far = 0;

    while i < j {
        let left = height[i];
        let right = height[j];
        let area = ((j - i) as i32) * i32::min(left, right);
        if left < right {
            max_so_far = i32::max(area, max_so_far);
            i += 1;
        } else {
            max_so_far = i32::max(area, max_so_far);
            j -= 1;
        }
    }

    max_so_far
}

fn main() {
    println!("{:?}", max_area(vec![1, 1]));
}
