pub fn binary_search(input: Vec<i32>, target: i32) -> i32 {
    let mut min: usize = 0;
    let mut max: usize = input.len() - 1;
    let mut diff: usize = 0;
    let mut search: usize = 0;
    // [-1, 0, 3, 5, 9, 12]
    loop {
        println!("max : {}", max);
        println!("min : {}", min);
        // println!("input[search] : {}", input[search]);
        diff = (max + min) / 2;
        if diff == 0 {
            if input[max] == target {
                return max as i32;
            } else if input[min] == target {
                return min as i32;
            } else {
                return -1;
            }
        } else {
            if input[diff] > target {
                max = diff - 1;
            } else if input[diff] < target {
                min = diff + 1;
            } else {
                return diff as i32;
            }
            if min > max {
                return -1;
            }
        }
    }
}
