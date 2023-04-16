use std::cmp;


pub fn now_millis() -> u128 {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub enum RateUnit {
    Seconds,
    Minutes,
}

impl RateUnit {
    pub(crate) fn get_rate_to_conversion(&self) -> i32 {
        match self {
            RateUnit::Seconds => 1000,
            RateUnit::Minutes => 60*1000,
        }
    }
}

#[derive(Debug,Clone,Default)]
pub struct RateLimiter {
    pub(crate) rate_to_ms_conversion:i32,
    pub(crate) consumed_tokens:i32,
    pub(crate) last_refill_time:i64,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self::new_by_unit(RateUnit::Seconds)
    }

    pub fn new_by_unit(unit:RateUnit) -> Self {
        Self::new_by_conversion(unit.get_rate_to_conversion())
    }

    pub fn new_by_conversion(rate_to_ms_conversion:i32) -> Self {
        Self { 
            rate_to_ms_conversion, 
            consumed_tokens: 0, 
            last_refill_time: now_millis() as i64 
        }
    }

    pub fn acquire(&mut self,burst_size:i32,average_rate:i64) -> bool {
        self.acquire_by_time(burst_size, average_rate, now_millis() as i64)
    }

    pub fn acquire_by_time(&mut self,burst_size:i32,average_rate:i64,current_time_millis:i64) -> bool {
        if burst_size <=0 || average_rate <=0 {
            return true;
        }
        self.refill_token(burst_size, average_rate, current_time_millis);
        self.consume_token(burst_size)
    }

    fn refill_token(&mut self,burst_size:i32,average_rate:i64,current_time_millis:i64) {
        let time_detla = current_time_millis - self.last_refill_time;
        let new_tokens = time_detla * average_rate /(self.rate_to_ms_conversion as i64);
        if new_tokens <=0 {
            return;
        }
        self.last_refill_time = if self.last_refill_time== 0 {
            current_time_millis
        }
        else{
            self.last_refill_time + new_tokens * (self.rate_to_ms_conversion as i64)/average_rate
        };
        let adjusted_level = cmp::min(self.consumed_tokens,burst_size);
        self.consumed_tokens = cmp::max(0,adjusted_level - new_tokens as i32);
    }

    fn consume_token(&mut self,burst_size:i32) -> bool {
        if self.consumed_tokens >= burst_size {
            return false;
        }
        self.consumed_tokens += 1;
        true
    }

    pub fn reset(&mut self){
        self.consumed_tokens = 0;
        self.last_refill_time = 0;
    }
}

#[derive(Debug,Clone,Default)]
pub struct QpsLimiter{
    inner_limit:RateLimiter,
    burst_size:i32,
    qps_limit:i64,
}

impl QpsLimiter {
    pub fn new(qps_limit:u64) -> Self {
        Self {
            burst_size: 1,
            inner_limit: RateLimiter::new(),
            qps_limit: qps_limit as i64,
        }
    }

    pub fn set_burst_size(mut self,burst_size:u32) -> Self{
        self.burst_size = cmp::max(1,burst_size as i32);
        self
    }

    pub fn set_second_limit(mut self,qps_limit:u64) -> Self{
        self.qps_limit = qps_limit as i64;
        self
    }

    pub fn acquire(&mut self) -> bool {
        self.inner_limit.acquire(self.burst_size , self.qps_limit)
    }
    
    pub fn reset(&mut self){
        self.inner_limit.reset()
    }
}