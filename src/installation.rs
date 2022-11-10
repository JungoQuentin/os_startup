use crate::utils::inst;
use crate::Distro;

use colored::Colorize;
use std::process::exit;
use std::process::{self, Command};

pub fn default_installation(distro: Distro) {
    inst_git(&distro);
    inst_nvim(&distro);
    inst_zsh(&distro);
    if !inst(&distro, "gh").success() {
        inst(&distro, "github-cli");
    }
    inst(&distro, "lldb");
    inst(&distro, "curl");
    inst_rust();
}

pub fn custom_installation(distro: Distro) {
    todo!("custom to finish !!!");
}

fn inst_rust() {
    Command::new("curl")
        .arg("--proto '=https'")
        .arg("--tlsv1.2")
        .arg("-sSf https://sh.rustup.rs | sh")
        .output()
        .expect("get la last v de nvim");
}

fn inst_nvim(distro: &Distro) {
    if matches!(distro, Distro::Debian) {
        Command::new("add-apt-repository")
            .arg("ppa:neovim-ppa/stable")
            .output()
            .expect("get la last v de nvim");
        Command::new("apt-get")
            .arg("update")
            .output()
            .expect("Il ne veut pas supprimer la config nvim");
    }
    inst(distro, "neovim");
    Command::new("rm")
        .arg("-rf")
        .arg("~/.config/nvim")
        .output()
        .expect("Il ne veut pas supprimer la config nvim");
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/QuentinPoto/nvim_config.git")
        .arg("~/.config/nvim")
        .output()
        .expect("Il ne veut pas git clone ma config nvim");
    Command::new("git")
        .arg("clone")
        .arg("--depth 1")
        .arg("https://github.com/wbthomason/packer.nvim")
        .arg("~/.local/share/nvim/site/pack/packer/start/packer.nvim")
        .output()
        .expect("Il ne veut pas git clone ma config nvim");
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/github/copilot.vim")
        .arg("~/.config/nvim/pack/github/start/copilot.vim")
        .output()
        .expect("Il ne veut pas git clone packer");
}

fn inst_zsh(distro: &Distro) {
    inst(distro, "zsh");
    // TODO : zsh en main
}
fn inst_git(distro: &Distro) {
    inst(distro, "git");
    // TODO set username et email...
    /*
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/github/copilot.vim")
        .arg("~/.config/nvim/pack/github/start/copilot.vim")
        .output()
        .expect("Il ne veut pas git clone packer");
    */
}
