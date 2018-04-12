use std::env;
// use std::error::Error;
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

fn main() {

	let args: Vec<String> = env::args().collect();
	for i in &args {
		println!("{}", i);
	}
	// println!("{:?}", args);

    // let path = Path::new("/Users/matthew.downs/Downloads/tag.txt");
    // let display = path.display();

    // let mut file = match File::open(&path) {
    // 	Err(why) => panic!("couldn't open {}: {}", display, why.description()),
    // 	Ok(file) => file,
    // };

    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    // 	Err(why) => panic!("couldn't read {}: {}", display, why.description()),
    // 	Ok(_) => print!("{} contains:\n{}", display, s),
    // }


}
