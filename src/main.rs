pub mod hashmap_forloop_two_sum;
fn main() {
    let input: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;

    println!("input {:?}", input);

    let result = hashmap_forloop_two_sum::hash_map_forloop(input, target);
    assert!(result == vec![0, 1]);

    println!("result {:?}", result);
}
