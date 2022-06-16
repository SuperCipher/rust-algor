fn main() {
    let input: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;

    println!("input {:?}", input);
    let two_sum_list2 = two_sum(input, target);
    println!("two_sum_list2 {:?}", two_sum_list2);

    let maybe_index: Option<(i32, i32)> = two_sum_list2;
    let index_pair: (i32, i32) = match maybe_index {
        // The division was valid
        Some(tuple) => tuple,
        // The division was invalid
        None => (0, 0),
    };
    let (l_result, r_result) = index_pair;
    let result: Vec<i32> = [l_result, r_result].to_vec();
    assert!(result == vec![0, 1]);
}

fn two_sum(input: Vec<i32>, target: i32) -> Option<(i32, i32)> {
    let n: usize = input.len();
    println!("{:?}", n);
    let maybe_index = (0..(n as i32))
        .flat_map(|x| (0..=x - 1).map(move |y| (y, x))) // pair of possible combination
        .find(|(x, y)| (input[*x as usize] + input[*y as usize]) == target);
    return maybe_index;
}
// leetcode runtime
// Runtime: 103 ms, faster than 5.15% of Rust online submissions for Two Sum.
// Memory Usage: 2.4 MB, less than 31.82% of Rust online submissions for Two Sum.
