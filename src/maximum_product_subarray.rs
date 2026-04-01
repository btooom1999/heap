fn max_product(nums: Vec<i32>) -> i32 {
    let mut res = *nums.iter().max().unwrap();

    let mut cur_max = 1;
    let mut cur_min = 1;
    for num in nums {
        if num == 0 {
            cur_max = 1;
            cur_min = 1;
            continue;
        }

        let temp = num * cur_max;
        cur_max = (num * cur_max).max(num * cur_min).max(num);
        cur_min = temp.min(num * cur_min).min(num);
        res = res.max(cur_max);
    }

    res
}

pub fn main() {
    let nums = [2,3,-2,4];
    println!("{:?}", max_product(nums.into()));
}
