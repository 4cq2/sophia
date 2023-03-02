use s8::Player;

fn main() {
   let play = Player::new().unwrap();
   let count = play.view_count().unwrap();
   let title = play.title().unwrap();
   dbg!(count, title);
}
