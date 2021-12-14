use std::process::Command;
use std::env;
mod texts;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let option = &args[1].trim();

    match &option[..] {

        "--install-fedora-root" => {
                
            lib::texto(texts::DNF, "/etc/dnf/dnf.conf");

            Command::new("dnf")
                .args(Some("update"))
                .args(Some("-y"))
                .status()
                .expect("Erro ao atualizar o DNF");

            Command::new("dnf")
                .args(Some("install"))
                .args(Some("rpmfusion-free-release-35.noarch.rpm"))
                .args(Some("rpmfusion-nonfree-release-35.noarch.rpm"))
                .status()
                .expect("Erro ao instalar o rpmfusion");

            Command::new("dnf")
                .args(Some("install"))
                .args(Some("lsb"))
                .args(Some("cups"))
                .args(Some("simple-scan"))
                .args(Some("ghostscript"))
                .args(Some("gutenprint"))
                .status()
                .expect("Erro ao instalar as dependencias da impressora");

            Command::new("dnf")
                .args(Some("install"))
                .args(Some("epson-inkjet-printer-escpr-1.7.17-1lsb3.2.x86_64.rpm"))
                .args(Some("epson-printer-utility-1.1.1-1lsb3.2.x86_64.rpm"))
                .status()
                .expect("Erro ao instalar os drivers da impressora Epson L3150");

            Command::new("./epsonscan2-bundle-6.6.40.0.x86_64/install.sh")
                .status()
                .expect("Erro ao instalar o epsonscan2");

            Command::new("rpm")
                .args(Some("--import"))
                .args(Some("https://packages.microsoft.com/keys/microsoft.asc"))
                .status()
                .expect("Erro ao baixar a key do vscode");

            lib::texto(texts::VSCODE, "/etc/yum.repos.d/vscode.repo");
            
            Command::new("dnf")
                .args(Some("config-manager"))
                .args(Some("--add-repo"))
                .args(Some("https://dl.winehq.org/wine-builds/fedora/35/winehq.repo"))
                .status()
                .expect("Erro ao adicionar o repositorio do wine");

            Command::new("dnf")
                .args(Some("update"))
                .args(Some("-y"))
                .status()
                .expect("Erro ao atualizar o sistema");

            Command::new("dnf")
                .args(Some("install"))
                .args(Some("code"))
                .args(Some("winehq-staging"))
                .args(Some("telegram-desktop"))
                .args(Some("gnome-system-monitor"))
                .args(Some("gnome-disk-utility"))
                .args(Some("gnome-screenshot"))
                .args(Some("fedora-workstation-backgrounds"))
                .args(Some("gnome-tweaks"))
                .args(Some("gnome-extensions-app"))
                .args(Some("gvfs-mtp"))
                .args(Some("gvfs-goa"))
                .args(Some("unrar"))
                .args(Some("p7zip"))
                .args(Some("libCg"))
                .args(Some("libGL"))
                .args(Some("firefox"))
                .args(Some("gnome-music"))
                .args(Some("gimp"))
                .args(Some("zsh"))
                .args(Some("keepassxc"))
                .args(Some("lutris"))
                .args(Some("steam"))
                .args(Some("evince"))
                .args(Some("eog"))
                .args(Some("totem"))
                .args(Some("neovim"))
                .args(Some("git"))
                .args(Some("cmake"))
                .args(Some("zsh"))
                .args(Some("docker"))
                .args(Some("figlet"))
                .args(Some("gdm"))
                .args(Some("gnome-shell"))
                .args(Some("gnome-terminal"))
                .args(Some("fedora-workstation-backgrounds"))
                .args(Some("file-roller"))
                .args(Some("nautilus"))
                .args(Some("gnome-terminal-nautilus"))
                .args(Some("NetworkManager"))
                .args(Some("@base-x"))
                .args(Some("curl"))
                .args(Some("wget"))
                .args(Some("qemu-kvm"))
                .args(Some("qemu"))
                .args(Some("libvirt"))
                .args(Some("-y"))
                .status()
                .expect("Erro ao instalar meus apps");

            Command::new("usermod")
                .args(Some("-aG"))
                .args(Some("libvirt"))
                .args(Some("pedro"))
                .status()
                .expect("Erro ao adicionar o usuario pedro ao grupo libvirt");

            Command::new("usermod")
                .args(Some("-aG"))
                .args(Some("kvm"))
                .args(Some("pedro"))
                .status()
                .expect("Erro ao adicionar o usuario pedro ao grupo kvm");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("docker"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o docker na inicialização");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("NetworkManager"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao iniciar o NetworkManager na inicialização");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("gdm"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o gdm na inicialização");

            Command::new("systemctl")
                .args(Some("set-default"))
                .args(Some("graphical.target"))
                .status()
                .expect("Erro ao habiliar a inicialização em modo grafico");

            Command::new("dnf")
                .args(Some("remove"))
                .args(Some("gnome-tour"))
                .args(Some("-y"))
                .status()
                .expect("Erro ao remover alguns apps");

            Command::new("systemctl")
                .args(Some("enable"))
                .args(Some("cups"))
                .args(Some("-f"))
                .status()
                .expect("Erro ao habilitar o cups na inicialização");

        },

        "--install-fedora-user" => {
            
            Command::new("wget")
                .args(Some("https://github.com/robbyrussell/oh-my-zsh/raw/master/tools/install.sh"))
                .args(Some("-O"))
                .args(Some("-"))
                .args(Some("|"))
                .args(Some("zsh"))
                .status()
                .expect("Erro ao instalar o oh my zsh");

            Command::new("curl")
                .args(Some("--proto"))
                .args(Some("'=https'"))
                .args(Some("--tlsv1.2"))
                .args(Some("-sSf"))
                .args(Some("https://sh.rustup.rs"))
                .args(Some("|"))
                .args(Some("sh"))
                .status()
                .expect("Erro ao instalar o rustup, cargo e rustc");
                
            lib::texto(texts::ZSHRC, "/home/pedro/.zshrc");

            Command::new("chsh")
                .args(Some("-s"))
                .args(Some("/bin/zsh"))
                .args(Some("pedro"))
                .status()
                .expect("Erro ao definir o zsh como shell padrao do usuario pedro");

        },

        _ => {
            println!("Erro de um argumento valido");
        },

    }

}

//xset led on or xset led 0,1,2,3,4,5,6,7,8,9...
//echo "1" >> /sys/class/leds/input4\:\:scrolllock/brightness