
use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::json;
use std::collections::HashMap;
use setup;

//Allow creation of JSON and creation from JSON
#[derive(RustcDecodable, RustcEncodable)]
pub struct Settings{
 	pub version: u32,
	pub data: HashMap<String,String>,
}

impl Settings{

	pub fn get_default(&self,key: &str ,default: &str) -> String {
		match self.data.get( key) {
			Some(v) => v.to_string(),
			None => default.to_string(),
		}
	}
	pub fn get(&self, key: &str) -> Option<&String> {
		self.data.get( key)
	}
}

pub fn get_config() -> Settings{
	return read_setting(&setup::get_base_path());
}

fn read_setting(path: &str) -> Settings{
    let mut tmp_path = path.to_string();
	tmp_path.push_str("settings.json");
	let mut file = match File::open(tmp_path) {
		Ok(file) => file,
		Err(..)  => panic!(format!("Unable to open the config at {}",path)),
	};
	let mut json = String::new();
	file.read_to_string(&mut json).unwrap();
	let settings: Settings =  json::decode(&json).unwrap();
	return settings;
}

#[test]
fn test_get_config()
{
   let settings = get_config();
}
