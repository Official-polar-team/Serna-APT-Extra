use std::io::Write;
use std::env;
use std::fs::OpenOptions;
use std::process::Command;
use std::process;

fn main(){
    let repo_url: Vec<String> = env::args().collect();

    let data_to_add = "deb ".to_owned() + &repo_url[1] + " ./";

    let mut source_list = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/usr/local/etc/apt/sources.list.d/novus.list")
        .unwrap();

    if let Err(e) = writeln!(source_list, "{}", data_to_add) {
        eprintln!("Couldn't write to file: {}", e);
    };

    let repo_string = String::from(&repo_url[1]);
    let repo_key_dir = repo_string.to_owned() + "repoKey";
    Command::new("curl").arg("-Os").arg(repo_key_dir).arg("|");
    Command::new("sudo").arg("apt-key").arg("add").arg("repokey.asc").arg(">/dev/null 2>&1").status();
    process::exit(0);

}
