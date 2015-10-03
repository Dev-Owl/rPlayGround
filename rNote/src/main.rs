extern crate rustc_serialize;
extern crate time;

mod setting;
mod notes;
mod setup;

use notes::Note;

//TODO: Path handling is not correct change to PathBuf ??
//TODO: Add function to check cmd arguments
//TODO: Add function to search for notes
//TODO: Add function to print notes to scrren (inside note obj)

#[cfg(not(test))]
fn main() {
   //Setup env for application
   setup::env_check();
   let settings = setting::get_config();
}
