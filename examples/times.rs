
#![allow(dead_code, deprecated)]
use chrono::{DateTime, Duration, Utc, Local, FixedOffset, Timelike, Datelike};

fn day_earlier(date_time : DateTime<Utc>) -> Option<DateTime<Utc>>{
    date_time.checked_sub_signed(Duration::days(1))
}
fn main(){
    let now = Utc::now();
    println!("{}", now);
    let almost_three_weeks_from_now = now.checked_add_signed(Duration::weeks(2))
            .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
            .and_then(day_earlier);
    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => println!("Almost three weeks from now overflows!"),   
    }
    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => println!("We can't use chrono to telll the time the Solar System 
                  to complete more than one full orbit around the galactic center!"), 
    }

    println!("{}", "=".repeat(64));
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let china_timezone = FixedOffset::east(8*3600);
    let rio_timezone = FixedOffset::west(2*3600);
    println!("Local time now is {}", local_time);
    println!("Utc time now is {}", utc_time);
    println!("Time in HongKong now is {}", utc_time.with_timezone(&china_timezone));
    println!("Time in Rio de Janeiro now is {}", utc_time.with_timezone(&rio_timezone));


    println!("{}", "=".repeat(64));
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    println!("The current UTC time is {:02}:{:02}:{:02} {}", 
           hour, now.minute(), now.second(), if is_pm {"PM"} else {"AM"});

   println!("And there have been {} seconds since midnight", now.num_seconds_from_midnight());
   let (is_common_era, year)= now.year_ce();
   println!("The current UTC Date is {}-{:02}-{:02}-{} ({})", 
           year, now.month(), now.day(), now.weekday(), if is_common_era {"CE"} else {"BCE"});
   println!("And the common Era began {} days ago", now.num_days_from_ce());
   println!("{}", "=".repeat(64));
}