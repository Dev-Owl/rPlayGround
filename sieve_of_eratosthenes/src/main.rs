use std::io;

fn main() {
    println!("Input max number:");
    let mut max_number = String::new();
    io::stdin().read_line(&mut max_number)
            .ok()
            .expect("failed to read line");
   
   let max_number: u32 = match max_number.trim().parse::<u32>() {
            Ok(num) => num+1,
            Err(_) => panic!("you breaked the yolo # core!"),
        };
	
	let mut list = Vec::new();
	for i in 0..max_number {
		 list.push(true);
	}	
	
	//for i in 2..max_number {
	//	if match list.get(i as usize) { 
	//		Some(v) => v,
	//		None
	//	} == true {
	//		println!("true");
	//	}	
	//}
}
