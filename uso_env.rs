use std::env;




fn main() {
	for argument in env::args(){
		if argument == "--help" {
			println!("pasaste --help como argumento");
		} 
		if argument == "--main" {
			println!("pasaste --main como argumento");
		}
		if argument == "--run" {
			println!("pasaste --run como argumento");
		}

		if argument == "--write" {
			println!("pasaste --write como argumento");
		}

	}
}