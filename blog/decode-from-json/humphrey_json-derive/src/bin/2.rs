use humphrey_json::FromJson;

#[derive(Debug, FromJson)]
struct Date {
   #[rename = "currentMonth"]
   current_month: u8,
   #[rename = "currentDay"]
   current_day: u8
}

fn main() {
   let raw = r#"{"currentMonth": 12, "currentDay": 31}"#;
   let d: Date = humphrey_json::from_str(raw).unwrap();
   assert!(d.current_month == 12 && d.current_day == 31);
}
