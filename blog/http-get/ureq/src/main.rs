fn main() -> Result<(), ureq::Error> {
   let body: String = ureq::get("http://httpbin.org/get")
      .call()?
      .into_string()?;
   dbg!(body);
   Ok(())
}
