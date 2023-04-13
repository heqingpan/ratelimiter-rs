
use std::thread;
use std::time::Duration;
use ratelimiter_rs::{QpsLimiter,RateLimiter,now_millis};

fn main() {
    println!("----- ratelimiter -----");
    ratelimiter();
    println!("----- ratelimiter -----");
    qpslimiter();
}

fn ratelimiter(){
    let mut limiter = RateLimiter::new();
    let mut times = 0;
    for _ in 0..3000 {
        thread::sleep(Duration::from_millis(1));
        if limiter.acquire(10,10) {
            times +=1;
            println!("time: {}",now_millis())
        }
        else{
            continue;
        }
    }
    println!("time: {}, times: {}",now_millis(),&times);
}

fn qpslimiter(){
    let mut limiter = QpsLimiter::new(10);
    let mut times = 0;
    for _ in 0..3000 {
        thread::sleep(Duration::from_millis(1));
        if limiter.acquire() {
            times +=1;
            println!("time: {}",now_millis())
        }
        else{
            continue;
        }
    }
    println!("time: {}, times: {}",now_millis(),&times);
}