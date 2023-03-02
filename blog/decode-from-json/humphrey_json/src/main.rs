use humphrey_json::Value;

fn main() {
   let res = attohttpc::get("http://httpbin.org/get").send().unwrap();
   let body = res.text().unwrap();
   let val = Value::parse(&body).unwrap();
   let origin = val["origin"].as_str().unwrap();
   assert!(origin != "");
}
