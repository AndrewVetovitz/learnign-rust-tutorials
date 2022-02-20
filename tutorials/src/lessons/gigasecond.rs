use time::{PrimitiveDateTime as DateTime, Duration};

const GIGA_SECOND: i64 = 1_000_000_000;
const DURATION: Duration = Duration::seconds(GIGA_SECOND);

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    return start + DURATION;
}
