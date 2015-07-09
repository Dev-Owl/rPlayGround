use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::json;
use std::borrow::Borrow;
use std::env;
use std::path::Path;
use std::fs;
use std::collections::HashMap;

use setting::Settings;

//Check all file issues here, create/upgrade settings file add data folder

#[cfg(any(unix))]
pub fn get_basePath() -> String {
	let result = env::home_dir();
	let mut path;
	
	if !result.is_none(){
		path = result.unwrap();
		path = path.join("rnote/");
		let mut s = String::new();
		s.push_str(path.to_str().unwrap());
		return s;
	}
	panic!("Unable to get home dir");
}

#[cfg(any(windows))]
pub fn get_basePath() -> String {
	let result = env::home_dir();
	if !result.is_none(){
		let mut path = result.unwrap();
		path = path.join("rnote\\");
		return path.to_str().unwrap();
	}
	panic!("Unable to get home dir");
}

pub fn env_check(){
	

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
	let mut tmp_hash = HashMap::new();
	tmp_hash.insert("UTC".to_string(),"1".to_string());
	tmp_hash.insert("id_offset".to_string(),"500".to_string());
	tmp_hash.insert("data".to_string(),"data".to_string());
	let defaults = Settings { version: 0, data: tmp_hash};
	file.write_all(json::encode(&defaults).unwrap().into_bytes().borrow()).unwrap();
	//Reopen read only
	file = match File::open(path) {
		Ok(file) => file,
		Err(..)  => panic!(format!("Unable to open or create config file at {}", path)),
	};
	return file;
}
