use std::io::Write;
use std::env;
use std::fs::OpenOptions;
use std::process::Command;
use std::process;

fn main(){
    //Collects the args given by the user on at the time of executing the command
    let mut repository_urls: Vec<String> = env::args().collect();
    for i in 0 .. repository_urls.len() - 1 {
    	//If the repo url contains "http://" then replace it with https://
	    if repository_urls[i + 1].contains("http://") && !repository_urls[i + 1].contains("https://") {
    	repository_urls[i + 1] = repository_urls[i + 1].replace("http://", "https://");
	    //If the repo url doesnt contain anything, then replace it with https://, an example "hello" the output would be "https://hello"
    	} else if !repository_urls[i + 1].contains("http://") && !repository_urls[i + 1].contains("https://") && !repository_urls[i + 1].contains("fpt://") && !repository_urls[i + 1].contains("sfpt://") {
    	repository_urls[i + 1] = "https://".to_owned() + &repository_urls[i + 1];
    	//Replaces fpt:// with sfpt://
    	} else if repository_urls[i + 1].contains("fpt://") && !repository_urls[i + 1].contains("sfpt://") {
    		repository_urls[i + 1] = "sfpt://".to_owned() + &repository_urls[i + 1];
    	}

    	//Makes sure there's a / at the end of the link to prevent any issues, have to have an empty if statement here because ! can't be used when checking characters.
    	if repository_urls[i + 1].chars().last().unwrap() == '/' {

    	} else {
    		repository_urls[i + 1] = repository_urls[i + 1].to_owned() + "/"
    	}

	    //Adds the word "deb" then grabs the repo url and ads "./" at the end.
    	let data_to_add = "deb ".to_owned() + &repository_urls[i + 1] + " ./";

	    //Open source list, append so everything is a new line, this is why OpenOptions::new is used
    	let mut source_list = OpenOptions::new()
        	.write(true)
	        .append(true)
    	    //Source list is located here.
        	.open("/usr/local/etc/apt/sources.list.d/novus.list")
	        .unwrap();
    
    	//Basic checks if it was or wasn't able to write to the file.
    	if let Err(e) = writeln!(source_list, "{}", data_to_add) {
    	    eprintln!("Couldn't write to file: {}", e);
	    };
    	//Exxecute curl to grab the key and "|" so we can read the output and mix it with apt-key later
    	Command::new("curl").arg("-Os").arg(repository_urls[i + 1].to_owned() + "repokey.asc").arg("|").status().expect("Oof unknown error");
    	//Add the key and do it to dev/null so is silent
    	Command::new("sudo").arg("apt-key").arg("add").arg("repokey.asc").arg(">/dev/null 2>&1").status().expect("Oof unknown error");
    }
    //Ends the process
    process::exit(0);
}
