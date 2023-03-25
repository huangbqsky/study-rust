
// 汉明距离
fn hamming_distance(source: u64, target: u64) -> u32 {
    let mut count = 0;
    let mut xor = source ^ target;
    println!("xor is {xor}");
    while xor != 0 {
        count += xor & 1;
        xor >>= 1;
        println!("count: {count}, xor:{xor}");
    }

    count as u32
}
fn hamming_distance2(source: u64, target: u64) -> u32 {
    (source ^ target).count_ones()
 }

fn hamming_distance_str (source: &str, target: &str) -> u32 {
    let mut count = 0;
    let mut source = source.chars();
    let mut target = target.chars();

    loop {
        match (source.next(), target.next()) {
            (Some(cs),  Some(ct)) if cs != ct => {count += 1},
            (Some(_), None) | (None, Some(_)) => panic!("Must have the same length"),
            (None, None) => break,
            _ => continue,
        }
    }
    count as u32
}


 fn main() { 
    let source = 10;
    let target = 33; 
    let distance = hamming_distance(source, target);
    let distance2 = hamming_distance2(source, target);

    println!("source: {source}, target: {target}, the hamming distance is {distance}");
    println!("source: {source}, target: {target}, the hamming distance2 is {distance2}");


    let source = "abcde";
    let target = "decfk";

    let distance_str = hamming_distance_str(source, target);
    println!("source: {source}, target: {target}, the hamming distance_str is {distance_str}");



}