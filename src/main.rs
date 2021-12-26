use std::{fs, io};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use lnk::ShellLink;
use powershell_script::run;
use structopt::clap::Shell;
use structopt::StructOpt;
use structopt_derive::StructOpt;

const TAG_FOLDER: &str = "tags";

#[derive(Debug, StructOpt)]
#[structopt(name = "Tagger", about = "Tagger tool")]
struct Opt {
    #[structopt(short, long)]
    path: String,
}


fn get_input() -> String {
    let mut this_input = String::from("");

    io::stdin().read_line(&mut this_input).expect("Failed to read line");

    this_input.trim().to_string()
}

fn get_tag_dir() -> PathBuf {
    let homeDir = dirs::home_dir().unwrap();
    let homeDirTagsPathBuf = homeDir.join(&TAG_FOLDER);
    return homeDirTagsPathBuf;
}


fn add_to_quickaccess() {
    let dir = get_tag_dir();
    let command = format!("$o = new-object -com shell.application; \
    $o.Namespace('{}').Self.InvokeVerb(\"pintohome\")", dir.display());
    /*
        let mut process = Command::new("powershell")
            .args(["-Command", &*command]);*/
    match run(&*command, false) {
        Ok(output) => {
            //println!("{}", output);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn main() {
    let opt = Opt::from_args();

    let path: String = opt.path;
    let splitted: Vec<String> = path.split("\\").map(|x| x.to_string()).collect();
    let length = splitted.len();

    let folderName = &splitted[length - 1];
    println!("Target name: {}", folderName);
    println!("Target path: {}", path);
    println!("Please enter Tag name:");
    let tagName = get_input().to_lowercase();

    let homeDir = dirs::home_dir().unwrap();
    let homeDirTagsPathBuf = homeDir.join(&TAG_FOLDER);
    let tagFolder = homeDirTagsPathBuf.as_path();
    let targetPath = Path::new(&path);
    let tagTargetPathBuf = tagFolder.join(&tagName);
    let mut tagTarget = tagTargetPathBuf.as_path();


    let linkFileName = format!("{}", &folderName);
    let linkPathBuf = tagTarget.join(&linkFileName);
    let linkPath = linkPathBuf.as_path();
    fs::create_dir_all(tagTarget);
    add_to_quickaccess();
    let buf = create_link(&tagTarget, &linkFileName);
    if targetPath.is_dir() {
        let result = std::os::windows::fs::symlink_dir(&targetPath, &buf);
        println!("{:?}", result.err());
    }
    if targetPath.is_file() {
        let result = std::os::windows::fs::symlink_file(&targetPath, &buf);
        println!("{:?}", result.err());
    }
}

fn create_link(tagTarget: &Path, linkName: &String) -> PathBuf {
    let linkPathOrig = tagTarget.join(linkName);
    let mut linkPath = linkPathOrig.clone();

    let mut iterator = 1;
    while linkPath.exists() {
        let name = format!("{}{}", linkName, format!("-{}", iterator));
        linkPath = tagTarget.join(name);
        iterator += 1;
    }

    return linkPath;
}

