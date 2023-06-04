use time::{Duration, PrimitiveDateTime as DateTime};

const GIGA_SECOND: i64 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.saturating_add(Duration::new(GIGA_SECOND, 0))
}
