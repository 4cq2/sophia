use humphrey_json::FromJson;

#[derive(Debug, FromJson)]
struct Hello {
   origin: String,
   url: String
}

fn main() {
   let res = attohttpc::get("http://httpbin.org/get").send().unwrap();
   let body = res.text().unwrap();
   let h: Hello = humphrey_json::from_str(body).unwrap();
   assert!(h.origin != "" && h.url != "");
}
