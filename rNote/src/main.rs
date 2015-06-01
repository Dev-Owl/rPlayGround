extern crate rustc_serialize;
extern crate time;

mod setting;
mod notes;
use notes::Note;
use std::env;


fn main() {
   tests();   
}

fn tests(){
	println!("");
	println!("-----------------Self test-----------------------------");
	print!("Get config: ");
	let settings = setting::get_config();
	println!("OK");
	println!("Creating new note");
	print!("1. New id: ");
	let new_id: u32 = notes::Note::new_id( &settings.get_default("data","data"), 
										   settings.get_default("id_offset","500").parse::<u32>().unwrap());
	println!("{}", new_id);
	print!("Note struct: ");
	let mut my_note = notes::Note::new(new_id,"Owls everywhere");
	println!("OK");
	print!("Add a tag: ");
	my_note.add_tag("Testing");
	println!("OK");
	println!("Our note so far:");
	println!("ID          -> {}", my_note.id);
	println!("title       -> {}", my_note.title);
	println!("tag         -> {:?}", my_note.tag);
	println!("creation    -> {}", my_note.creation);
	println!("last_update -> {}", my_note.last_update);
	println!("-------------------------------------------------------");
}
