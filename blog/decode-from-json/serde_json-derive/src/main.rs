use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Get {
   origin: String,
   url: String
}

fn main() {
   let res = attohttpc::get("http://httpbin.org/get").send().unwrap();
   let val: Get = serde_json::from_reader(res).unwrap();
   assert!(val.origin != "" && val.url != "");
}
