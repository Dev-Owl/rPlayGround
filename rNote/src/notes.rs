use time;
use std::io;
use std::fs;
use std::path::Path;
use setup;
use rustc_serialize::json;
use std::io::prelude::*;
use std::borrow::Borrow;

#[derive(RustcDecodable, RustcEncodable, Default)]
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
	
	pub fn add_tag(&mut self, tag: String){
		if !self.has_tag(&tag){
			self.tag.push(tag.to_string());
		}
		self.update();
	}
	
	pub fn has_tag(&self,tag: &str) ->bool{
		self.tag.contains(&tag.to_string())
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
							tmp = x.parse::<u32>().unwrap();
						}
					  } 
					},
			error => panic!("Unable to read in data directory at {}", path),
		};
		
		new_id += tmp +1;
		new_id
	}
	
	pub fn save(&self, path: &str){
		let mut final_path = path.to_string();
		final_path.push_str("/");
		final_path.push_str(&self.id.to_string());
		if fs::metadata(&final_path).is_err(){
			let mut file = setup::file_create(&final_path);
			file.write_all(json::encode(&self).unwrap().into_bytes().borrow()).unwrap();
		}
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
			files.push( entry.path().file_name().unwrap().to_str().unwrap().to_string());
		}
	}
    Ok(files)
}
