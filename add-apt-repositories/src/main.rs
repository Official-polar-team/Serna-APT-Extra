extern crate unicase;
use std::io::Write;
use unicase::UniCase;
use std::env;
use std::fs::OpenOptions;
use std::process::Command;
use std::process;

fn main(){
	//Collects the args given by the user on at the time of executing the command
	let mut repository_urls: Vec<String> = env::args().collect();
	if repository_urls.len() == 1 {
		eprintln!("You have to enter at least one repository url!");
		process::exit(1);
	} else {
		for i in 0 .. repository_urls.len() - 1 {
			if (UniCase::new(&repository_urls[i + 1]) == UniCase::new("--advanced") && repository_urls.len() >= i + 5) || (UniCase::new(&repository_urls[i + 1]) == UniCase::new("-a") && repository_urls.len() >= i + 5) {
				//If the repo url contains "http://" then replace it with https://
				if repository_urls[i + 2].contains("http://") && !repository_urls[i + 2].contains("https://") {
					repository_urls[i + 2] = repository_urls[i + 2].replace("http://", "https://");
					//If the repo url doesnt contain anything, then replace it with https://, an example "hello" the output would be "https://hello"
				} else if !repository_urls[i + 2].contains("http://") && !repository_urls[i + 2].contains("https://") && !repository_urls[i + 2].contains("fpt://") && !repository_urls[i + 2].contains("sfpt://") {
					repository_urls[i + 2] = "https://".to_owned() + &repository_urls[i + 2];
					//Replaces fpt:// with sfpt://
				} else if repository_urls[i + 2].contains("fpt://") && !repository_urls[i + 2].contains("sfpt://") {
					repository_urls[i + 2] = repository_urls[i + 2].replace("fpt://", "sfpt://");
				}

				//Makes sure there's a / at the end of the link to prevent any issues, have to have an empty if statement here because ! can't be used when checking characters.
				if repository_urls[i + 2].chars().last().unwrap() == '/' {} else {
					repository_urls[i + 2] = repository_urls[i + 2].to_owned() + "/"
				}

				//Open source list, append so everything is a new line, this is why OpenOptions::new is used
				let mut source_list = OpenOptions::new()
					.write(true)
					.append(true)
					//Source list is located here.
					.open("/usr/local/etc/apt/sources.list.d/novus.list")
					.unwrap();
	
				//Basic checks if it was or wasn't able to write to the file.
				if let Err(e) = writeln!(source_list, "{}", "deb ".to_owned() + &repository_urls[i + 2] + " " + &repository_urls[i + 3] + " " + &repository_urls[i + 4]) {
					eprintln!("Couldn't write to file: {}", e);
				};
				//Exxecute curl to grab the key and "|" so we can read the output and mix it with apt-key later
				Command::new("curl").arg("-Os").arg(repository_urls[i + 2].to_owned() + "repokey.asc").arg("|").status().expect("Oof unknown error");
				repository_urls.remove(i + 4);
				repository_urls.remove(i + 3);
				repository_urls.remove(i + 2);
			} else {
				//If the repo url contains "http://" then replace it with https://
				if repository_urls[i + 1].contains("http://") && !repository_urls[i + 1].contains("https://") {
					repository_urls[i + 1] = repository_urls[i + 1].replace("http://", "https://");
					//If the repo url doesnt contain anything, then replace it with https://, an example "hello" the output would be "https://hello"
				} else if !repository_urls[i + 1].contains("http://") && !repository_urls[i + 1].contains("https://") && !repository_urls[i + 1].contains("fpt://") && !repository_urls[i + 1].contains("sfpt://") {
					repository_urls[i + 1] = "https://".to_owned() + &repository_urls[i + 1];
					//Replaces fpt:// with sfpt://
				} else if repository_urls[i + 1].contains("fpt://") && !repository_urls[i + 1].contains("sfpt://") {
					repository_urls[i + 1] = repository_urls[i + 1].replace("fpt://", "sfpt://");
				}

				//Makes sure there's a / at the end of the link to prevent any issues, have to have an empty if statement here because ! can't be used when checking characters.
				if repository_urls[i + 1].chars().last().unwrap() == '/' {} else {
					repository_urls[i + 1] = repository_urls[i + 1].to_owned() + "/"
				}

				//Open source list, append so everything is a new line, this is why OpenOptions::new is used
				let mut source_list = OpenOptions::new()
					.write(true)
					.append(true)
					//Source list is located here.
					.open("/usr/local/etc/apt/sources.list.d/novus.list")
					.unwrap();
	
				//Basic checks if it was or wasn't able to write to the file.
				if let Err(e) = writeln!(source_list, "{}", "deb ".to_owned() + &repository_urls[i + 1] + " ./") {
					eprintln!("Couldn't write to file: {}", e);
				};
				//Exxecute curl to grab the key and "|" so we can read the output and mix it with apt-key later
				Command::new("curl").arg("-Os").arg(repository_urls[i + 1].to_owned() + "repokey.asc").arg("|").status().expect("Oof unknown error");
			}
			//Add the key and do it to dev/null so is silent
			Command::new("sudo").arg("apt-key").arg("add").arg("repokey.asc").status().expect("Oof unknown error");
			//Stops crash from happening when this finishes running.
			if i + 1 >= repository_urls.len() - 1 {
				break;
			}
		}
		//Refreshes repositories with NovusCLI
		Command::new("nvs").arg("update").status().expect("Oof unknown error");
		//Ends the process normally
		process::exit(0);
	}
}
