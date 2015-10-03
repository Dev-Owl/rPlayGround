extern crate rustc_serialize;
extern crate time;

mod setting;
mod notes;
mod setup;

use notes::Note;

//TODO: Path handling is not correct change to PathBuf ??

#[cfg(not(test))]
fn main() {
   //Setup env for application
   setup::env_check();
   let settings = setting::get_config();
}
