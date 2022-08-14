struct Solution {}

fn main() {
    println!("Hello, world!");
    println!("{:?}", Solution::product_except_self(vec![3, 0, 0]));
}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let zeros = Self::zeros_indices(&nums);
        match zeros.len() {
            0 => {
                let total_product = nums.iter().fold(1, |acc, x| acc * x);
                return nums.iter().map(|x| total_product / x).collect();
            }
            1 => {
                let mut result = vec![0; nums.len()];
                let mut total_product = 1;
                for num in nums {
                    if num != 0 {
                        total_product *= num;
                    }
                }
                result[zeros[0]] = total_product;
                return result;
            }
            _ => {
                return vec![0; nums.len()];
            }
        }
    }

    fn zeros_indices(nums: &Vec<i32>) -> Vec<usize> {
        let mut zeros_count = 0;
        let mut result = vec![];
        for (i, &val) in nums.iter().enumerate() {
            if val == 0 {
                zeros_count += 1;
                if zeros_count > 1 {
                    //Values won't matter!
                    return vec![0, 0];
                }
                result.push(i)
            }
        }
        return result;
    }
}
