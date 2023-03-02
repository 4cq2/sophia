use {
   humphrey_json::json,
   oxhttp::Client,
   oxhttp::model::Method,
   oxhttp::model::Request,
   std::io
};

fn main() {
   let value = json!({
      "clientName": "ANDROID",
      "clientVersion": "17.32.35"
   });
   let req = Request::builder(
      Method::POST, "http://httpbin.org/post".parse().unwrap()
   ).with_body(value.serialize());
   let mut body = Client::new().request(req).unwrap().into_body();
   io::copy(&mut body, &mut io::stdout()).unwrap();
}
