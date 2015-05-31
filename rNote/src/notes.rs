use time;
use std::io;
use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct Note{
	pub title: String,
	pub tag: Vec<String>,
	pub last_update: u32,
	pub creation: u32,
	pub id: u32,
}


impl Note{
	
	pub fn new(new_id : u32,  new_title: String) -> Note{
		Note {creation: unix_timestamp(), id:new_id,title:new_title, ..Default::default() }
	}
	
	pub fn add_tag(&mut self, tag: &str){
		if !self.tag.contains( &tag.to_string()){
			self.tag.push( tag.to_string());
		}
	}
	
	pub fn has_tag(&self, tag: &str) ->bool{
		self.tag.contains( &tag.to_string())
	}
	
	pub fn update(&mut self){
		self.last_update = unix_timestamp();
	}
	
	pub fn new_id(path: String, id_offset: u32) -> u32{
		//Get list of files search for next free id
		1
	}
	
}

pub fn unix_timestamp() -> u32{
	time::now_utc().to_timespec().sec as u32
}


pub fn file_list(dir: &Path) -> io::Result<Vec<String>>{
    let mut files : Vec<String> = Vec::new();
	for entry in try!(fs::read_dir(dir)) {
		let entry = try!(entry);
		let meta  = try!(fs::metadata( entry.path()));
		if meta.is_file() {
			files.push( entry.path().to_str().unwrap().to_string());
		}
	}
    Ok(files)
}
