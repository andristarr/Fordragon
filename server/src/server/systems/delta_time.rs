use std::time::Instant;

use bevy_ecs::{resource::Resource, system::ResMut};
use log::trace;

#[derive(Resource)]
pub struct DeltaTime {
    time: Instant,
}

impl DeltaTime {
    pub fn new() -> Self {
        DeltaTime {
            time: Instant::now(),
        }
    }

    pub fn set(&mut self, time: Instant) {
        self.time = time;
    }

    pub fn current(&self) -> Instant {
        self.time
    }
}

impl Default for DeltaTime {
    fn default() -> Self {
        DeltaTime::new()
    }
}

pub fn update_delta_time(mut delta_time: ResMut<DeltaTime>) {
    let now = Instant::now();

    trace!(
        "Elapsed time since last update: {:?}",
        now.duration_since(delta_time.current())
    );

    delta_time.set(now);
}
