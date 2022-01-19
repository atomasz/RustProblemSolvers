use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut number_index = HashMap::new();
    let mut result:Vec<i32> = Vec::new();
    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if number_index.contains_key(&complement) {
            result.push(*number_index.get(&complement).unwrap());
            result.push(i as i32);
            return result;
        }
        number_index.insert(num,i as i32);
    }
    result
}
