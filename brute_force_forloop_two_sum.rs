let input: Vec<i32> = vec![2, 7, 11, 15];
let target: i32 = 9;

println!("input {:?}", input);
let mut tuple = (0, 0);
for x in 0..input.len() {
    for y in 0..input.len() {
        if (input[x] + input[y]) == target && (x != y) {
            tuple = (x, y);
            println!("tuple {:?}", tuple)
        }
    }
}
let (l_result, r_result) = tuple;
let result: Vec<i32> = [l_result as i32, r_result as i32].to_vec();
// assert!(result == vec![0, 1]);
