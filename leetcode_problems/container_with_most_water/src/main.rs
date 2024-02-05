use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut right:i32 = <usize as TryInto<i32>>::try_into(height.len()).unwrap()-1;
    let mut left:i32 = 0;
    let mut max_water = 0;
    while left < right {
        let diff = right - left;
        let left_value = height[left as usize];
        let right_value = height[right as usize];
        let new_value = cmp::min(left_value, right_value) * diff;
        max_water = cmp::max(new_value, max_water);
        if left_value < right_value {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max_water
}

fn main() {
    let height:Vec<i32> = vec![1,8,6,2,5,4,8,3,7];
    let result = max_area(height);
    println!("The most water this container can hold is: {}L", result);
}