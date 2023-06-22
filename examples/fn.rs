

fn main() {
    // 误解一：认为没move关键字，就一定是引用捕获
    let s = "hello".to_string();
    let c = || {
        println!("s is {:?}", s);
        s // 有这行后，主动对s的所有权进行了消费转移，就对s进行了所有权捕获；没这行则会变成不可变引用捕获
    };

    // 误解二：认为有了move关键字，所有权捕获型的，就一定是FnOnce
    // 得看捕获的类型，复制语义和移动语义的大不相同：
    let mut x = 5;
    let mut c1 = move || x += 1;  // 有move却是复制捕获，闭包更自由了，能多次调用还是随便Copy，Fn
    let mut c2 = || x += 1; // 没move，默认可变引用捕获，FnMut
    c1();
    c2();

    // 如果move的是一个所有权对象，如String，那也得看闭包行为怎么用这个对象，是只读式的，还是消费掉所有权
    let s = "hello".to_string();
    let c = move || println!("s is {:?}", s); // 虽然是所有权捕获，但依然可以多次调用，Fn
    c();
    c();

    // 误解三：对闭包变量使用mut修饰，就认为是FnMut
    let s = "hello".to_string();
    let mut c = || println!("s is {:?}", s);  // 除了编译器会提供一个多余的mut告警，c还是Fn
    c();
    c();

    // 误解四：认为Fn/FnMut/FnOnce跟Copy/Clone有关系
    let s = "hello".to_string();
    let c1 = move || println!("{s}");  // Fn，但没有实现Copy，因闭包捕获变量存储区有所有权
    c1();

    let mut x = 5;
    let mut c = || x += 1; // FnMut，但没实现Copy，因为可变引用，要满足**可变不共享**规则
    c();

    // 误解五：认为能够满足FnOnce限定的就一定是FnOnce
    // 任何闭包，都可以满足FnOnce限定，因为Fn/FnMut都可以调用多次，更不怕FnOnce只调用一次了。只是以FnOnce调用一次，会丢掉闭包所有权，Fn/FnMut闭包必须是具备Copy/Clone特质的才好说

}