use std::io;

fn main() {
   let req = attohttpc::get("https://httpbin.org/get");
   let mut res = req.send().unwrap();
   io::copy(&mut res, &mut io::stdout()).unwrap();
}
