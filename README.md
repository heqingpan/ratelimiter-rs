
# ratelimiter-rs

A lite ratelimit utils for rust.

## examples

```rust
use std::thread;
use std::time::Duration;
use ratelimiter_rs::{QpsLimiter,RateLimiter,AtomicRateLimiter,now_millis};

fn qpslimiter(){
    let mut limiter = QpsLimiter::new(10);
    // AtomicRateLimiter can clone to other thread and use
    //let limiter = AtomicRateLimiter::new(10);
    let mut times = 0;
    for _ in 0..3000 {
        thread::sleep(Duration::from_millis(1));
        if limiter.acquire() {
            times +=1;
            //println!("time: {}",now_millis())
        }
        else{
            continue;
        }
    }
    println!("time: {}, times: {}",now_millis(),&times);
}
```
