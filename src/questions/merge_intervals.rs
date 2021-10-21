/// This function finds overlapping intervals in the intervals vector and merges them so no intervals are overlapping.
pub fn merge_intervals(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut sorted_intervals = intervals;
    sorted_intervals.sort();
    let mut ans = vec![sorted_intervals[0].clone()];
    for curr in sorted_intervals.into_iter().skip(1) {
        let prev = ans.last().unwrap();
        let ans_len = &ans.len();
        let (prev_start, prev_end) = prev;
        let (curr_start, curr_end) = curr;
        if prev_end < &curr_start {
            ans.push((curr_start, curr_end));
        } else if prev_end >= &curr_start {
            ans[ans_len - 1] = (
                i32::min(*prev_start, *prev_end),
                i32::max(*prev_end, curr_end),
            );
        }
    }
    ans
}
