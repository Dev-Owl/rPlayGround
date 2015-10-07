use time;
use std::io;
use std::fs;
use std::path::Path;
use setup;
use rustc_serialize::json;
use std::io::prelude::*;
use std::borrow::Borrow;
use std::fmt;
use std::fs::OpenOptions;

use setting;

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
		Note {creation: unix_timestamp(), id:new_id,title:new_title,done:false,started:false, ..Default::default() }
	}

	pub fn load(exising_id : u32, path: &str) -> Note{
		let mut final_path = path.to_string();
		final_path.push_str("/");
		final_path.push_str(&exising_id.to_string());
		return match json::decode(&setup::file_read(&final_path)){
			Ok(n) => n,
			Err(e) => panic!("decode of json failed:{} for ID: {}", e, exising_id),
		};
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
		self.last_update = unix_timestamp();
	}

	pub fn new_id(path: &str, id_offset: u32) -> u32{
		let mut new_id: u32 = 0;
		let mut tmp : u32 = id_offset;
		//Get list of files search for next free id
		match file_list( Path::new( path)){
			Ok(v) => {
					  for x in v.iter()
					  {
						if match x.parse::<u32>() {
							Ok(v) => v,
							error => 0,
						} > tmp {
							tmp = match x.parse::<u32>() {
								Ok(t) => t,
								error => 0,
							};
						}
					  }
					},
			error => panic!("Unable to read in data directory at {}", path),
		};

		new_id += tmp +1;
		new_id
	}

	pub fn exists(path: &str,id : u32) -> bool{
		let mut result = false;
		match file_list( Path::new( path)){
			Ok(v) => {
					  for x in v.iter()
					  {
						  if match x.parse::<u32>() {
  							Ok(v) => v,
  							error => 0,
  						} == id {
  							result = true;
  						}
					  }
				  },
		    Err(e) => panic!("Unable to read data directory at {}",path),
		};
		result
	}

	pub fn save(&self, path: &str){
		let mut final_path = path.to_string();
		final_path.push_str("/");
		final_path.push_str(&self.id.to_string());
		let mut file = setup::file_create(&final_path);
		//file.write_all().unwrap();
		//file.sync_all();
		match OpenOptions::new().create(true).write(true).append(false).open(&final_path) {
        	Ok(ref mut file) => {
            	write!(
                	file,
					"{}",
                	json::encode(&self).unwrap()
            	).unwrap();
        	},
        	Err(err) => { panic!("Failed to open log file: {}", err); }
    	}

	}

	pub fn delete(&self, path: &str){
		let mut final_path = path.to_string();
		final_path.push_str("/");
		final_path.push_str(&self.id.to_string());
		if ! fs::metadata(&final_path).is_err(){
			if( fs::remove_file(&final_path).is_err())
			{
				panic!("Failed to delete a note at {}",path);
			}
		}
		else{
			panic!("Loaded note is not on the harddsik")
		}
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
		note_text.push_str(&format!("Last update: {}", from_unix_timestamp(self.last_update).ctime()));
		note_text.push_str("\n");
		note_text.push_str(&format!("Last update: {}", from_unix_timestamp(self.creation).ctime()));
		note_text.push_str("\n");
		write!(f,"{}",note_text)
    }
}

pub fn from_unix_timestamp(timestamp: i64) -> time::Tm{
	let mut tm: time::Tm =  time::empty_tm();
	tm.tm_year = 70;
	return tm + time::Duration::seconds(timestamp);
}

pub fn unix_timestamp() -> i64{
	time::now_utc().to_timespec().sec
}


pub fn file_list(dir: &Path) -> io::Result<Vec<String>>{
    let mut files : Vec<String> = Vec::new();
	for entry in try!(fs::read_dir(dir)) {
		let entry = try!(entry);
		let meta  = try!(fs::metadata( entry.path()));
		if meta.is_file() {
			files.push( entry.path().file_name().unwrap().to_str().unwrap().to_string());
		}
	}
    Ok(files)
}

#[test]
fn test_note_print()
{
	let settings = setting::get_config();let settings = setting::get_config();
	let new_id: u32 = Note::new_id( &settings.get_default("data","data"),
										settings.get_default("id_offset","500").parse::<u32>().unwrap());

	let mut my_note = Note::new(new_id,"Owls everywhere".to_string());
	my_note.add_tag("Testing".to_string());
	my_note.add_tag("FuBar".to_string());
	my_note.add_tag("Wurst".to_string());
	my_note.set_text("This is my very long description for a very short and easy test task :)");
	println!("{}", my_note);
}


#[test]
fn test_note_create_exists()
{
  let settings = setting::get_config();
  let new_id: u32 = Note::new_id( &settings.get_default("data","data"),
										 settings.get_default("id_offset","500").parse::<u32>().unwrap());

  let mut my_note = Note::new(new_id,"Owls everywhere".to_string());
  my_note.add_tag("Testing".to_string());
  my_note.save( &settings.get_default("data","data"));
  assert_eq!( Note::exists(&settings.get_default("data","data"),new_id),true);
  my_note.delete(&settings.get_default("data","data"));
}

#[test]
fn test_note_delete(){
	let settings = setting::get_config();
	let new_id: u32 = Note::new_id( &settings.get_default("data","data"),
  										 settings.get_default("id_offset","500").parse::<u32>().unwrap());

    let my_note = Note::new(new_id,"Owls everywhere".to_string());
	my_note.save( &settings.get_default("data","data"));
	my_note.delete(&settings.get_default("data","data"));
}

#[test]
fn test_note_load(){
	let settings = setting::get_config();
	let new_id: u32 = Note::new_id( &settings.get_default("data","data"),
										settings.get_default("id_offset","500").parse::<u32>().unwrap());

	let mut my_note = Note::new(new_id,"Owls everywhere".to_string());

	my_note.add_tag("Testing".to_string());
	my_note.save( &settings.get_default("data","data"));
	let mut loaded_note = Note::load(new_id, &settings.get_default("data","data"));
	my_note.delete(&settings.get_default("data","data"));

}
