fn main() {
    let input = "  this   is  a sentence ".to_string();
    println!("{}", reorder_spaces(input));
    let input2 = "a ".to_string();
    println!("{}", reorder_spaces(input2));
}

fn reorder_spaces(text: String) -> String {
    let mut blank_cnt = 0;
    for c in text.chars() {
        if c.is_whitespace() {
            blank_cnt += 1;
        }
    }
    let sp = text.split(" ");
    let mut valid = vec![];
    for t in sp {
        if t.len() == 0 {
            continue;
        }
        valid.push(t);
    }
    if valid.len() == 1 {
        let mut res: String = valid[0].to_owned();
        let mut remains: String = "".to_owned();
        for idx in 0..blank_cnt {
            remains.push_str(&" ");
        }
        res.push_str(&remains);
        return res;
    }
    let each_blank = blank_cnt / (valid.len() - 1);
    let remainder = blank_cnt % (valid.len() - 1);
    let mut res: String = "".to_owned();
    let mut remains: String = "".to_owned();
    for idx in 0..each_blank {
        res.push_str(&" ");
    }
    for idx in 0.. remainder {
        remains.push_str(&" ");
    }

    let mut res2: String = valid.join(&res);
    res2.push_str(&remains);
    return res2;
}