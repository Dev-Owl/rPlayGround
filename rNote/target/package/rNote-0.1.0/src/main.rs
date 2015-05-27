extern crate rustc_serialize;

mod setting;

fn main() {
   let settings = setting::get_config();
   println!("------Configuration-----");
   println!("UTC      : {}",settings.utc);
   println!("id_offset: {}",settings.id_offset);
   println!("data     : {}",settings.utc);
   println!("------------------------");
}
