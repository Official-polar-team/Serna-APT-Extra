use std::io::Write;
use std::env;
use std::fs::OpenOptions;
use std::process::Command;
use std::process;

fn main(){
    //Collects the args given by the user on at the time of executing the command
    let mut repository_urls: Vec<String> = env::args().collect();

    //If the repo url contains "http://" then replace it with https://
    if repository_urls[1].contains("http://") && !repository_urls[1].contains("https://") {
    //carreplace_http_with_https
    //If the repo url doesnt contain anything, then replace it with http://, an example "hello" the output would be "https://hello"
    } else if !repository_urls[1].contains("http://") && !repository_urls[1].contains("https://") {
    repository_urls[1] = "https://".to_owned() + &repository_urls[1];
    }

    //Adds the word "deb" then grabs the repo url and ads "./" at the end.
    let data_to_add = "deb ".to_owned() + &repository_urls[1] + " ./";

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
    
    //Convert the repo url to a string here, Rust doesn't like convertig vectors to owned
    let repo_string = String::from(&repository_urls[1]);
    //Repo key dir, takes the string of the repo and converts it to owned, them combines it with the "repokey"
    let repo_key_dir = repo_string.to_owned() + "repokey.asc";
    //Exxecute curl to grab the key and "|" so we can read the output and mix it with apt-key later
    Command::new("curl").arg("-Os").arg(repo_key_dir).arg("|");
    //Add the key and do it to dev/null so is silent
    Command::new("sudo").arg("apt-key").arg("add").arg("repokey.asc").arg(">/dev/null 2>&1");
    //End the process 
    process::exit(0);

}
