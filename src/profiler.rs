use std::time::{SystemTime, UNIX_EPOCH};

pub struct Profiler {
    started: u128,
}

impl Profiler {
    pub fn start() -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Self {
            started: since_the_epoch.as_millis(),
        }
    }

    pub fn stop_and_print(&self) {
        let end = SystemTime::now();
        let since_the_epoch = end.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let diff_as_millis = since_the_epoch.as_millis() - self.started;
        let unit_format = match diff_as_millis {
            0..=1000 => format!("{} milliseconds", diff_as_millis),
            _ => format!("{:.3} seconds", diff_as_millis as f64 / 1000 as f64,),
        };
        println!("Finished in {}", unit_format);
    }
}
