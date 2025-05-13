fn main() {
    let mut strs: [String; 2] = [String::from("first"), String::from("apple")];

    for i in 0..strs.len() {
        let first = strs[i].chars().next();

        match first {
            Some('a') => strs[i].push_str("-hay"),
            Some('e') => strs[i].push_str("-hay"),
            Some('o') => strs[i].push_str("-hay"),
            Some('i') => strs[i].push_str("-hay"),
            Some('u') => strs[i].push_str("-hay"),
            _ => {
                let cut_char = strs[i].remove(0);
                let new_str = format!("-{cut_char}ay");
                strs[i].push_str(&new_str);
            }
        }
    }

    println!("{:?}", strs)
}
