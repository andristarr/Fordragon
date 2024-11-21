use std::{
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use semaphore::Semaphore;

pub trait TickerTrait: Send + Sync {
    fn register(&mut self, callback: Box<dyn Fn() + Send>);
    fn run(&mut self);
}

pub struct Ticker {
    tick_count: u8,
    semaphore: Semaphore<bool>,
    state: Arc<Mutex<TickerState>>,
}

pub struct TickerState {
    callbacks: Vec<Box<dyn Fn() + Send>>,
    running: bool,
}

impl Ticker {
    pub fn new(tick_count: u8) -> Ticker {
        Ticker {
            tick_count,
            semaphore: Semaphore::new(1, false),
            state: Arc::new(Mutex::new(TickerState {
                callbacks: vec![],
                running: false,
            })),
        }
    }
}

impl TickerTrait for Ticker {
    fn register(&mut self, callback: Box<dyn Fn() + Send>) {
        self.state.lock().unwrap().callbacks.push(callback);
    }

    fn run(&mut self) {
        println!("Running ticker...");
        let tick_length = 1_000 / u128::from(self.tick_count);

        let mut next_run_time: u128 = tick_length;

        let shared = Arc::clone(&self.state);

        let mut state = self.state.lock().unwrap();

        if self.semaphore.try_access().is_err() {
            return;
        }

        if !state.running {
            state.running = true;

            tokio::spawn(async move {
                let mut now = std::time::Instant::now();
                loop {

                    let millis = now.elapsed().as_millis();

                    if millis >= tick_length {
                        if millis > tick_length {
                            println!("Throtting detected, last tick took {}ms", millis);
                        }

                        now = std::time::Instant::now();

                        for callback in &shared.lock().unwrap().callbacks {
                            callback();
                        }
                    }
                }
            });
        }
    }
}
