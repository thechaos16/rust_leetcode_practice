fn main() {
    let g = vec![1, 2, 3];
    let s = vec![1, 2,3];
    println!("{}", find_content_children(g, s));
}

fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    if g.len() == 0 || s.len() == 0 {
        return 0;
    }
    let mut child = 0;
    let mut g_sort = g;
    let mut s_sort = s;
    g_sort.sort_by(|a, b| b.cmp(a));
    s_sort.sort_by(|a, b| b.cmp(a));
    let mut s_idx = 0;
    for idx in 0..g_sort.len(){
        if s_idx >= s_sort.len() {
            break;
        }
        if g_sort[idx] <= s_sort[s_idx] {
            child += 1;
            s_idx += 1;
        }
    }
    return child;
}