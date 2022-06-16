use std::collections::HashMap;
fn main() {
    let input: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;

    let mut hash_map = HashMap::<i32, usize>::new();

    let mut tuple = (0, 0);
    for x in 0..input.len() {
        hash_map.insert(input[x], x as usize);
    }
    for y in 0..input.len() {
        let complement = target - input[y];
        match hash_map.get(&complement) {
            Some(key) => {
                if *key != y {
                    tuple = (*key, y);
                }
            }
            None => (),
        };
    }

    let (l_result, r_result) = tuple;
    let result: Vec<i32> = [l_result as i32, r_result as i32].to_vec();
    // assert!(result == vec![0, 1]);
}
// leetcode runtime
// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Two Sum.
// Memory Usage: 2.7 MB, less than 7.38% of Rust online submissions for Two Sum.
