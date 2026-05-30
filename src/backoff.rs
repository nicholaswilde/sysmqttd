use std::sync::Mutex;
use std::time::Duration;

/// A self-contained, lightweight Linear Congruential Generator (LCG) PRNG.
/// This avoids external dependencies and makes cross-compilation robust and fast.
#[derive(Debug)]
struct LcgRng {
    state: u64,
}

impl LcgRng {
    fn new() -> Self {
        // Seed with current system time nanos, or a default constant if system time is unavailable.
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(123456789);
        Self { state: seed }
    }

    #[cfg(test)]
    fn with_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        // MMIX LCG multiplier and increment
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }

    fn next_f64(&mut self) -> f64 {
        let val = self.next_u64();
        // Map 64-bit value to [0.0, 1.0)
        (val as f64) / (u64::MAX as f64)
    }
}

/// Dynamic backoff strategy implementing Full Jitter exponential backoff.
///
/// Formula: `actual_delay = random(0, min(max_delay, initial_delay * 2^retries))`
#[derive(Debug)]
pub struct Backoff {
    initial_delay: Duration,
    max_delay: Duration,
    retries: u32,
    rng: Mutex<LcgRng>,
}

impl Backoff {
    /// Creates a new Backoff instance with configuration limits.
    pub fn new(initial_delay: Duration, max_delay: Duration) -> Self {
        Self {
            initial_delay,
            max_delay,
            retries: 0,
            rng: Mutex::new(LcgRng::new()),
        }
    }

    /// Resets the consecutive retries count to 0.
    pub fn reset(&mut self) {
        self.retries = 0;
    }

    /// Increments the consecutive retries count.
    pub fn increment(&mut self) {
        self.retries = self.retries.saturating_add(1);
    }

    /// Returns the current consecutive retries count.
    pub fn retries(&self) -> u32 {
        self.retries
    }

    /// Calculates the next delay, increments the retry counter, and returns the delay.
    pub fn next_delay(&mut self) -> Duration {
        let delay = self.calculate_delay(self.retries);
        self.increment();
        delay
    }

    /// Mathematical delay calculation with full jitter for a specific retry count.
    /// Does not mutate the retry counter state.
    pub fn calculate_delay(&self, retries: u32) -> Duration {
        // Capping retries at 30 to avoid exponent overflow in checked_mul
        let factor = 2u64.saturating_pow(retries.min(30));
        let max_range = if let Some(product) = self.initial_delay.checked_mul(factor as u32) {
            product.min(self.max_delay)
        } else {
            self.max_delay
        };

        if max_range.is_zero() {
            return Duration::ZERO;
        }

        // Lock RNG and get a factor in [0.0, 1.0)
        let mut rng = self.rng.lock().unwrap();
        let rand_factor = rng.next_f64();

        let nanos = max_range.as_nanos() as f64 * rand_factor;
        Duration::from_nanos(nanos as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backoff_ranges() {
        let initial = Duration::from_secs(2);
        let max_delay = Duration::from_secs(300);
        let backoff = Backoff {
            initial_delay: initial,
            max_delay,
            retries: 0,
            rng: Mutex::new(LcgRng::with_seed(42)),
        };

        // For retry = 0: range is [0, 2s]
        for _ in 0..100 {
            let delay = backoff.calculate_delay(0);
            assert!(delay <= Duration::from_secs(2));
            assert!(delay >= Duration::ZERO);
        }

        // For retry = 3: range is [0, min(300, 2 * 2^3)) = [0, 16s]
        for _ in 0..100 {
            let delay = backoff.calculate_delay(3);
            assert!(delay <= Duration::from_secs(16));
            assert!(delay >= Duration::ZERO);
        }

        // For retry = 10: range is [0, min(300, 2 * 1024)) = [0, 300s]
        for _ in 0..100 {
            let delay = backoff.calculate_delay(10);
            assert!(delay <= Duration::from_secs(300));
            assert!(delay >= Duration::ZERO);
        }
    }

    #[test]
    fn test_backoff_max_delay_respected() {
        let initial = Duration::from_secs(2);
        let max_delay = Duration::from_secs(30);
        let backoff = Backoff {
            initial_delay: initial,
            max_delay,
            retries: 0,
            rng: Mutex::new(LcgRng::with_seed(101)),
        };

        // For high retry numbers, max limit should be respected
        for retry in 10..35 {
            for _ in 0..50 {
                let delay = backoff.calculate_delay(retry);
                assert!(delay <= max_delay, "Retry {} produced delay {:?} exceeding max {:?}", retry, delay, max_delay);
            }
        }
    }

    #[test]
    fn test_backoff_state_progression() {
        let initial = Duration::from_secs(2);
        let max_delay = Duration::from_secs(300);
        let mut backoff = Backoff::new(initial, max_delay);

        assert_eq!(backoff.retries(), 0);
        
        let d1 = backoff.next_delay();
        assert_eq!(backoff.retries(), 1);
        assert!(d1 <= Duration::from_secs(2));

        let d2 = backoff.next_delay();
        assert_eq!(backoff.retries(), 2);
        assert!(d2 <= Duration::from_secs(4));

        backoff.reset();
        assert_eq!(backoff.retries(), 0);
    }

    #[test]
    fn test_mathematical_distribution() {
        let initial = Duration::from_secs(2);
        let max_delay = Duration::from_secs(300);
        let backoff = Backoff {
            initial_delay: initial,
            max_delay,
            retries: 0,
            rng: Mutex::new(LcgRng::with_seed(999)),
        };

        // Run many samples for retry = 5 -> range [0, 64s]
        let mut total_nanos = 0u64;
        let iterations = 1000;
        let mut min_seen = Duration::from_secs(999);
        let mut max_seen = Duration::ZERO;

        for _ in 0..iterations {
            let delay = backoff.calculate_delay(5);
            total_nanos += delay.as_nanos() as u64;
            if delay < min_seen {
                min_seen = delay;
            }
            if delay > max_seen {
                max_seen = delay;
            }
        }

        let average = Duration::from_nanos(total_nanos / iterations);
        // Average of uniform [0, 64] should be close to 32
        assert!(average > Duration::from_secs(20) && average < Duration::from_secs(44),
                "Average delay of {:?} is out of expected distribution center [20s, 44s]", average);

        assert!(min_seen < Duration::from_secs(5), "Expected to see some small delays, got min_seen = {:?}", min_seen);
        assert!(max_seen > Duration::from_secs(58), "Expected to see some large delays near 64s, got max_seen = {:?}", max_seen);
    }
}
