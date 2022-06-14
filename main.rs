fn main() {
    let input: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;

    // let pair_result: Vec<(i32, i32)> = pair_combi(input.len() as i32);
    // println!("pair_combi {:?}", pair_result);
    println!("input {:?}", input);
    let two_sum_list2 = two_sum(input, target);
    println!("two_sum_list2 {:?}", two_sum_list2);

    let maybe_index: Option<(i32, i32)> = two_sum_list2;
    // let index: usize = match maybe_index {
    //     // The division was valid
    //     Some(x) => x,
    //     // The division was invalid
    //     None => 0,
    // };
    // let (l_result, r_result) = pair_result[index];
    // let result: Vec<i32> = [l_result, r_result].to_vec();
    // assert!(result == [0, 1]);
    // println!("result {:?}", result);
}

fn two_sum(input: Vec<i32>, target: i32) -> Option<(i32, i32)> {
    let n: usize = input.len();
    println!("{:?}", n);
    let maybe_index = (0..(n as i32))
        .flat_map(|x| (0..=x - 1).map(move |y| (y, x))) // pair of possible combination
        .find(|(x, y)| (input[*x as usize] + input[*y as usize]) == target);
    // .find_map(|t| t == target);
    // let index: usize = match maybe_index {
    //     // The division was valid
    //     Some(x) => x,
    //     // The division was invalid
    //     None => 0,
    // };
    return maybe_index;
}
