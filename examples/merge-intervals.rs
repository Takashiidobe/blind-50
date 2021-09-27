macro_rules! tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, merge_intervals(input));
        }
    )*
    }
}

tests! {
    ex1: (vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]], vec![vec![1,6],vec![8,10],vec![15,18]]),
    ex2: (vec![vec![1,4],vec![4,5]], vec![vec![1,5]]),

}
pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut sorted_intervals = intervals;
    sorted_intervals.sort();
    let mut ans = vec![sorted_intervals[0].clone()];
    for curr in sorted_intervals.into_iter().skip(1) {
        let prev = ans.last().unwrap();
        let ans_len = &ans.len();
        let prev_start = prev[0];
        let prev_end = prev[1];
        let curr_start = curr[0];
        let curr_end = curr[1];
        if prev_end < curr_start {
            ans.push(vec![curr_start, curr_end]);
        } else if prev_end >= curr_start {
            ans[ans_len - 1] = vec![i32::min(prev_start, prev_end), i32::max(prev_end, curr_end)];
        }
    }
    ans
}

fn main() {
    assert_eq!(
        merge_intervals(vec![vec![1, 4], vec![4, 5]]),
        vec![vec![1, 5]]
    );
}
