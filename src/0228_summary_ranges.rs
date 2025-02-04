fn main() {
    let nums = vec![0, 1, 2, 4, 5, 7];
    println!("{:?}", summary_ranges(nums));
}

fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.len() == 0 {
        return vec![];
    }
    let mut ans = vec![];
    let mut iter = nums.into_iter().peekable();
    if let Some(mut start) = iter.next() {
        let mut end = start;
        while let Some(&next) = iter.peek() {
            if next == end + 1 {
                end = next;
                iter.next();
            } else {
                ans.push(format_range(start, end));
                start = next;
                end = next;
                iter.next();
            }
        }
        ans.push(format_range(start, end));
    }
    return ans;
}

fn format_range(start: i32, end: i32) -> String {
    if start == end {
        return start.to_string();
    } else {
        return format!("{}->{}", start, end);
    }
}