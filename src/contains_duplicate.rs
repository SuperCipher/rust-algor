use std::collections::HashMap;

pub fn one_pass_hash_map(input: Vec<i32>) -> bool {
    let mut hash_map = HashMap::<i32, usize>::new();
    let mut is_duplicate = false;
    for i in 0..input.len() {
        match hash_map.insert(input[i], i as usize) {
            Some(key) => {
                is_duplicate = true;
                break;
            }
            None => (),
        };
    }
    is_duplicate
}
