fn main() {
    let n = 2126753390;
    println!("{}", first_bad_version(n));
}

fn first_bad_version(n: i32) -> i32 {
    return run(0, n);
}

fn run(start: i32, end: i32) -> i32 {
    if start > end {
        return -1;
    } else if start == end {
        return start;
    }
    let start_quot = start / 2;
    let start_rem = start % 2;
    let end_quot = end / 2;
    let end_rem = end % 2;
    let pivot = start_quot + end_quot + (start_rem + end_rem) / 2;
    if isBadVersion(pivot) {
        return run(start, pivot);
    } else {
        return run(pivot + 1, end);
    }
}

fn isBadVersion(n: i32) -> bool {
    let bad = 1702766719;
    if n < bad {
        return false;
    } else {
        return true;
    }
}