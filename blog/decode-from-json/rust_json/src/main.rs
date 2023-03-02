fn main() {
   let res = attohttpc::get("http://httpbin.org/get").send().unwrap();
   let body = res.text().unwrap();
   let value = rust_json::json_parse(&body).unwrap();
   let origin: String = value["origin"].clone().get().unwrap();
   assert!(origin != "");
}
