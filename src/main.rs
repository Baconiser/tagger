use std::{fs, io};

use std::path::{Path, PathBuf};

use powershell_script::run;

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
    let home_dir = dirs::home_dir().unwrap();
    let home_dir_tags_path_buf = home_dir.join(&TAG_FOLDER);
    return home_dir_tags_path_buf;
}


fn add_to_quickaccess() {
    let dir = get_tag_dir();
    let command = format!("$o = new-object -com shell.application; \
    $o.Namespace('{}').Self.InvokeVerb(\"pintohome\")", dir.display());
    /*
        let mut process = Command::new("powershell")
            .args(["-Command", &*command]);*/
    match run(&*command, false) {
        Ok(_output) => {
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

    let folder_name = &splitted[length - 1];
    println!("Target name: {}", folder_name);
    println!("Target path: {}", path);
    println!("Please enter Tag name:");
    let tag_name = get_input().to_lowercase();

    let home_dir = dirs::home_dir().unwrap();
    let home_dir_tags_path_buf = home_dir.join(&TAG_FOLDER);
    let tag_folder = home_dir_tags_path_buf.as_path();
    let target_path = Path::new(&path);
    let tag_target_path_buf = tag_folder.join(&tag_name);
    let tag_target = tag_target_path_buf.as_path();
    let link_file_name = format!("{}", &folder_name);

    let error_option = fs::create_dir_all(tag_target).err();
    if error_option.is_some() {
        let error = error_option.unwrap();
        panic!("{}", error);
    }


    add_to_quickaccess();
    let buf = create_link(&tag_target, &link_file_name);
    if target_path.is_dir() {
        let result = std::os::windows::fs::symlink_dir(&target_path, &buf);
        println!("{:?}", result.err());
    }
    if target_path.is_file() {
        let result = std::os::windows::fs::symlink_file(&target_path, &buf);
        println!("{:?}", result.err());
    }
}

fn create_link(tag_target: &Path, link_name: &String) -> PathBuf {
    let link_path_orig = tag_target.join(link_name);
    let mut link_path = link_path_orig.clone();

    let mut iterator = 1;
    while link_path.exists() {
        let name = format!("{}{}", link_name, format!("-{}", iterator));
        link_path = tag_target.join(name);
        iterator += 1;
    }

    return link_path;
}

