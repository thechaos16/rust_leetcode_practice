fn main() {
    let words = vec!["aba".to_string(), "bcb".to_string(), "ece".to_string(), "aa".to_string(), "e".to_string()];
    let queries = vec![vec![0, 2], vec![1, 4], vec![1, 1]];
    println!("{:?}", vowel_strings(words, queries));
}

fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = vec![];
    let mut vowel_cnt = 0;
    let mut cumsum = vec![];
    for word in words {
        let word_len = word.len();
        if vec!['a', 'e', 'i', 'o', 'u'].contains(&word.chars().nth(0).unwrap()) && vec!['a', 'e', 'i', 'o', 'u'].contains(&word.chars().nth(word_len - 1).unwrap()) {
            vowel_cnt += 1;
        }
        cumsum.push(vowel_cnt);
    }
    for query in queries {
        let start;
        if query[0] == 0 {
            start = 0;
        } else {
            start = cumsum[query[0] as usize - 1];
        }
        ans.push(cumsum[query[1] as usize] - start);
    }
    return ans;
}