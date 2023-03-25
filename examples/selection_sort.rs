#![allow(dead_code)]

// 选择排序
fn selection_sort(nums: &mut Vec<i32>) {
    let mut left = nums.len() - 1; // 待排序的数据下标
    while left > 0 {
        let mut pos_max = 0;
        for i in 1..=left {
            if nums[i] > nums[pos_max] { 
               pos_max = i; // 选择当前轮次最大的值的下标
            }
        }
        // 数据交换，完成一个数据的排序，待排序数据减1
        nums.swap(left, pos_max);
        left -= 1;
    }

}

fn main() {
    let mut nums = vec![54,32,99,18,75,31,43,56,21,22];
    selection_sort(&mut nums);
    println!("sorted nums: {:?}", nums);

}