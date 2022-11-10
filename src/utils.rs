use crate::Distro;
use colored::Colorize;
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
    let cmd = Command::new(match distro {
        Distro::Arch => "pacman",
        Distro::Debian => "apt-get",
    })
    .arg(match distro {
        Distro::Arch => "-Sy",
        Distro::Debian => "install",
    })
    .arg(software)
    .output()
    .expect("A problem occur when...");
    if cmd.status.success() {
        println!(
            "{}{}\n",
            std::str::from_utf8(&cmd.stdout).expect(""),
            format!("{software} installed !").bold().green(),
        );
    } else {
        println!(
            "{}\n{}",
            format!("L'erreur suivante est survenue :").bold().red(),
            std::str::from_utf8(&cmd.stderr).expect("")
        );
        exit(1);
    }
    // TODO enlever le return ??
    return cmd.status;
}
