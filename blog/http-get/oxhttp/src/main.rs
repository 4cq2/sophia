use {
   oxhttp::Client,
   oxhttp::model::Method,
   oxhttp::model::Request,
   std::io
};

fn main() {
   let req = Request::builder(
      Method::GET, "https://httpbin.org/get".parse().unwrap()
   ).build();
   let mut body = Client::new().request(req).unwrap().into_body();
   io::copy(&mut body, &mut io::stdout()).unwrap();
}
