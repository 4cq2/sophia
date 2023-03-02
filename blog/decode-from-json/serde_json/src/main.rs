use serde_json::Value;

fn main() {
   let res = attohttpc::get("http://httpbin.org/get").send().unwrap();
   let value: Value = serde_json::from_reader(res).unwrap();
   let origin = value["origin"].as_str().unwrap();
   dbg!(origin);
}
