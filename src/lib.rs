//! # wait_timer
//! 
//! Crate for more accurate wait times than `Delay` using `rp2040_hal::timer::Timer`. 
//! In addition to `wait` function that waits for a specified number of seconds, 
//! `gate` function is implemented that keeps the time until it is re-executed constant.
//! 
//! ## How to use
//! 
//! 1. Create `rp2040_hal::timer::Timer`.
//! 
//! ```rust
//! let timer = hal::timer::Timer::new(pac.TIMER, &mut pac.RESETS);
//! ```
//! 
//! 2. Create `wait_timer::Wait`.
//! 
//! ```rust
//! let wait = Wait::new(&timer);
//! ```
//! 
//! 3. Use `wait` and/or `gate` methods.
//! 
//! ## Methods
//! 
//! ### `wait` functions
//! 
//! More accurate waiting times than Delay.
//! 
//! ```rust
//! wait.wait_us(1_000_000);
//! wait.wait_ms(1_000);
//! wait.wait_sec(1);
//! ```
//! 
//! ### `gate` functions
//! 
//! Wait a specified number of seconds since the last execution.
//! 
//! ```rust
//! loop {
//!     wait.gate_sec(1); // A
//!     // ...
//!     wait.gate_ms(500); // B
//!     // ...
//! }
//! ```
//! 
//! 1. A is ignored the first time.
//! 2. The process from A to B is executed in 500 milliseconds.
//! 3. Processes from B onwards (B to A) are executed in a second.

#![no_std]

/// Refer to `rp2040_hal::timer::Timer` and provide `wait` and `gate` methods.
pub struct Wait<'a> {
    timer: &'a rp2040_hal::timer::Timer,
    gating: bool,
    gate_start: u64,
}

impl<'a> Wait<'a> {
    /// Create `Wait` struct.
    pub fn new(timer: &'a rp2040_hal::timer::Timer) -> Self {
        Self { timer: timer, gating: false, gate_start: 0, }
    }

    /// Get the current count of the `rp2040_hal::timer::Timer`.
    pub fn time(&self) -> u64 {
        self.timer.get_counter().ticks()
    }

    /// Wait for the specified time (microseconds).
    pub fn wait_us(&self, us: u64) {
        let start = self.time();
        while self.time() - start < us {}
    }
    /// Wait for the specified time (milliseconds).
    pub fn wait_ms(&self, ms: u64) {
        self.wait_us(ms * 1000);
    }
    /// Wait for the specified time (seconds).
    pub fn wait_sec(&self, sec: u64) {
        self.wait_ms(sec * 1000)
    }

    /// Wait for the specified time (microseconds) 
    /// since the last time a `gate` function was executed.
    pub fn gate_us(&mut self, us: u64) -> Result<(), u64> {
        if !self.gating {
            self.gate_start = self.time();
            self.gating = true;
            return Ok(());
        }
        if self.time() - self.gate_start > us {
            let elapsed_time = self.gate_start - self.time();
            self.gate_start = self.time();
            return Err(elapsed_time);
        }
        while self.time() - self.gate_start < us {}
        self.gate_start = self.time();
        Ok(())
    }
    /// Wait for the specified time (milliseconds) 
    /// since the last time a `gate` function was executed.
    pub fn gate_ms(&mut self, ms: u64) -> Result<(), u64> {
        self.gate_us(ms * 1000)
    }
    /// Wait for the specified time (seconds) 
    /// since the last time a `gate` function was executed.
    pub fn gate_sec(&mut self, sec: u64) -> Result<(), u64> {
        self.gate_ms(sec * 1000)
    }
}