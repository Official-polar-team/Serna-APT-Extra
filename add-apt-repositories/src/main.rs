use std::io::Write;
use std::env;
use std::fs::OpenOptions;
use std::process::Command;
use std::process;
mod common;
mod advanced;

fn main(){
	//Collects the args given by the user on at the time of executing the command
	let mut repository_urls: Vec<String> = env::args().collect();
	if repository_urls.len() == 1 {
		eprintln!("You have to enter at least one repository url!");
		process::exit(1);
	} else if repository_urls.len() == 2 {
		//If its equal to two, that means not advanced arg was used so call the common repo method.
		common::add_repo();
	} else {
		//If it doesnt contain advanced then just execute the normal method
		if repository_urls[2] != "--advanced" {
			common::add_repo();
		} else if repository_urls[2] == "--advanced" {
		//Execute advanced repo function since the user decided so.	
			advanced::add_repo();
		}
	}
}