fn main() {
    let input: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;

    println!("input {:?}", input);

    let mut tuple = (0, 0);

    (0..input.len())
        .map(|x| {
            (0..input.len())
                .map(|y| {
                    if (input[x] + input[y]) == target && (x != y) {
                        tuple = (x, y);
                    }
                })
                .collect::<()>();
        })
        .collect::<()>();

    let (l_result, r_result) = tuple;
    let result: Vec<i32> = [l_result as i32, r_result as i32].to_vec();
    println!("result {:?}", result);
}
// leetcode runtime
// Runtime: 108 ms, faster than 5.15% of Rust online submissions for Two Sum.
// Memory Usage: 2.1 MB, less than 99.20% of Rust online submissions for Two Sum.
