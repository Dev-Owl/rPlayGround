use time;
use std::path::Path;
use rustc_serialize::json;
use std::io::prelude::*;
use std::borrow::Borrow;
use std::fmt;
use note_handler;
use setup;
use setting;
use helper_time::unix_utc_now;
use helper_time::to_tm;



#[derive(RustcDecodable, RustcEncodable, Default)]
pub struct Note{
	pub title: String,
	pub tag: Vec<String>,
	pub project: String,
	pub text: String,
	pub done: bool,
	pub started: bool,
	pub last_update: i64,
	pub creation: i64,
	pub id: u32
}


impl Note{

	pub fn new(new_id : u32,  new_title: String) -> Note{
		Note {creation: unix_utc_now(), id:new_id,title:new_title,done:false,started:false, ..Default::default() }
	}

	pub fn add_tag(&mut self, tag: String){
		if !self.has_tag(&tag){
			self.tag.push(tag.to_string());
		}
		self.update();
	}

	pub fn has_tag(&self,tag: &str) ->bool{
		self.tag.contains(&tag.to_string())
	}

	pub fn set_text(&mut self,text: &str){
		self.text = text.to_string();
		self.update();
	}

	pub fn update(&mut self){
		self.last_update = unix_utc_now();
	}
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut note_text = String::new();
		note_text.push_str(&format!("----{}----\n", self.title ));
		note_text.push_str(&format!("ID: {}",self.id));
		note_text.push_str("\n");
		if self.tag.len() > 0{
			note_text.push_str("Tags:");
			for tag in &self.tag {
    			note_text.push_str(&format!(" {} ", tag ));
			}
		}
		note_text.push_str("\n");
		note_text.push_str("---------------------------");
		note_text.push_str("\n");
		note_text.push_str(&format!("{}\n", self.text ));
		note_text.push_str("---------------------------");
		note_text.push_str("\n");
		if !self.project.is_empty(){
			note_text.push_str(&format!("Projekt: {}",self.id));
			note_text.push_str("\n");
		}
		note_text.push_str(&format!("Started: {}",if self.started { "Yes"} else {"No"}));
		note_text.push_str("\n");
		if self.done{
			note_text.push_str("Done: Yes");
			note_text.push_str("\n");
		}
		note_text.push_str(&format!("Last update: {}", to_tm(self.last_update).ctime()));
		note_text.push_str("\n");
		note_text.push_str(&format!("Last update: {}", to_tm(self.creation).ctime()));
		note_text.push_str("\n");
		write!(f,"{}",note_text)
    }
}



#[test]
fn test_note_print()
{
	let settings = setting::get_config();
    let handler = note_handler::Note_handler::new(settings.get_default("data","data"),
                                         settings.get_default("id_offset","500").parse::<u32>().unwrap());
    let mut my_note = handler.new_note("Fu bar".to_string());
	my_note.add_tag("Testing".to_string());
	my_note.add_tag("FuBar".to_string());
	my_note.set_text("This is my very long description for a very short and easy test task :)");
	println!("{}", my_note);
}
