use std::path::Path;
use std::io::prelude::*;
use std::fmt;
use std::fs;
use rustc_serialize::json;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;

use setup;
use notes::Note;
use setting;


pub struct Note_handler {
    pub data_path: String,
    pub id_offset: u32
}

impl Note_handler{

    pub fn new(note_data_path:String, note_id_offset: u32) -> Note_handler{
        Note_handler{data_path : note_data_path, id_offset: note_id_offset}
    }

    pub fn new_note(&self, new_title: String) ->Note{
          Note::new( self.next_id(), new_title)
    }


    pub fn save_note(&self, note: &Note){
        let mut final_path = self.data_path.clone();
		final_path.push_str("/");
		final_path.push_str(&note.id.to_string());
		let mut file = setup::file_create(&final_path);
		match OpenOptions::new().create(true).write(true).append(false).open(&final_path) {
        	Ok(ref mut file) => {
            	write!(
                	file,
					"{}",
                	json::encode(&note).unwrap()
            	).unwrap();
        	},
        	Err(err) => { panic!("Failed to open log file: {}", err); }
    	}
    }

    pub fn exists_note(&self, id: &u32) ->bool{
        let mut result = false;
		match file_list( Path::new( &self.data_path)){
			Ok(v) => {
					  for x in v.iter()
					  {
						  if match x.parse::<u32>() {
  							Ok(v) => v,
  							error => 0,
  						} == *id {
  							result = true;
  						}
					  }
				  },
		    Err(e) => panic!("Unable to read data directory at {}",self.data_path),
		};
		result
    }

    pub fn load_note(&self, id: &u32) -> Note{
        let mut final_path = self.data_path.clone();
		final_path.push_str("/");
		final_path.push_str(&id.to_string());
		return match json::decode(&setup::file_read(&final_path)){
			Ok(n) => n,
			Err(e) => panic!("decode of json failed:{} for ID: {}", e, id),
		};
    }

    pub fn delete_note(&self, id: &u32){
        let mut final_path = self.data_path.clone();
		final_path.push_str("/");
		final_path.push_str(&id.to_string());
		if ! fs::metadata(&final_path).is_err(){
			if( fs::remove_file(&final_path).is_err())
			{
				panic!("Failed to delete a note at {}",final_path);
			}
		}
		else{
			panic!("Loaded note is not on the harddsik")
		}
    }

    pub fn next_id(&self) -> u32{
        let mut new_id: u32 = 0;
		let mut tmp : u32 = self.id_offset;
		//Get list of files search for next free id
		match file_list( Path::new( &self.data_path)){
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
			error => panic!("Unable to read in data directory at {}", self.data_path),
		};
		new_id += tmp +1;
		new_id
    }

}

//move to file_utily later!
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
fn test_note_create_exists()
{
  let settings = setting::get_config();
  let handler = Note_handler::new(settings.get_default("data","data"),
                                       settings.get_default("id_offset","500").parse::<u32>().unwrap());
  let mut my_note = handler.new_note("Fu bar".to_string());
  my_note.add_tag("Testing".to_string());
  handler.save_note(&my_note);
  assert_eq!( handler.exists_note(&my_note.id),true);
  handler.delete_note(&my_note.id);
}

#[test]
fn test_note_load(){
    let settings = setting::get_config();
    let handler = Note_handler::new(settings.get_default("data","data"),
                                         settings.get_default("id_offset","500").parse::<u32>().unwrap());
    let mut my_note = handler.new_note("Fu bar".to_string());
    my_note.add_tag("Testing".to_string());
    handler.save_note(&my_note);
    assert_eq!( handler.exists_note(&my_note.id),true);
	let mut loaded_note = handler.load_note(&my_note.id);
	handler.delete_note(&loaded_note.id);
}
