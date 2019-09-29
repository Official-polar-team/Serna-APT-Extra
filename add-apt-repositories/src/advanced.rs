extern crate rprompt;
use std::io::Write;
use std::io;
use std::env;
use std::fs::OpenOptions;
use std::process::Command;
use std::process;



pub fn add_repo() {
    let mut repository_urls: Vec<String> = env::args().collect();
    //This is so user realizes he he did an advanced repository 
	println!("You entered and advanced repository. This means you have extra input that was supplied to you.");
    
	//This are the prompts
    let mut codename: String = rprompt::prompt_reply_stderr("Codename: ").unwrap();
    let mut component: String = rprompt::prompt_reply_stderr("Component: ").unwrap();

    if repository_urls[1].contains("http://") && !repository_urls[1].contains("https://") {
				repository_urls[1] = repository_urls[1].replace("http://", "https://");
				//If the repo url doesnt contain anything, then replace it with https://, an example "hello" the output would be "https://hello"
			} else if !repository_urls[1].contains("http://") && !repository_urls[1].contains("https://") && !repository_urls[1].contains("fpt://") && !repository_urls[1].contains("sfpt://") {
				repository_urls[1] = "https://".to_owned() + &repository_urls[1];
				//Replaces fpt:// with sfpt://
			} else if repository_urls[1].contains("fpt://") && !repository_urls[1].contains("sfpt://") {
				repository_urls[1] = repository_urls[1].replace("fpt://", "sfpt://");
			}

			//Makes sure there's a / at the end of the link to prevent any issues, have to have an empty if statement here because ! can't be used when checking characters.
			if repository_urls[1].chars().last().unwrap() == '/' {} else {
				repository_urls[1] = repository_urls[1].to_owned() + "/"
			}

            //Open source list, append so everything is a new line, this is why OpenOptions::new is used
			let mut source_list = OpenOptions::new()
				.write(true)
				.append(true)
				//Source list is located here.
				.open("/usr/local/etc/apt/sources.list.d/novus.list")
				.unwrap();
       
	   //Write to the file but also, add the white spaces to it works.
       if let Err(e) = writeln!(source_list, "{}", "deb ".to_owned() + &repository_urls[1] + &" ".to_owned() + &codename + &" ".to_owned() + &component) {
				eprintln!("Couldn't write to file: {}", e);
			};

            Command::new("curl").arg("-Os").arg(repository_urls[1].to_owned() + "repokey.asc").arg("|").status().expect("Oof unknown error");
			//Add the key and do it to dev/null so is silent
			Command::new("sudo").arg("apt-key").arg("add").arg("repokey.asc").status().expect("Oof unknown error");
		//Refreshes repositories with NovusCLI
		Command::new("nvs").arg("update").status().expect("Oof unknown error");
		//Ends the process normally
		process::exit(0);
} 