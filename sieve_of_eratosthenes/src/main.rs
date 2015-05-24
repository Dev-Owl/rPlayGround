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
	for i in 1..max_number {
		list.push(i);
	}	
	
	for i in 2..10 {
		list.retain( |&n| n%i !=0 || n==i);
	}
	println!("Prime numbers:");
	let mut index = 0;
	for i in list.iter() {
		if index % 10 == 0{
			println!("");
			index = 0;
		}
		print!("{} ",*i);
		index+=1;
    }
    println!("");
}
