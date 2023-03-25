#![allow(dead_code)]
// 归并排序
fn merge_sort(nums: &mut [i32]) {
   if nums.len() > 1 {
      let mid = nums.len() >> 1;
      println!("merge_sort mid: {mid}, nums: {:?}", nums);
      merge_sort(&mut nums[..mid]); // 排序前半部分
      merge_sort(&mut nums[mid..]); // 排序后半部分
      merge(nums, mid); // 合并排序结果
   }
}

fn merge(nums: &mut [i32], mid: usize) {
    let mut i = 0; //标记前半部分数据
    let mut k = mid; // 标记后半部分数据
    let mut temp = Vec::new();

    for _j in 0..nums.len() {
        if k == nums.len() || i == mid {
            break;
        }

        if nums[i] < nums[k] {
            temp.push(nums[i]);
            i += 1;
        } else {
            temp.push(nums[k]);
            k += 1;
        }
    }

    // 合 并 的 两 部 分 数 据 长 度 大 概 率 不 一 样 长
    // 所 以 要 将 未 处 理 完 集 合 的 数 据 全 部 加 入
    if i < mid && k == nums.len() {
        for j in i..mid {
            temp.push(nums[j]);
        }
    } else if i == mid && k < nums.len() {
        for j in k..nums.len() {
            temp.push(nums[j]);
        }
    }

    // temp 数据放回 nums， 完成排序
    for j in 0..nums.len() {
        nums[j] = temp[j];
    }
    println!("merge mid: {mid}, Vec: {:?}", temp);
}

fn main() { 
    let mut nums = [54,32,99,22,18,75,31,43,56,21];
    merge_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}