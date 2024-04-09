


pub fn sort_zh_test(){

    let mut test = vec!["a", "2", "三", "1", "一", "3", "二", "b"];
    // 在几乎所有的程式语言，对Array 或Vector 进行sort()的做法都是直接拿Unicode 的Hex Code 进行排序
    test.sort();
    // print show ["1", "2", "3", "a", "b", "一", "三", "二"]，看出问题点了吗？为什么「三」排序在「二」之前呢？
    println!("{:?}", test); 

}