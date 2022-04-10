use std::{
    process::Command, // Importando a biblioteca para a execução de comandos do sistema operacional
};

pub fn install_and_configure_utils_on_archlinux_to_user_pedro() {

    Command::new("sudo")
        .args(Some("pacman"))
        .args(Some("-Syu"))
        .args(Some("xorg"))
        .args(Some("networkmanager"))
        .args(Some("gvfs-mtp"))
        .args(Some("gvfs-goa"))
        .args(Some("gvfs-google"))
        .args(Some("exfat-utils"))
        .args(Some("p7zip"))
        .args(Some("firefox"))
        .args(Some("zip"))
        .args(Some("unzip"))
        .args(Some("unrar"))
        .args(Some("system-config-printer"))
        .args(Some("adwaita-icon-theme"))
        .args(Some("xf86-video-intel"))
        .args(Some("libgl"))
        .args(Some("mesa"))
        .args(Some("nvidia"))
        .args(Some("nvidia-libgl"))
        .args(Some("xf86-video-amdgpu"))
        .args(Some("docker"))
        .args(Some("qemu"))
        .args(Some("ovmf"))
        .args(Some("neovim"))
        .args(Some("git"))
        .args(Some("xdg-user-dirs"))
        .args(Some("bluez"))
        .status()
        .expect("Erro ao instalar os utilitarios do sistema");
    
    Command::new("git")
        .args(Some("clone"))
        .args(Some("https://aur.archlinux.org/epson-inkjet-printer-escpr.git"))
        .status()
        .expect("Erro ao clonar o repositorio do driver da impressora");
    
    Command::new("cd")
        .args(Some("epson-inkjet-printer-escpr/"))
        .status()
        .expect("Erro ao entrar na pasta do driver da impressora");
    
    Command::new("makepkg")
        .args(Some("-sicr"))
        .status()
        .expect("Erro ao compilar o driver da impressora");
    
    Command::new("cd")
        .args(Some(".."))
        .status()
        .expect("Erro ao sair do diretorio do driver da impressora");
    
    Command::new("rm")
        .args(Some("-rf"))
        .args(Some("epson-inkjet-printer-escpr/"))
        .status()
        .expect("Erro ao remover a pasta: epson-inkjet-printer-escpr/");
    
    Command::new("sudo")
        .args(Some("pacman"))
        .args(Some("-Syu"))
        .args(Some("cups"))
        .args(Some("--noconfirm"))
        .status()
        .expect("Erro ao instalar o cups");
    
    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("cups"))
        .args(Some("-f"))
        .status()
        .expect("Erro ao habilitar o deamon do cups na inicialização do sistema");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("start"))
        .args(Some("cups"))
        .args(Some("-f"))
        .status()
        .expect("Erro ao startar o deamon do cups agora");

    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("start"))
        .args(Some("bluetooth"))
        .args(Some("-f"))
        .status()
        .expect("Erro ao startar o deamon do bluetooth agora");
    
    Command::new("sudo")
        .args(Some("systemctl"))
        .args(Some("enable"))
        .args(Some("bluetooth"))
        .args(Some("-f"))
        .status()
        .expect("Erro ao habilitar o deamon do bluetooth na inicialização do sistema");

}