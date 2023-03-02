use humphrey_json::{
   Value,
   error::TracebackError
};

#[derive(Debug)]
pub enum Error {
   HTTP(attohttpc::Error),
   JSON(TracebackError)
}

impl From<attohttpc::Error> for Error {
   fn from(err: attohttpc::Error) -> Self {
      Self::HTTP(err)
   }
}

impl From<TracebackError> for Error {
   fn from(err: TracebackError) -> Self {
      Self::JSON(err)
   }
}

const YOUTUBE: &str = "https://www.youtube.com/youtubei/v1/player";

impl Player {
   
   pub fn new() -> Result<Self, Error> {
      let body = humphrey_json::json!({
         "videoId": "aGCdLKXNF3w",
         "context": {
            "client": {
               "clientName": "ANDROID",
               "clientVersion": "17.32.35"
            }
         }
      });
      let res = attohttpc::post(YOUTUBE).
         header("X-Goog-API-Key", "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8").
         bytes(body.serialize()).
         send()?;
      let body = res.text()?;
      let play = Value::parse(&body)?;
      Ok(Self(play))
   }
   
   pub fn title(&self) -> Option<&str> {
      self.video_details()["title"].as_str()
   }
   
   pub fn view_count(&self) -> Option<&str> {
      self.video_details()["viewCount"].as_str()
   }

   fn video_details(&self) -> &Value {
      &self.0["videoDetails"]
   }
   
   fn adaptive_formats(&self) -> &Value {
      &self.0["streamingData"]["adaptiveFormats"]
   }
}

pub struct Player(Value);
