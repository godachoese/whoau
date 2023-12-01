use chrono::{FixedOffset, Utc};

pub fn now_kst() -> String {
    let kst = FixedOffset::east_opt(9 * 60 * 60).unwrap();
    let now = Utc::now().with_timezone(&kst);
    now.format("%H:%M:%S").to_string()
}
