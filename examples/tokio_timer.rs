use chrono::Local;
use tokio::{self, time::{self, Duration, Instant, MissedTickBehavior}};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

#[tokio::main]
async fn main() {
    interval_timer().await;
    bursty_behavior_timer().await;
    delay_behavior_timer().await;
    skip_behavior_timer().await;
}
/**
 * 间隔任务: tokio::time::Interval
 * tokio::time::interval()和tokio::time::interval_at()用于设置间隔性的任务。
 */
async fn interval_timer(){
    println!("before: {} for interval_timer", now());

    // 计时器的起始计时点：此时此刻之后的5秒后
    let start = Instant::now() + Duration::from_secs(5);
    // 时间间隔
    let interval = Duration::from_secs(1);
    // tokio::time::interval_at()用于设置间隔性的任务。
    let mut intv = time::interval_at(start, interval);

    // 该计时任务"阻塞"，直到5秒后被唤醒
    intv.tick().await;
    println!("task 1: {}", now());

    // 该计时任务"阻塞"，直到1秒后被唤醒
    intv.tick().await;
    println!("task 2: {}", now());

    // 该计时任务"阻塞"，直到1秒后被唤醒
    intv.tick().await;
    println!("task 3: {}", now());
}

/**
 * Burst策略，冲刺型的计时策略，当出现延迟后，将尽量快地完成接下来的tick，直到某个tick赶上它正常的计时时间点
 */
async fn bursty_behavior_timer(){
    println!("before: {} for bursty_behavior_timer", now());

    let start = Instant::now() + Duration::from_secs(5);
    let interval = Duration::from_secs(1);
    let mut intv = time::interval_at(start, interval);

    time::sleep(Duration::from_secs(10)).await;
    intv.tick().await;
    println!("task 1: {}", now());
    intv.tick().await;
    println!("task 2: {}", now()); 
    intv.tick().await;
    println!("task 3: {}", now());
}
/**
 * Delay策略，延迟性的计时策略，当出现延迟后，仍然按部就班地每隔指定的时长计时。在内部，这种策略是在每次执行tick之后，
 * 都修改下一次计时起点为Instant::now() + Duration。因此，这种策略下的任何相邻两次的tick，其中间间隔的时长都至少达到Duration。
 */
async fn delay_behavior_timer(){
    println!("before: {} for delay_behavior_timer", now());

    let mut intv = time::interval_at(
        Instant::now() + Duration::from_secs(5),
        Duration::from_secs(2),
    );
    intv.set_missed_tick_behavior(MissedTickBehavior::Delay);

    time::sleep(Duration::from_secs(10)).await;

    println!("start: {}", now());
    intv.tick().await;
    println!("tick 1: {}", now());
    intv.tick().await;
    println!("tick 2: {}", now());
    intv.tick().await;
    println!("tick 3: {}", now());  
}

/**
 * Skip策略，忽略型的计时策略，当出现延迟后，仍然所有已经被延迟的计时任务。
 * 这种策略总是以定义计时器时的起点为基准，类似等差数量，每一次执行tick的时间点，一定符合Start + N * Duration。
 */
async fn skip_behavior_timer(){
    println!("before: {} for skip_behavior_timer", now());

    let mut intv = time::interval_at(
        Instant::now() + Duration::from_secs(5),
        Duration::from_secs(2),
    );
    intv.set_missed_tick_behavior(MissedTickBehavior::Skip);

    time::sleep(Duration::from_secs(10)).await;

    println!("start: {}", now());
    intv.tick().await;
    println!("tick 1: {}", now());
    intv.tick().await;
    println!("tick 2: {}", now());
    intv.tick().await;
    println!("tick 3: {}", now()); 
}