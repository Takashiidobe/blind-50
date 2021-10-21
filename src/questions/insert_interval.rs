use std::cmp::{max, min};

macro_rules! tests {
    ($($name:ident: $value:expr,)*) => {
    #[cfg(test)]
    $(
        #[test]
        fn $name() {
            let (interval, new_interval, expected) = $value;
            assert_eq!(expected, insert_interval(interval, new_interval));
        }
    )*
    }
}

tests! {
    ex1: (vec![vec![1, 3], vec![6, 9]], vec![2, 5], vec![vec![1, 5], vec![6, 9]]),
    ex2: (vec![vec![1, 2], vec![6, 9]], vec![2, 5], vec![vec![1, 5], vec![6, 9]]),
}

pub fn insert_interval(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut left = vec![];
    let mut right = vec![];
    let mut start = new_interval[0];
    let mut end = new_interval[1];

    for curr in intervals {
        if curr[1] < new_interval[0] {
            left.push(curr);
        } else if curr[0] > new_interval[1] {
            right.push(curr);
        } else {
            start = min(curr[0], start);
            end = max(curr[1], end);
        }
    }
    left.push(vec![start, end]);
    left.extend(right);
    left
}
