// 计数排序
fn radix_sort(nums: &mut [usize]) {
   if nums.len() <= 1 { return; }
   let max_num = match nums.iter().max() {
       Some(&x) => x,
       None => return,
   };

   let radix = nums.len().next_power_of_two();
   println!("radix: {:?}", radix);
   let mut digit = 1;
   while digit <= max_num {
      let index_of = |x| x/digit % radix;
      let mut counter = vec![0; radix];
      for &x in nums.iter() {
        println!("{x}: index_of: {:?}", index_of(x));
        counter[index_of(x)] += 1;
      }
      println!("digit: {digit}, counter: {:?}", counter);
      for i in 1..radix {
        counter[i] += counter[i-1];
      }
      println!("counter1: {:?}", counter);

      for &x in nums.to_owned().iter().rev() {
         let index = index_of(x);
         counter[index] -= 1;
         let nums_pos = counter[index];
         println!("rev-->{x}: index_of: {index}, nums_pos: {nums_pos}");
         nums[nums_pos] = x;
      }
      println!("counter2: {:?}", counter);
      digit *= radix;
   }
} 

fn main() {
    let mut nums = [54,32,99,18,75,31,43,56,21,22];
    radix_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}