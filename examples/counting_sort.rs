
// 计数排序
fn counting_sort(nums: &mut [usize]) {
    if nums.len() <= 1 { return; }
    let max_bkt_num = nums.iter().max().unwrap() + 1;
    let mut counter = vec!(0; max_bkt_num);
    for &v in nums.iter() {
        counter[v] += 1;
    }
    println!("max_bkt_num: {max_bkt_num}\ncounter: {:?}", counter);
    let mut j = 0;
    for i in 0..max_bkt_num {
        while counter[i] > 0 {
            nums[j] = i;
            println!("{i}, {j} : {:?}",  nums[j]);
            counter[i] -= 1;
            j += 1;
        }
    }
}


fn main() { 
    let mut nums = [54,32,99,18,75,31,43,56,21,22];
    counting_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}