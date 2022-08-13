struct Solution {}

fn main() {
    println!("Hello, world!");
    println!(
        "{:?}",
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = -10000;
        let mut current_sum = 0;

        for n in nums {
            current_sum += n;
            max_sum = max_sum.max(current_sum);
            current_sum = current_sum.max(0);
        }
        max_sum
    }
}
