use std::collections::HashMap;
pub fn hash_map_forloop(input: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map = HashMap::<i32, usize>::new();

    let mut tuple = (0, 0);
    for x in 0..input.len() {
        hash_map.insert(input[x], x as usize);
    }
    for y in 0..input.len() {
        let complement = target - input[y];
        match hash_map.get(&complement) {
            Some(key) if *key != y => tuple = (*key, y),
            _ => (),
            // without guard seems to be faster
            // Some(key) => {
            //     if *key != y {
            //         tuple = (*key, y);
            //     }
            // }
            // None => (),
        };
    }

    let (l_result, r_result) = tuple;
    let result: Vec<i32> = [l_result as i32, r_result as i32].to_vec();
    return result;
}
// leetcode runtime
// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Two Sum.
// Memory Usage: 2.7 MB, less than 7.38% of Rust online submissions for Two Sum.
