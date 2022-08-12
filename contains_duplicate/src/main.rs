use std::collections::HashSet;

struct Solution {}

fn main() {
    if Solution::contains_duplicate_sort(vec![3, 3]) == true {
        println!("true");
    } else {
        println!("false");
    }
}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.iter().collect::<HashSet<_>>().len() != nums.len()
    }

    pub fn contains_duplicate_sort(nums: Vec<i32>) -> bool {
        let mut nums_copy = nums.clone();
        nums_copy.sort();

        for i in 1..nums_copy.len() {
            if nums_copy[i] == nums_copy[i - 1] {
                return true;
            }
        }
        false
    }
}
