use std::collections::HashMap;

pub fn print_all_people_in_company(department_map: &HashMap<String, &str>) {
    println!();
    let mut people_list: Vec<_> = department_map
        .iter()
        .map(|(k, v)| (k.as_str(), *v))
        .collect();

    people_list.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
    println!("{:?}", people_list);
}

pub fn print_people_in_department(department_map: &HashMap<String, &str>, department: &str) {
    println!();
    department_map
        .iter()
        .filter(|(_, v)| **v == department)
        .for_each(|(k, v)| println!("{}: {}", k, v));
}
