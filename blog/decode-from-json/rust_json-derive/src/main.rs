use {
   rust_json::json_parse,
   rust_json_derive::FromJson
};

#[derive(FromJson)]
struct Hello {
   origin: String,
   url: String
}

fn main() {
   let res = attohttpc::get("http://httpbin.org/get").send().unwrap();
   let body = res.text().unwrap();
   let hi: Hello = json_parse(&body).unwrap().
      get().unwrap();
   assert!(hi.origin != "" && hi.url != "");
}
