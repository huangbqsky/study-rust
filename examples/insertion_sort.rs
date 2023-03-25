#![allow(dead_code)]

// 插入排序
fn insertion_sort(nums: &mut [i32]){
    for i in 1..nums.len() {
        let mut pos = i;
        let curr = nums[i];
        while pos > 0 && curr < nums[pos - 1] {
            nums[pos] = nums[pos-1];
            pos -= 1;
        }
        nums[pos] = curr; // 插入数据
    }
}
fn main() { 
    let mut nums = [54,32,99,18,75,31,43,56,21];
    insertion_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}