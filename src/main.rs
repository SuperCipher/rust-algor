// pub mod hashmap_forloop_two_sum;
//
// pub mod buysell_stock;
// pub mod contains_duplicate;
pub mod product_of_array_except_self;
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

    // let contains_duplicate: Vec<i32> = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
    // let result = contains_duplicate::one_pass_hash_map(contains_duplicate);
    let product_of_array_except_self: Vec<i32> = vec![-1, 1, 0, -3, 3];
    let product_of_array_except_self2: Vec<i32> = vec![1, 2, 3, 4];

    println!("input {:?}", product_of_array_except_self2);
    let result = product_of_array_except_self::two_pass(product_of_array_except_self2);
    println!("result {:?}", result);

    assert!(result == [24, 12, 8, 6]);

    let result = product_of_array_except_self::two_pass(product_of_array_except_self);
    println!("result {:?}", result);

    assert!(result == [0, 0, 9, 0, 0]);
}
