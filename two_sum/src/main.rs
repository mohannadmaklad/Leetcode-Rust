use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let need = target - num;
        if map.contains_key(&need) {
            vec![map[&need], i as i32]
        }
        map.insert(num, i as i32);
    }
    vec![]
}
