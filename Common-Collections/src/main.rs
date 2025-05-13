use std::collections::HashMap;

fn main() {
    let mut list: Vec<i32> = vec![1, 2, 10, 12, 21, 30, 30, 100];

    list.sort();

    let median_index = list.len() / 2;

    println!("Median: {}", list[median_index]);

    let mut count_hashmap = HashMap::new();

    for num in list {
        let count = count_hashmap.entry(num).or_insert(0);
        *count += 1;
    }

    let mode_number = count_hashmap.iter().max_by_key(|entry| entry.1).unwrap();

    println!("Mode: {}", mode_number.0);
}
