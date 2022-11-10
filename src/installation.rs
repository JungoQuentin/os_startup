use crate::utils::{bonus_inst,manual_install, inst, print_res};
use crate::{Distro, Infos};
use std::process::Command;

pub fn default_installation(distro: Distro) {
    let info = Infos {
        distro: &distro,
        username: String::new(),
    };
    inst_git(&distro);
    inst(&distro, "nodejs");
    inst_nvim(&info);
    inst_zsh(&distro);
    if !inst(&distro, "gh").success() {
        inst(&distro, "github-cli");
    }
    inst(&distro, "lldb");
    inst(&distro, "curl");
    inst_rust();

    // Bonus
    bonus_inst(&distro, "godot");
    while manual_install(&distro) {}
}

pub fn custom_installation(distro: Distro) {
    todo!("custom to finish !!! {:?}", distro);
}

fn inst_rust() {
    let cmd = Command::new("sudo")
        .arg("curl")
        .arg("--proto '=https'")
        .arg("--tlsv1.2")
        .arg("-sSf https://sh.rustup.rs | sh")
        .output()
        .expect("Rust Install fail");
    print_res(&cmd, "Rust installed !");
}

fn inst_nvim(infos: &Infos) {
    if matches!(infos.distro, Distro::Debian) {
        let cmd = Command::new("sudo")
            .arg("add-apt-repository")
            .arg("-y")
            .arg("ppa:neovim-ppa/stable")
            .output()
            .expect("get la last v de nvim");
        println!("{}", std::str::from_utf8(&cmd.stdout).expect(""));
        print_res(&cmd, "nvim getted from last version");
        let cmd = Command::new("sudo")
            .arg("apt-get")
            .arg("-y")
            .arg("update")
            .output()
            .expect("Il ne veut pas supprimer la config nvim");
        println!("{}", std::str::from_utf8(&cmd.stdout).expect(""));
        print_res(&cmd, "updated");
    }
    inst(&infos.distro, "neovim");
    todo!("le home veut pas.."
    let _cmd = Command::new("rm")
        .arg("-rf")
        .arg("$(HOME)/.config/nvim")
        .output()
        .expect("Il ne veut pas supprimer la config nvim");
    let cmd = Command::new("git")
        .arg("clone")
        .arg("https://github.com/QuentinPoto/nvim_config.git")
        .arg("$(HOME)/.config/nvim")
        .output()
        .expect("Il ne veut pas git clone ma config nvim");
    print_res(&cmd, "Git config cloned");
    let cmd = Command::new("git")
        .arg("clone")
        .arg("--depth 1")
        .arg("https://github.com/wbthomason/packer.nvim")
        .arg("$(HOME)/.local/share/nvim/site/pack/packer/start/packer.nvim")
        .output()
        .expect("Il ne veut pas git clone ma config nvim");
    print_res(&cmd, "Packer installed");
    let cmd = Command::new("git")
        .arg("clone")
        .arg("https://github.com/github/copilot.vim")
        .arg("$(HOME)/.config/nvim/pack/github/start/copilot.vim")
        .output()
        .expect("Il ne veut pas git clone packer");
    print_res(&cmd, "Github copilot installed");
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
