use std::time::Instant;

/// Tracks elapsed time for an operation.
pub struct Progress {
    start: Instant,
}

impl Progress {
    /// Starts the timer.
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Returns the elapsed time in seconds since creation.
    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}
