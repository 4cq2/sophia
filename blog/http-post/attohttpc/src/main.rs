use {
   rust_json::json,
   std::io
};

fn main() {
   let value = json!({
      "clientName": "ANDROID",
      "clientVersion": "17.32.35"
   });
   let mut res = attohttpc::post("http://httpbin.org/post").
      bytes(value.to_string()).
      send().unwrap();
   io::copy(&mut res, &mut io::stdout()).unwrap();
}
