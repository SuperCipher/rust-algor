pub fn one_pass_over_engineer(input: Vec<i32>) -> i32 {
    let mut min_index: usize = 0;
    let mut max_index: usize = 0;

    let mut min_prev_index: usize = 0;
    let mut max_prev_index: usize = 0;

    for elem in 0..input.len() {
        let is_lowest = input[elem] < input[min_index];
        let is_highest = input[elem] > input[max_index];
        println!("min_index {}", min_index);
        println!("max_index {}", max_index);
        println!("input[min_index] : {}", input[min_index]);
        println!("input[max_index] {}", input[max_index]);
        println!("input[elem] {}", input[elem]);
        println!("min_prev_index {}", min_prev_index);

        println!("-----------");

        if is_lowest {
            min_index = elem;
            max_index = elem;
        } else {
            if is_highest {
                max_index = elem;
            }
        }
        if (input[max_index] - input[min_index]) > (input[max_prev_index] - input[min_prev_index]) {
            min_prev_index = min_index;
            max_prev_index = max_index;
        }
    }
    println!(">>>>>>>>>");

    println!("min_prev_index {}", min_prev_index);
    println!("max_prev_index {}", max_prev_index);
    println!("min_index {}", min_index);
    println!("max_index {}", max_index);
    if (input[max_prev_index] - input[min_prev_index]) > (input[max_index] - input[min_index]) {
        input[max_prev_index] - input[min_prev_index]
    } else {
        input[max_index] - input[min_index]
    }
}

pub fn one_pass2(input: Vec<i32>) -> i32 {
    let mut min_price: i32 = i32::MAX;
    let mut max_profit: i32 = 0;

    for elem in 0..input.len() {
        if input[elem] < min_price {
            min_price = input[elem]
        } else if input[elem] - min_price > max_profit {
            max_profit = input[elem] - min_price
        }
    }
    max_profit
}
