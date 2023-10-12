use std::collections::HashSet;


fn main(){
    let words = vec!["gin".to_string(),"zen".to_string(),"gig".to_string(),"msg".to_string()];
    println!("{}", unique_morse_representations(words));
}

fn unique_morse_representations(words: Vec<String>) -> i32 {
    let mut hashset = HashSet::new();
    let code = vec![".-".to_string(),"-...".to_string(),"-.-.".to_string(),"-..".to_string(),".".to_string(),"..-.".to_string(),"--.".to_string(),"....".to_string(),"..".to_string(),".---".to_string(),"-.-".to_string(),".-..".to_string(),"--".to_string(),"-.".to_string(),"---".to_string(),".--.".to_string(),"--.-".to_string(),".-.".to_string(),"...".to_string(),"-".to_string(),"..-".to_string(),"...-".to_string(),".--".to_string(),"-..-".to_string(),"-.--".to_string(),"--..".to_string()];
    for ch in words {
        let mut cur_str = "".to_string();
        for c in ch.chars() {
            cur_str.push_str(&code[(c as u8 - 97) as usize]);
        }
        hashset.insert(cur_str);
    }
    return hashset.len() as i32;
}