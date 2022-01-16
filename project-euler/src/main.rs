use std::collections::HashMap;

mod euler_one;
mod euler_two;

fn main() {
    euler_one::project_euler_question_one();
    euler_two::project_euler_question_two();
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut numberIndex = HashMap::new();
        let mut result:Vec<i32> = Vec::new();
        for i in 0..nums.len(){
            let num = nums[i];
            let complement = target - num;
            if numberIndex.contains_key(&complement)
            {
                result.push(*numberIndex.get(&complement).unwrap());
                result.push(i as i32);
                return result;
            }
            numberIndex.insert(num,i as i32);
        }
	    result
}
