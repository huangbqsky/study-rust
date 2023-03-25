
// 二分查找
fn binary_search1(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;
    while low <= high && !found {
        let mid: usize = (low + high) >> 1;
        println!("low: {low}, high: {high} , mid: {mid}");
        if num == nums[mid] {
            found = true;
        } else if num < nums[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    found
}
// 递归查找
fn binary_search2(nums: &[i32], num: i32) -> bool {
    if 0 == nums.len() { return false;}
    let len = nums.len();
    let mid: usize = len >> 1;
    println!("len: {len}, mid: {mid}");
    if num == nums[mid] {
        return true;
    } else if num < nums[mid] {
        return binary_search2(&nums[..mid], num);
    } else {
        return binary_search2(&nums[mid+1..], num);
    }
}
// 内插查找 
fn interpolation_search(nums: &[i32], target: i32) -> bool {
    if nums.is_empty() { return false; }

    let mut high = nums.len() - 1;
    let mut low  = 0usize;
    loop {
        let low_val  = nums[low];
        let high_val = nums[high];
        if high <= low || target < low_val || target > high_val {
            break;
        }

        // 计算插值位置
        let offset = (target - low_val)*(high - low) as i32 / (high_val - low_val);
        let interpolant = low + offset as usize;

        // 更新上下界 high、low
        if nums[interpolant] > target {
            high = interpolant - 1;
        } else if nums[interpolant] < target {
            low = interpolant + 1;
        } else {
            break;
        }
    }

    // 判断最终确定的上界处是否是 target
    if target == nums[high] {
        true
    } else {
        false
    }
}

// 指数查找
fn exponential_search(nums: &[i32], target: i32) -> bool {
    let size = nums.len();
    if size == 0 { return false; }
    let mut high = 1usize;
    while high < size && nums[high] < target {
        high <<= 1;
        println!("exponential_search-->size: {size}, high: {high}");
    }

    let low = high >> 1;
    println!("exponential_search-->size: {size}, low: {low}");
    // 使用前面的二分查找
    binary_search1(&nums[low..size.min(high +1)], target)
}

fn main() {
    let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 60, 62, 64];
    let target = 3;
    let found = binary_search1(&nums, target);
    println!("{target}, is in nums: {found}");


    let num = 63;
    let found = binary_search2(&nums, num);
    println!("{num} is in nums: {found}");

    let nums = [1,9,10,15,16,17,19,23,27,28,29,30,32,35];
    let target = 9;
    let found = interpolation_search(&nums, target);
    println!("{target} in nums: {found}");


    let nums = [1,9,10,15,16,17,19,23,27,28,29,30,32,35];
    let target = 10;
    let found = exponential_search(&nums, target);
     println!("exponential_search--> {target} is in nums: {found}");
}