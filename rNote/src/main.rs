extern crate rustc_serialize;
extern crate time;

mod setting;
mod notes;
use notes::Note;


fn main() {
   let settings = setting::get_config();
   let mut my_note = notes::Note::new();
   println!("{}", my_note.creation);
}
