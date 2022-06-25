// pub mod hashmap_forloop_two_sum;
//
pub mod buysell_stock;
pub mod contains_duplicate;

fn main() {
    // let input_two_sum: Vec<i32> = vec![2, 7, 11, 15];
    // let target: i32 = 9;
    // let result = hashmap_forloop_two_sum::hash_map_forloop(input_two_sum, target);
    // assert!(result == vec![0, 1]);

    // let input_buysell_stock: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
    // let input_buysell_stock2: Vec<i32> = vec![2, 4, 1];
    // let input_buysell_stock3: Vec<i32> = vec![3, 2, 6, 5, 0, 3];
    //
    // println!("input_buysell_stock {:?}", input_buysell_stock);
    // let result = buysell_stock::one_pass2(input_buysell_stock);
    // assert!(result == 5);
    // println!("input_buysell_stock2 {:?}", input_buysell_stock2);
    // let result = buysell_stock::one_pass2(input_buysell_stock2);
    // assert!(result == 2);
    // println!("input_buysell_stock3 {:?}", input_buysell_stock3);
    //
    // let result = buysell_stock::one_pass2(input_buysell_stock3);
    // assert!(result == 4);

    let contains_duplicate: Vec<i32> = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
    let result = contains_duplicate::one_pass_hash_map(contains_duplicate);

    println!("result {:?}", result);
}
