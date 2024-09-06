use std::{
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use tokio::sync::Semaphore;

pub trait TickerTrait {
    fn register(&mut self, callback: Box<dyn Fn() + Send + Sync>);
    fn run(&mut self);
}

pub struct Ticker {
    tick_count: u8,
    semaphore: Semaphore,
    state: Arc<Mutex<TickerState>>,
}

pub struct TickerState {
    callbacks: Vec<Box<dyn Fn() + Send + Sync>>,
    running: bool,
}

impl Ticker {
    pub fn new(tick_count: u8) -> Ticker {
        Ticker {
            tick_count,
            semaphore: Semaphore::new(1),
            state: Arc::new(Mutex::new(TickerState {
                callbacks: vec![],
                running: false,
            })),
        }
    }
}

impl TickerTrait for Ticker {
    fn register(&mut self, callback: Box<dyn Fn() + Send + Sync>) {
        self.state.lock().unwrap().callbacks.push(callback);
    }

    fn run(&mut self) {
        let tick_length = 1_000 / u128::from(self.tick_count);

        let mut next_run_time: u128 = tick_length;

        let shared = Arc::clone(&self.state);

        let mut state = self.state.lock().unwrap();

        // semaphore should be synced here

        if !state.running {
            state.running = true;

            tokio::spawn(async move {
                loop {
                    let now = SystemTime::now();

                    let millis = now.duration_since(UNIX_EPOCH).unwrap().as_millis() % 1_000;

                    if millis >= next_run_time {
                        next_run_time = (millis / tick_length) * tick_length + tick_length;

                        for callback in &shared.lock().unwrap().callbacks {
                            callback();
                        }
                    }
                }
            });
        }
    }
}
