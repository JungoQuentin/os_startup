use crate::Distro;
use colored::Colorize;
use question::{Answer, Question};
use std::iter;
use std::process::{self, exit, Command};

pub fn print_welcome() {
    const MSG: &str = "Welcome to the myself, have fun re-install your own config";
    const PADDING: usize = 6;
    let left: String = String::from("*  ");

    let right: String = left.chars().rev().collect::<String>();
    let mut welcome = left.clone();
    let mut line = left.clone();
    let mut blank_line = left.clone();
    let middle: String = iter::repeat('-')
        .take(MSG.len() + PADDING)
        .collect::<String>();
    let blank_middle: String = iter::repeat(' ')
        .take(MSG.len() + PADDING)
        .collect::<String>();

    welcome.push_str(&iter::repeat(' ').take(PADDING / 2).collect::<String>());
    welcome.push_str(MSG);
    welcome.push_str(&iter::repeat(' ').take(PADDING / 2).collect::<String>());
    welcome.push_str(&right);
    line.push_str(&middle);
    line.push_str(&right);
    blank_line.push_str(&blank_middle);
    blank_line.push_str(&right);

    let final_msg = format!(
        "\n\n{}\n{}\n{}\n{}\n{}\n\n",
        line, blank_line, welcome, blank_line, line
    );
    println!("{}", final_msg.bold().green());
}

pub fn inst(distro: &Distro, software: &str) -> process::ExitStatus {
    let cmd = Command::new("sudo")
        .args(match distro {
            Distro::Arch => ["pacman", "-Sy", "--noconfirm", ""],
            Distro::Debian => ["DEBIAN_FRONTEND=noninteractive", "apt-get", "install", "-y"],
        })
        .arg(software)
        .output()
        .expect("A problem occur when...");
    // enlever ??
    println!("{}", std::str::from_utf8(&cmd.stdout).expect(""));
    print_res(&cmd, format!("{software} installed !").as_str());
    return cmd.status;
}

pub fn bonus_inst(distro: &Distro, software: &str) {
    match Question::new(format!("Do you want to install {software} ?").as_str()).confirm() {
        Answer::YES => {
            inst(&distro, software);
        }
        _ => {}
    }
}

pub fn manual_install(distro: &Distro) -> bool {
    if matches!(
        Question::new("Do you want to type an other software to install ?").confirm(),
        Answer::NO
    ) {
        return false;
    }
    let software = Question::new("type the name of the distro")
        .ask()
        .expect("fallait juste donner un nom..");

    let software = match software {
        Answer::RESPONSE(r) => r,
        _ => "r".to_string(),
    };
    inst(distro, &software);
    true
}

pub fn print_res(cmd: &std::process::Output, success_msg: &str) {
    if cmd.status.success() {
        println!(
            "{}{}\n",
            std::str::from_utf8(&cmd.stdout).expect(""),
            format!("{success_msg}").bold().green(),
        );
    } else {
        println!(
            "{}\n{}",
            format!("L'erreur suivante est survenue :").bold().red(),
            std::str::from_utf8(&cmd.stderr).expect("")
        );
        exit(1);
    }
}
