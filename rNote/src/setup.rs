use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::json;
use std::borrow::Borrow;
use std::env;
use std::fs;
use std::collections::HashMap;


use setting::Settings;

//Check all file issues here, create/upgrade settings file add data folder

#[cfg(any(unix))]
pub fn get_base_path() -> String {
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
pub fn get_base_path() -> String {
	let result = env::home_dir();
	if !result.is_none(){
		let mut path = result.unwrap();
		path = path.join("rnote\\");
		return path.to_str().unwrap();
	}
	panic!("Unable to get home dir");
}

pub fn env_check(){
	//Check env create folders/files if needed
	create_setting(&get_base_path());
}


fn create_setting(path: &str){
	
	//Check the base folder
	folder_create(path);
	
	//Check the settings files
	let mut tmp_path = path.to_string();
	tmp_path.push_str("settings.json");
	
	if fs::metadata(&tmp_path).is_err() {
		let mut file;
		//Create file
		file = file_create(&tmp_path);
		tmp_path = path.to_string();
		tmp_path.push_str("data/");
		//Add defaults
		let mut tmp_hash = HashMap::new();
		tmp_hash.insert("UTC".to_string(),"1".to_string());
		tmp_hash.insert("id_offset".to_string(),"500".to_string());
		tmp_hash.insert("data".to_string(),tmp_path);
		let defaults = Settings { version: 0, data: tmp_hash};
		file.write_all(json::encode(&defaults).unwrap().into_bytes().borrow()).unwrap();
	}
	tmp_path = path.to_string();
	tmp_path.push_str("data");
	folder_create(&tmp_path);
}

fn folder_create(path: &str){
	if fs::metadata(path).is_err() {
		//Add check to skip dir creation when exists
		if fs::create_dir(path).is_err() {
			panic!(format!("Unable to create folder at {}", path));
		}
	}
}

pub fn file_create(path: &str)->File{
	return match File::create(path) {
			Ok(file) => file,
			Err(..) => panic!(format!("Unable to create config file at {}", path)),
	};
}
