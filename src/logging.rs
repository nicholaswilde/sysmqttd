use std::sync::atomic::{AtomicBool, Ordering};

pub static QUIET_LOGGING: AtomicBool = AtomicBool::new(false);

/// Enable or disable quiet logging mode.
pub fn set_quiet_logging(quiet: bool) {
    QUIET_LOGGING.store(quiet, Ordering::Relaxed);
}

/// Returns whether quiet logging mode is active.
pub fn is_quiet_logging() -> bool {
    QUIET_LOGGING.load(Ordering::Relaxed)
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        if !$crate::logging::is_quiet_logging() {
            std::println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        if !$crate::logging::is_quiet_logging() {
            std::eprintln!($($arg)*);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiet_logging_toggle() {
        set_quiet_logging(false);
        assert!(!is_quiet_logging());

        set_quiet_logging(true);
        assert!(is_quiet_logging());

        // Reset
        set_quiet_logging(false);
    }

    #[test]
    fn test_quiet_logging_macros() {
        set_quiet_logging(true);
        println!("This should not be printed to stdout");
        eprintln!("This should not be printed to stderr");

        set_quiet_logging(false);
        println!("This should be printed to stdout");
    }
}
