use std::collections::HashMap;

fn main() {
    let nums = [3,2,7,6,5,3,2,7,12,15,1];
    let target = 14;
    let nums = nums.to_vec();

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    
        let len_nums = nums.len();

        let mut num_to_idx: HashMap<i32, usize> = HashMap::with_capacity(len_nums);
        for (idx, num) in nums.into_iter().enumerate() {
            let expected_sum = target - num;
            match num_to_idx.get(&expected_sum) {
                Some(&prev_idx) => {
                    return vec![prev_idx as i32, idx as i32];
                }
                None => {
                    num_to_idx.insert(num, idx);
                }
            }
        }
        unreachable!()
    }
    let result = two_sum(nums, target);
    println!("Index 1: {}\nIndex 2: {}", result[0], result[1]);
}
      
