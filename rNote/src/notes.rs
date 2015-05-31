use time;

#[derive(Default)]
pub struct Note{
	pub title: String,
	pub tag: Vec<String>,
	pub last_update: u32,
	pub creation: u32,
}


impl Note{
	
	pub fn new() -> Note{
		Note {creation: unix_timestamp(), ..Default::default() }
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
	
}

pub fn unix_timestamp() -> u32{
	time::now_utc().to_timespec().sec as u32
}


