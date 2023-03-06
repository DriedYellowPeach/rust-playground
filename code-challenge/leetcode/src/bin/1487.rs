
fn main() {
    unimplemented!();
}

use std::collections::HashMap;
fn get_folder_names(names: Vec<String>) -> Vec<String> {
    let mut map = HashMap::new();
    names.into_iter().map(|name| {
        get_valid_name(&mut map, &name)
    }).collect()
}

fn get_valid_name(map: &mut HashMap<String, i32>, name: &str) -> String {
    let mut idx = match map.get(name) {
        Some(v) => *v,
        None => {
            map.insert(name.to_string(), 1);
            return name.to_string();
        }
    };

    let mut new_name = format!("{name}({idx})");
    while map.get(&new_name).is_some() {
        idx += 1;
        new_name = format!("{name}({idx})");
    }
    *map.get_mut(name).unwrap() = idx + 1;
    map.insert(new_name.clone(), 1);

    new_name
}

#[test]
fn test_get_folder_names() {
    let names = vec!["zelda".to_string(), "zelda".to_string(), "zelda(1)".to_string(), "zelda(2)".to_string(), "zelda".to_string()];
    println!("{:?}", get_folder_names(names));
}
