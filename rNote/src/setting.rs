
use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::json;
use std::borrow::Borrow;
use std::env;
use std::path::Path;
use std::fs;

//Allow creation of JSON and creation from JSON
#[derive(RustcDecodable, RustcEncodable)]
pub struct Settings{
 	pub utc: bool,
	pub id_offset:u32,
	pub data:String,
}


#[cfg(any(unix))]
pub fn get_config() -> Settings{	
	let result = env::home_dir();
	if !result.is_none(){
		let mut path = result.unwrap();
		path = path.join("rnote/.rnote");
		return read_setting( path.to_str().unwrap());
	}
	return read_setting( ".rnote");
}

#[cfg(any(windows))]
pub fn get_config() -> Settings{
	let result = env::home_dir();
	if !result.is_none(){
		let mut path = result.unwrap();
		path = path.join("rnote\\.rnote");
		return read_setting( path.to_str().unwrap());
	}
	return read_setting( ".rnote");
}

fn read_setting(path: &str) -> Settings{
	let mut file = match File::open(path) {
		Ok(file) => file,
		Err(..)  => create_setting(path),
	};
	let mut json = String::new();
	file.read_to_string(&mut json).unwrap();
	let settings: Settings =  json::decode(&json).unwrap();
	return settings;
}

fn create_setting(path: &str) -> File {
	//Create file or fail
	let o_path = Path::new( path);
	//Add check to skip dir creation when exists
	if fs::create_dir(o_path.parent().unwrap()).is_err() {
		panic!(format!("Unable to open or create config file at {}", path));
	}
	//Create file
	let mut file = match File::create(path) {
		Ok(file) => file,
		Err(..) => panic!(format!("Unable to open or create config file at {}", path)),
	};
	//Add defaults
	let defaults = Settings { utc:true,id_offset:500, data:".rnotes_data".to_string()};
	file.write_all(json::encode(&defaults).unwrap().into_bytes().borrow()).unwrap();
	//Reopen read only
	file = match File::open(path) {
		Ok(file) => file,
		Err(..)  => panic!(format!("Unable to open or create config file at {}", path)),
	};
	return file;
}


