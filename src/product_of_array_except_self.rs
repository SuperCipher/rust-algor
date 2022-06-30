pub fn two_pass(input: Vec<i32>) -> Vec<i32> {
    let mut output = vec![1i32];
    for i in 1..input.len() + 1 {
        output.push(input[i - 1] * output[i - 1]);
    }
    let mut product: i32 = 1;
    println!("1. output : {:?} ", output);
    println!("-----------------------------------");

    for i in (0..input.len()).rev() {
        println!("i {}", i);
        println!("input[i] {}", input[i]);
        println!("product {}", product);
        println!("output {:?}", output);
        println!("output[i] {:?}", output[i]);

        output[i] = output[i] * product;
        product *= input[i];
        println!("product {}", product);
        println!("output {:?}", output);
        println!("output[i] {:?}", output[i]);

        println!("------------- ");
    }
    output.pop().unwrap();
    output
}
