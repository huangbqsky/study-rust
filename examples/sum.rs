
fn sum_of_val(nums: &[i32], num: i32) -> i32 {
   let mut sum: i32 = 0;
   for n in nums {
      sum += n;
   }
   sum + num
}

// 优化版
fn sum_of_val2(nums: &[i32], num: i32) -> i32 {
    //  nums.iter().sum::<i32> + num
     let sum: i32 = nums.iter().sum();
     sum + num
}

fn sum_of_val3(nums: &[i32], num: i32) -> i32 {
     nums.iter().sum::<i32>() + num
}

fn main() {
    let num = 10;
    let nums = [1, 2, 3, 4, 5, 6, 7, 8];
    let sum = sum_of_val(&nums, num);
    println!("sum is {sum}");
    println!("sum is {}", sum_of_val2(&nums, num));
    println!("sum is {}", sum_of_val3(&nums, num));
}