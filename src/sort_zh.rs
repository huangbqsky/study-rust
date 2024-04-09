static LOWERCASE_NUM: [char; 25] = [
    '零', '一', '二', '三', '四', '五', '六', '七', '八', '九', '十', '百', '千', '萬', '億', '兆',
    '京', '垓', '秭', '穰', '溝', '澗', '正', '載', '極',
];

static UPPERCASE_NUM: [char; 25] = [
    '零', '壹', '貳', '參', '肆', '伍', '陸', '柒', '捌', '玖', '拾', '佰', '仟', '萬', '億', '兆',
    '京', '垓', '秭', '穰', '溝', '澗', '正', '載', '極',
];


pub fn sort_zh_test(){

    let mut test = vec!["a", "2", "三", "1", "一", "3", "二", "b"];
    // 在几乎所有的程式语言，对Array 或Vector 进行sort()的做法都是直接拿Unicode 的Hex Code 进行排序
    test.sort();
    // print show ["1", "2", "3", "a", "b", "一", "三", "二"]，看出问题点了吗？为什么「三」排序在「二」之前呢？
    println!("{:?}", test); 

    // sort_zh_icu_ucol();// 测试ICU中文排序
    // sort_zh_main();// 测试中文排序

}

// ICU（International Components for Unicode），是 Unicode 社群所開發的函式庫，提供多國語言的函式
// pub fn sort_zh_icu_ucol(){
//    // 先取得collator
//     // 这边的范例是zh-TW（台湾用的繁体字），也可以改成zh-HK、zh-CN等等
//     let collator = UCollator::try_from("zh-CN").unwrap();
//     // 设置用来测试的string slice，放在Vec中
//     let mut test_value = vec!["肆", "1", "一", "2", "二", "叁", "正", "十二测试", "拾贰测试", "贰拾测试", "拾测试二", "十测试二"];
//     // 把测试array，透过较为快速的unstable sort进行排序
//     // 由于我们要利用collator进行排序，所以用sort_unstable_by()这个function
//     test_value.sort_unstable_by(|a_value, b_value| {
//         // 利用rust_icu_ucol提供的function进行比较，成功的话会返回Ordering
//         collator
//             .strcoll_utf8(a_value, b_value)
//             .expect("Failed to collate with collator.")
//     });
//     // 排完后把结果print出来
//     println!("{:?}", test_value);
// }

// fn parse_zh_number(chars: Chars) -> (bool, Result<i64, ChineseNumberParseError>) {
//     // 用於判斷中文數字是否為大寫的變數
//     let mut upper_case = false;
//     // 用於判斷中文數字字元長度的變數
//     let mut zh_number_size = 1_usize;
//     // 將每個字元撈出來，拿去與上一步製作的字典做比對
//     chars.clone().enumerate().for_each(|(i, char)| {
//         // 第一個字元如果是大寫數字，則利用upper_case變數進行記錄
//         if i == 0_usize && UPPERCASE_NUM.contains(&char) {
//             upper_case = true
//         }
//         // 當字元不在字典檔中，計算數字長度並記錄到zh_number_size變數中
//         if !UPPERCASE_NUM.contains(&char) && !LOWERCASE_NUM.contains(&char) {
//             // 由於Rust的index使用usize型別，需要另行轉換才能運用
//             zh_number_size = (i as u32 - 1) as usize;
//         }
//     });
//     // 回傳tuple
//     (
//         upper_case,
//         // 利用chinese-number提供的function，將中文數字轉換為阿拉伯數字並回傳
//         parse_chinese_number_to_i64(
//             ChineseNumberCountMethod::TenThousand,
//             String::from_iter(chars.collect::<Vec<char>>()[0..zh_number_size].iter()),
//         ),
//     )
// }

// // 這邊我們會拿到Vec<(usize, T)>，其中usize是index，而指定 Ord 特性限制 （trait bound）的泛型部分則是需要排序的元素
// // 由於最後進行合併時，會從參數指定的Vec中撈回中文，所以僅需回傳usize的部份即可
// fn sort_number<T: Ord>(mut vec: Vec<(usize, T)>) -> Vec<usize> {
//     // 採用較為快速的unstable sort，因為我們需要從tuple中存取泛型的部分進行排序，所以一樣要用sort_unstable_by()自訂比較
//     vec.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));
//     // 將處理好的Vec unzip，將Vec<(usize, T)>分離成(Vec<usize>, Vec<T>)
//     // 由於我們只需要usize的部分，所以T可以直接忽略
//     let (processed_vec, _): (Vec<usize>, Vec<_>) = vec.into_iter().unzip();
//     // 回傳排序後的index
//     processed_vec
// }

// // 需要同時傳入collator，我們會利用它進行排序
// // 一樣回傳usize即可
// fn sort_zh_word(mut vec: Vec<(usize, &str)>, collator: UCollator) -> Vec<usize> {
//     // 一樣使用unstable sort，針對&str的部分排序
//     vec.sort_unstable_by(|(_, a_value), (_, b_value)| {
//         // 使用傳入的collator，利用rust_icu_ucol提供的function進行比較
//         collator
//             .strcoll_utf8(a_value, b_value)
//             .expect("Failed to collate with collator.")
//     });
//     let (processed_vec, _): (Vec<usize>, Vec<_>) = vec.into_iter().unzip();
//     // 回傳排序後的index
//     processed_vec
// }

// fn sort_zh_main() {
//     // 先取得collator
//     // 這邊的範例是zh-TW（台灣用的繁體字），也可以改成zh-HK、zh-CN等等
//     let collator = UCollator::try_from("zh-TW").unwrap();
//     // 設定用來測試的string slice，放在Vec中
//     let mut test_value = vec!["肆", "1", "一", "2", "二", "參", "正", "十二測試", "拾貳測試", "貳拾測試", "拾測試二", "十測試二"];

//     // 將等等會用到的四個小Vec都先開好
//     let mut ascii_word_vec: Vec<(usize, &str)> = Vec::new();
//     let mut zh_upper_number_vec: Vec<(usize, i64)> = Vec::new();
//     let mut zh_lower_number_vec: Vec<(usize, i64)> = Vec::new();
//     let mut zh_word_vec: Vec<(usize, &str)> = Vec::new();

//     // 對測試值進行iterate，並將index透過enumerate()一併foreach出來
//     test_value.iter().enumerate().for_each(|(i, element)| {
//         // 將文字轉換為Chars
//         let chars = element.chars();
//         // 透過peek取得下一筆資料，並確認是否為ASCII字元
//         if chars.clone().peekable().peek().unwrap().is_ascii() {
//             // 是則推入ascii_word_vec中
//             ascii_word_vec.push((i, element))
//         } else {
//             // 透過第二步製作的parse_zh_number()解析中文數字
//             match parse_zh_number(chars.clone()) {
//                 // 這邊僅對轉換結果做判斷
//                 (upper_case, Ok(parsed)) => {
//                     if !upper_case {
//                         // 如果不是大寫數字，推入zh_lower_number_vec中
//                         zh_lower_number_vec.push((i, parsed))
                        
//                     } else if upper_case {
//                         // 如果是大寫數字，推入zh_upper_number_vec中
//                         zh_upper_number_vec.push((i, parsed))
//                     } else {
//                         // 都不是，視為文字，推入zh_word_vec中
//                         zh_word_vec.push((i, element))
//                     }
//                 }
//                 // 轉換失敗，視為文字，推入zh_word_vec中
//                 (_, Err(_)) => zh_word_vec.push((i, element)),
//             }
//         }
//     });

//     // 將ASCII排序
//     let mut final_vec = sort_number(ascii_word_vec);
//     // 將中文小寫數字排序
//     final_vec.append(&mut sort_number(zh_lower_number_vec));
//     // 將中文大寫數字排序
//     final_vec.append(&mut sort_number(zh_upper_number_vec));
//     // 將中文文字排序
//     final_vec.append(&mut sort_zh_word(zh_word_vec, collator));

//     // 將組合好的index從test_value取回原值，並組合成新的test_value
//     test_value = final_vec
//         .into_iter()
//         .map(|i| test_value[i])
//         .collect::<Vec<&str>>();

//     // 組合後把結果print出來
//     println!("{:?}", test_value);
// }