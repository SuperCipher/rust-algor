fn main() {
    let input: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;

    let pair_result: Vec<(i32, i32)> = pair_combi(input.len() as i32);
    println!("pair_combi {:?}", pair_result);
    println!("input {:?}", input);

    let two_sum_list: Vec<i32> = two_sum_list(pair_result.clone(), input.clone(), target);
    println!("two_sum_list {:?}", two_sum_list);
    let two_sum_list2 = two_sum(input, target);
    println!("two_sum_list2 {:?}", two_sum_list2);

    let maybe_index: Option<usize> = two_sum_list2.into_iter().position(|t| t == target);
    let index: usize = match maybe_index {
        // The division was valid
        Some(x) => x,
        // The division was invalid
        None => 0,
    };
    let (l_result, r_result) = pair_result[index];
    let result: Vec<i32> = [l_result, r_result].to_vec();
    assert!(result == [0, 1]);
    println!("result {:?}", result);
}

fn two_sum_list(list_pair: Vec<(i32, i32)>, input: Vec<i32>, target: i32) -> Vec<i32> {
    let result = list_pair
        .into_iter()
        .map(|(x, y)| input[x as usize] + input[y as usize])
        .collect::<Vec<i32>>();
    return result;
}

fn pair_combi(n: i32) -> Vec<(i32, i32)> {
    let pair_combi = (0..n)
        .flat_map(|x| (0..=x - 1).map(move |y| (y, x)))
        .collect::<Vec<(i32, i32)>>();
    return pair_combi;
}

fn two_sum(input: Vec<i32>, target: i32) -> Vec<i32> {
    let n: usize = input.len();
    println!("{:?}", n);
    let result = (0..(n as i32))
        .flat_map(|x| (0..=x - 1).map(move |y| (y, x))) // pair of possible combination
        .map(|(x, y)| input[x as usize] + input[y as usize])
        .collect::<Vec<i32>>();
    return result;
}
