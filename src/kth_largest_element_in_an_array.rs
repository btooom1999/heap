use std::collections::BinaryHeap;

fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    BinaryHeap::from(nums).into_iter().nth(k as usize - 1).unwrap()
}

pub fn main() {
    let nums = [3,2,1,5,6,4];
    let k = 2;
    println!("{}", find_kth_largest(nums.into(), k));
}
