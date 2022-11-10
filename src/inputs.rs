//use inquire::{MultiSelect, Select, Text};
use crate::{Distro, InstallationType};
use inquire::Select;
use std::process::Command;

pub fn get_sudo() {
    let _cmd = Command::new("sudo")
        .arg("/usr/bin/id")
        .status()
        .unwrap()
        .success();
}

pub fn get_installation_type(distro: &Distro) -> InstallationType {
    return Select::new(
        &format!("Welcome msg on {:?}", distro),
        vec![InstallationType::Default, InstallationType::Custom],
    )
    .prompt()
    .expect("You need to choose your installation type !");
}

pub fn get_distro() -> Option<Distro> {
    let pacman = Command::new("sh")
        .arg("pacman -v")
        .output()
        .expect("A problem occur when...");
    if !std::str::from_utf8(&pacman.stdout).expect("").is_empty() {
        return Some(Distro::Arch);
    }
    let pacman = Command::new("sh")
        .arg("-c")
        .arg("apt -v")
        .output()
        .expect("A problem occur when...");
    if !std::str::from_utf8(&pacman.stdout).expect("").is_empty() {
        return Some(Distro::Debian);
    }
    None
}

/*
pub fn project_name() -> String {
    Text::new("Let's begin, what is the name of your project ?")
        .prompt()
        .expect("You need a project name")
}

pub fn project_directory(project_name: &String) -> String {
    Select::new(
        &format!("Where do you want {} to be in ?", project_name),
        vec![
            get_current_working_dir(),
            format!("{}/{}", get_current_working_dir(), project_name),
        ],
    )
    .prompt()
    .expect("You need a directory")
}

pub fn project_modules() -> Option<Modules> {
    const MAKEFILE: &str = "Makefile";
    const FILES: &str = "Some c files";
    const GIT: &str = "GitInit";

    let starter_options = vec![MAKEFILE, FILES, GIT];
    match MultiSelect::new("What do you want to initialize ?", starter_options).prompt() {
        Ok(a) => {
            let mut module = Modules {
                git: None,
                files: None,
                makefile: None,
            };
            if a.contains(&MAKEFILE) {
                module.makefile = Some(Makefile::new());
            }
            if a.contains(&FILES) {
                module.files = Some(FilesInit {});
            }
            if a.contains(&GIT) {
                module.git = Some(GitInit {});
            }
            Some(module)
        }
        Err(_) => None,
    }
}


*/
