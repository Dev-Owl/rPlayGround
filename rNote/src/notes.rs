use time;
use std::io;
use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct Note<'a>{
	pub title: &'a str,
	pub tag: Vec<&'a str>,
	pub last_update: u32,
	pub creation: u32,
	pub id: u32,
}


impl<'a> Note<'a>{
	
	pub fn new(new_id : u32,  new_title: &str) -> Note{
		Note {creation: unix_timestamp(), id:new_id,title:new_title, ..Default::default() }
	}
	
	pub fn add_tag(&mut self, tag: &'a str){
		if !self.has_tag(tag){
			self.tag.push(tag);
		}
		self.update();
	}
	
	pub fn has_tag(&self, tag: &str) ->bool{
		self.tag.contains( &tag)
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
							Error => 0,
						} > tmp {
							tmp = x.parse::<u32>().unwrap();
						}
					  } 
					},
			Error => panic!("Unable to read in data directory at {}", path),
		};
		
		new_id += tmp +1;
		new_id
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
