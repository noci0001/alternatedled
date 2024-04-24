use std::time::{Instant, Duration};
use std::cmp::Ordering;

// pub fn run_animation(delay_ms_led1: usize, delay_ms_led2: usize) {
//
//     let mut led1 = Led::new(1, delay_ms_led1, "blue");
//     let mut led2 = Led::new(2, delay_ms_led2, "green");
//
//     let round_length = round_length(delay_ms_led1 as f64, delay_ms_led2 as f64);
//     println!("The other LED will change its state {} times after the blinking of the other LED", round_length);
//     let mut tracker = 0;
//     let start_time = Instant::now();
//     //turn both LEDs on at first
//     led1.switch();
//     led2.switch();
//     let mut was_last_less_frequent_led = false;
//     loop {
//         while tracker != round_length as i32 {
//             if was_last_less_frequent_led == true {
//                 std::thread::sleep(Duration::from_millis(delay_ms_led1 as u64 / 2));
//                 was_last_less_frequent_led = false;
//             } else {
//                 std::thread::sleep(Duration::from_millis(delay_ms_led1 as u64));
//             }
//             println!("Elapsed time: {:?}", start_time.elapsed().as_secs());
//             led1.switch();
//             tracker += 1;
//         }
//         tracker = 0;
//         std::thread::sleep(Duration::from_millis(delay_ms_led2 as u64 % delay_ms_led1 as u64));
//         println!("Elapsed time: {:?}", start_time.elapsed().as_secs());
//         led2.switch();
//         was_last_less_frequent_led = true;
//     }
// }

pub fn set_state(led: usize, state: bool) {
    if state {
        println!("Turning on LED {}", led);
    }
    else {
        println!("\x1b[31mTurning off LED {}\x1b[0m", led);
    }
}

#[derive(PartialEq)]
pub struct Led {
    id: u32,
    interval: usize,
    more_frequent: bool,
    state: bool,
    color: String,
}


impl Led {
    fn new(id: u32, interval: usize, color: &str) -> Self {
        Self {
            id,
            interval,
            state: false,
            more_frequent: false,
            color: color.to_string(),
        }

    }

    fn switch(&mut self) {
        if self.state == false {
            self.state = true;
            if self.color == "blue" {
                println!("\x1b[34m \tTurning on LED {}\x1b[0m", self.id);
            } else {
                println!("\x1b[32m \tTurning on LED {}\x1b[0m", self.id);
            }
        } else {
            self.state = false;
            println!("\x1b[31mTurning off LED {}\x1b[0m", self.id);
        }
    }
}

impl PartialOrd for Led {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.interval.cmp(&other.interval))
    }
}


fn round_length(delay1: f64, delay2: f64) -> f64 {
    if delay1 > delay2 {
        delay1 / delay2
    } else {
        delay2 / delay1
    }
}

pub fn get_smallest_interval(led1: &Led, led2: &Led) -> usize {
    if led1 > led2 {
        led2.interval
    } else {
        led1.interval
    }
}

pub fn get_biggest_interval(led1: &Led, led2: &Led) -> usize {
    if led1 > led2 {
        led1.interval
    } else {
        led2.interval
    }
}

pub fn switch_correct_led(led1: &mut Led, led2: &mut Led, smallest_interval: u64) {
    if smallest_interval == led1.interval as u64 {
        led1.switch();
    } else {
        led2.switch();
    }
}

pub fn run_animation(delay_ms_led1: usize, delay_ms_led2: usize) {

    let mut led1 = Led::new(1, delay_ms_led1, "blue");
    let mut led2 = Led::new(2, delay_ms_led2, "green");

    let round_length = round_length(led1.interval as f64, led2.interval as f64);
    let smallest_interval = get_smallest_interval(&led1, &led2) as u64;
    let biggest_interval = get_biggest_interval(&led1, &led2) as u64;
    println!("The other LED will change its state {} times after the blinking of the other LED", round_length);
    let mut tracker = 0;
    let start_time = Instant::now();
    led1.switch();
    led2.switch();
    let mut less_frequent_led_was_last = false;
    loop {
        while tracker != round_length as i32 {
            if less_frequent_led_was_last == true {
                std::thread::sleep(Duration::from_millis( smallest_interval / 2));
                less_frequent_led_was_last = false;
            } else {
                std::thread::sleep(Duration::from_millis(smallest_interval));
            }
            println!("Elapsed time: {:?}", start_time.elapsed().as_secs());
            switch_correct_led(&mut led1, &mut led2, smallest_interval);
            tracker += 1;
        }
        tracker = 0;
        std::thread::sleep(Duration::from_millis(biggest_interval % smallest_interval));
        println!("Elapsed time: {:?}", start_time.elapsed().as_secs());
        switch_correct_led(&mut led1, &mut led2, biggest_interval);
        less_frequent_led_was_last = true;
    }
}


fn main(){
    run_animation(2000, 15000);
}