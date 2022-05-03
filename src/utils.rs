use std::{
    io::{
        prelude::{
            Write // Importado o escritor padrão para escrever em arquivos
        },
    },
    process::{
        Command, // Importing the standard command library for running operating system commands
        exit // Importando o comando exit para sair do programa
    },
    path::{
        Path // Importando criador de arquivos e pastas padrão
    },
    fs::{
        File // Importando manipulador de arquivos padrão
    },
    fs // Importando manipulador de arquivos e pasta padrão
};


pub fn systemcommand_asuser(command: &str, args: &str, err: &str) {

    Command::new(command).args(args.split_ascii_whitespace()).status().expect(err);

}

pub fn systemcommand_asroot(command: &str, err: &str) {

    Command::new("sudo").args(command.split_ascii_whitespace()).status().expect(err);
    
}

pub fn remove_folder(folder: &str) {

    let remove_dir = fs::remove_dir(&folder);

    match remove_dir {

        Err(e) => {println!("Could not remove folder: {}  Error: {}",folder,e);},
        Ok(_) => {println!("Folder successfully removed");}

    };

}

pub fn create_dir(path: &str) {

    let cpath = Path::new(&path);
    let bools = cpath.exists();
    let display = cpath.display();
    
    match bools {

        true => {

            let remove_dir = fs::remove_dir(&cpath);

            match remove_dir {

                Err(e) => {

                    println!("Could not remove folder: {}  Error: {}",display,e);

                },

                Ok(_) => {

                    println!("Folder successfully removed");

                    let create = fs::create_dir(&cpath);

                    match create {

                        Err(e) => {

                            println!("Error creating folder: {}  Error: {}",display,e);
                            exit(1);

                        },

                        Ok(_) => {

                            println!("Directory: {} successfully created",display);

                        }

                    }

                }

            };

        },

        false => {

            let create = fs::create_dir(&cpath);

            match create {

                Err(e) => {
                    println!("Error creating folder: {}  Error: {}",display,e);
                    exit(1);
                },

                Ok(_) => {
                    println!("Directory: {} successfully created",display);
                }

            }

        }

    }

}

pub fn text(text: &str, path: &str) {
    
    let cpath = Path::new(&path);
    let bools = cpath.exists();
    let display = cpath.display();

    match bools {

        true => {

            match fs::remove_file(&cpath) {

                Err(e) => {

                    println!("Não foi possivel remover o arquivo: {}  Erro:{}",display,e);
                    exit(1);

                },

                Ok(_) => {

                    println!("Arquivo removido com sucesso");

                }

            };

            let mut file = match File::create(&cpath) {
                
                Err(e) => {

                    println!("Não foi possivel remover o arquivo: {} ERRO: {}",display,e);
                    exit(0);

                },

                Ok(file) => file

            };

            match file.write_all(text.as_bytes()) {

                Err(e) => {

                    println!("Não foi possivel escrever no arquivo: {} ERRO: {}",display,e);
                    exit(0);

                },

                Ok(_) => {

                    println!("Arquivo escrito com sucesso");

                }

            };

        },
        
        false => {
            
            let mut file = match File::create(&cpath) {
                
                Err(e) => {

                    println!("Não foi possivel criar o arquivo: {} ERRO: {}",display,e);
                    exit(0);

                },

                Ok(file) => file

            };

            match file.write_all(text.as_bytes()) {

                Err(e) => {

                    println!("Não foi possivel escrever no arquivo: {} ERRO: {}",display,e);
                    exit(0);

                },

                Ok(_) => {

                    println!("Arquivo escrito com sucesso");
                    
                }

            };

        }

    }

}

pub fn install_aur(url: &str, folder: &str) {

    Command::new("git").args(Some("clone")).args(Some(url)).status().expect("Error git repository not found");
    systemcommand_asuser("cd", folder, "Error folder not found");
    systemcommand_asuser("makepkg", "-sicr", "Error to compile aur");
    systemcommand_asuser("cd", "..", "Error exiting directory");
    remove_folder(folder);

}


pub fn install_utils(system: &str, user: &str) {

    match system {

        "archlinux" => {

            create_dir("/home/pedro/Desktop/");
            create_dir("/home/pedro/Documentos/");
            create_dir("/home/pedro/Downloads/");
            create_dir("/home/pedro/Imagens/");
            create_dir("/home/pedro/Modelos/");
            create_dir("/home/pedro/Músicas/");
            create_dir("/home/pedro/Projetos/");
            create_dir("/home/pedro/Público/");
            create_dir("/home/pedro/Utilitarios/");
            create_dir("/home/pedro/Vídeos/");
            create_dir("/home/pedro/VirtualMachines/");

            match user {

                "pedro" => {

                    pub const XDG_DATA_DIRS : &str = r#"
                    XDG_DESKTOP_DIR="$HOME/Desktop"
                    XDG_DOWNLOAD_DIR="$HOME/Downloads"
                    XDG_TEMPLATES_DIR="$HOME/Modelos"
                    XDG_PUBLICSHARE_DIR="$HOME/Público"
                    XDG_DUCUMENTS_DIR="$HOME/Documentos"
                    XDG_MUSIC_DIR="$HOME/Músicas"
                    XDG_PICTURES_DIR="$HOME/Imagens"
                    XDG_VIDEOS_DIR="$HOME/Vídeos""#;

                    systemcommand_asroot("pacman -Syu cups flatpak networkmanager gvfs-mtp gvfs-goa gvfs-google exfat-utils bluez p7zip zip unzip unrar system-config-printer adwaita-icon-theme xf86-video-intel libgl mesa ffmpeg gst-plugins-ugly gst-plugins-good gst-plugins-base gst-plugins-bad gst-libav gstreamer git docker qemu ovmf neovim xdg-user-dirs --noconfirm", "Error installing archlinux utilities");
                    text(XDG_DATA_DIRS, "/home/pedro/.config/user-dirs.dirs");
                    install_aur("https://aur.archlinux.org/preload.git", "preload/");
                    install_aur("https://aur.archlinux.org/epson-inkjet-printer-escpr.git", "epson-inkjet-printer-escpr/");
                    systemcommand_asroot("systemctl enable preload -f", "Error enabling preload deamon autostart at system boot");
                    systemcommand_asroot("systemctl enable bluetooth -f", "Error enabling bluetooth deamon autostart at system boot");
                    systemcommand_asroot("systemctl enable cups -f", "Error enabling cups deamon autostart at system boot");
                    systemcommand_asroot("systemctl enable NetworkManager -f", "Error enabling NetworkManager autostart at system boot");

                },

                "kauan" => {

                    pub const XDG_DATA_DIRS : &str = r#"
                    XDG_DESKTOP_DIR="$HOME/Desktop"
                    XDG_DOWNLOAD_DIR="$HOME/Downloads"
                    XDG_TEMPLATES_DIR="$HOME/Modelos"
                    XDG_PUBLICSHARE_DIR="$HOME/Público"
                    XDG_DUCUMENTS_DIR="$HOME/Documentos"
                    XDG_MUSIC_DIR="$HOME/Músicas"
                    XDG_PICTURES_DIR="$HOME/Imagens"
                    XDG_VIDEOS_DIR="$HOME/Vídeos""#;

                    systemcommand_asroot("pacman -Syu cups flatpak networkmanager gvfs-mtp gvfs-goa gvfs-google exfat-utils p7zip zip unzip unrar adwaita-icon-theme xf86-video-intel libgl mesa ffmpeg gst-plugins-ugly gst-plugins-good gst-plugins-base gst-plugins-bad gst-libav gstreamer git xdg-user-dirs --noconfirm", "Error installing archlinux utilities");
                    text(XDG_DATA_DIRS, "/home/kauan/.config/user-dirs.dirs");
                    install_aur("https://aur.archlinux.org/preload.git", "preload/");
                    install_aur("https://aur.archlinux.org/epson-inkjet-printer-escpr.git", "epson-inkjet-printer-escpr/");
                    systemcommand_asroot("systemctl enable preload -f", "Error enabling preload deamon autostart at system boot");
                    systemcommand_asroot("systemctl enable bluetooth -f", "Error enabling bluetooth deamon autostart at system boot");
                    systemcommand_asroot("systemctl enable cups -f", "Error enabling cups deamon autostart at system boot");
                    systemcommand_asroot("systemctl enable NetworkManager -f", "Error enabling NetworkManager autostart at system boot");

                },

                _ => {println!("Internal error, user not found");}

            }

        },

        "debian" => {

            match user {

                "pedro" => {

                    systemcommand_asroot("apt install sudo zip unzip unrar-free network-manager preload gvfs pulseaudio gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-ugly gstreamer1.0-plugins-bad ffmpeg sox twolame vorbis-tools lame faad mencoder exfat-utils p7zip-full system-config-printer adwaita-icon-theme bluez -y", "Error installing debian 11 utilities");
                    systemcommand_asroot("systemctl enable NetworkManager -f", "Error enabling NetworkManager autostart at system boot");
                    systemcommand_asroot("systemctl enable preload -f", "Error enabling preload deamon autostart at system boot");
                    systemcommand_asroot("systemctl enable bluez -f", "Error enabling bluetooth deamon autostart at system boot");

                },
                "kauan" => {

                    systemcommand_asroot("apt install sudo zip unzip unrar-free network-manager preload gvfs pulseaudio gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-ugly gstreamer1.0-plugins-bad ffmpeg sox twolame vorbis-tools lame faad mencoder exfat-utils p7zip-full adwaita-icon-theme -y", "Error installing debian 11 utilities");
                    systemcommand_asroot("systemctl enable NetworkManager -f", "Error enabling NetworkManager autostart at system boot");
                    systemcommand_asroot("systemctl enable preload -f", "Error enabling preload deamon autostart at system boot");

                },
                _ => {println!("Internal error, user not found");}

            }

        },

        "fedora" => {

            systemcommand_asroot("rm -r /etc/dnf/dnf.conf", "Error to remove /etc/dnf/dnf.conf");
            systemcommand_asroot(r#"echo "[main]" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "gpgcheck=1" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "installonly_limit=3" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "clean_requirements_on_remove=True" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "best=False" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "skip_if_unavailable=True" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "fastestmirror=True" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "max_parallel_downloads=7" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "defaultyes=True" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");
            systemcommand_asroot(r#"echo "install_weak_deps=false" >> /etc/dnf/dnf.conf"#, "Error writing to /etc/dnf/dnf.conf file");

            systemcommand_asroot("dnf update -y", "Error updating fedora 35");

            match user {

                "pedro" => {

                    systemcommand_asroot("dnf install bluez preload @multimedia unrar p7zip zip unzip NetworkManager fedora-workstation-backgrounds exfat-utils gvfs-mtp gvfs-goa system-config-printer gstreamer1-plugins-base gstreamer1-plugins-good gstreamer1-plugins-ugly gstreamer1-plugins-bad-free gstreamer1-plugins-bad-free gstreamer1-plugins-bad-freeworld gstreamer1-plugins-bad-free-extras ffmpeg https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-35.noarch.rpm https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-35.noarch.rpm -y", "Error installing fedora 35 utilities");
                    systemcommand_asroot("dnf install pipewire-codec-aptx -y", "Error to install aptx codec");
                    systemcommand_asroot("systemctl enable NetworkManager -f", "Error enabling NetworkManager autostart at system boot");
                    systemcommand_asroot("systemctl enable bluez -f", "Error enabling bluetooth deamon autostart at system boot");
                    systemcommand_asroot("systemctl enable preload -f", "Error enabling preload deamon autostart at system boot");

                },

                "kauan" => {

                    systemcommand_asroot("dnf install preload @multimedia unrar p7zip zip unzip NetworkManager fedora-workstation-backgrounds exfat-utils gvfs-mtp gvfs-goa gstreamer1-plugins-base gstreamer1-plugins-good gstreamer1-plugins-ugly gstreamer1-plugins-bad-free gstreamer1-plugins-bad-free gstreamer1-plugins-bad-freeworld gstreamer1-plugins-bad-free-extras ffmpeg https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-35.noarch.rpm https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-35.noarch.rpm -y", "Error installing fedora 35 utilities");
                    systemcommand_asroot("dnf install pipewire-codec-aptx -y", "Error to install aptx codec");
                    systemcommand_asroot("systemctl enable NetworkManager -f", "Error enabling NetworkManager autostart at system boot");
                    systemcommand_asroot("systemctl enable preload -f", "Error enabling preload deamon autostart at system boot");

                },

                _ => {println!("Internal error, user not found");}

            }

        },

        _ => {println!("Internal error, system not found");}

    }

}

pub fn remove_extra_files(system: &str) {

    match system {

        "archlinux" => {

            systemcommand_asroot("mv /usr/share/applications/avahi-discover.desktop /usr/share/applications/avahi-discover.backup", "Error to rename file: avahi-discover.desktop");
            systemcommand_asroot("mv /usr/share/applications/bssh.desktop /usr/share/applications/bssh.backup", "Error to rename file: bssh.desktop");
            systemcommand_asroot("mv /usr/share/applications/bvnc.desktop /usr/share/applications/bvnc.backup", "Error to rename file: bvnc.desktop");
            systemcommand_asroot("mv /usr/share/applications/nm-connection-editor.desktop /usr/share/applications/nm-connection-editor.backup", "Error to rename file: nm-connection-editor.desktop");
            systemcommand_asroot("mv /usr/share/applications/qv4l2.desktop /usr/share/applications/qv4l2.backup", "Error to rename file: qv4l2.desktop");
            systemcommand_asroot("mv /usr/share/applications/qvidcap.desktop /usr/share/applications/qvidcap.backup", "Error to rename file: qvidcap.desktop");

        },

        "debian" => {

            systemcommand_asroot("mv /usr/share/applications/vim.desktop /usr/share/applications/vim.backup", "Error to rename file: vim.desktop");

        },

        "fedora" => {

            println!("No extra files to remove");

        },

        _ => {

            println!("Internal error, system not found");

        }

    }

}

pub fn install_desktop_in_system(system: &str, desktop: &str, user: &str) {

    match user {

        "pedro" => {

            match system {

                "archlinux" => {

                    match desktop {

                        "lxde" => {

                            systemcommand_asroot("pacman -Syu xorg lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver mousepad totem atril gnome-screenshot gnome-disk-utility bluez transmission-gtk --noconfirm", "Error installing minimal lxde on archlinux");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "lxqt" => {

                            systemcommand_asroot("pacman -Syu xorg lxqt lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver vlc --noconfirm", "Error installing minimal lxqt on archlinux");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "xfce" => {

                            systemcommand_asroot("pacman -Syu xorg lightdm lightdm-gtk-greeter xfce4-settings xfce4-pulseaudio-plugin adwaita-icon-theme exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller --noconfirm", "Error installing xfce minimal on archlinux");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "gnome" => {

                            systemcommand_asroot("pacman -Syu gdm weston nautilus gnome-control-center adwaita-icon-theme gnome-disk-utility system-config-printer gnome-system-monitor gnome-calendar gnome-control-center gnome-software --noconfirm", "Error installing gnome minimal on archlinux");
                            systemcommand_asroot("systemctl enable gdm -f", "Error enabling gdm on startup");
                            systemcommand_asuser("flatpak","install com.mattjakeman.ExtensionManager io.github.seadve.Kooha com.valvesoftware.Steam de.haeckerfelix.Fragments md.obsidian.Obsidian org.chromium.Chromium org.gimp.GIMP org.gnome.Boxes org.gnome.Builder org.gnome.Evince org.gnome.FileRoller org.gnome.Music org.gnome.TextEditor org.gnome.Totem org.gnome.eog org.gnome.seahorse.Application org.inkscape.Inkscape org.kde.kdenlive org.keepassxc.KeePassXC org.onlyoffice.desktopeditors","Error to install flatpak packages");
                            install_aur("https://aur.archlinux.org/visual-studio-code-bin.git", "visual-studio-code-bin/");
                            install_aur("https://aur.archlinux.org/gnome-console.git", "gnome-console/");

                        },

                        "cinnamon" => {

                            systemcommand_asroot("pacman -Syu xorg lightdm lightdm-gtk-greeter cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations adwaita-icon-theme cjs muffin nemo nemo-fileroller file-roller --noconfirm", "Error installing cinnamon minimal on archlinux");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "mate" => {

                            systemcommand_asroot("pacman -Syu xorg lightdm lightdm-gtk-greeter mate-desktop adwaita-icon-theme mate-control-center mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja --noconfirm", "Error installing minimal mate on archlinux");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "kdeplasma" => {

                            systemcommand_asroot("pacman -Syu weston sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard adwaita-icon-theme spectacle dolphin discover --noconfirm", "Error installing cinnamon minimal on archlinux");
                            systemcommand_asroot("systemctl enable sddm -f", "Error enabling lightdm on startup");

                        },

                        "bspwm" => {println!("Coming soon");exit(0);},

                        "cutefish" => {println!("Coming soon");exit(0);},

                        _ => {println!("Internal error, system not found");}

                    }

                },

                "debian" => {

                    match desktop {

                        "lxde" => {

                            systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter lxde-core lxterminal deluge file-roller mousepad gpicview gnome-disk-utility evince lxappearance pavucontrol lxsession-default-apps lxinput menu gnome-system-tools connman connman-gtk xscreensaver policykit-1 policykit-1-gnome xarchiver -y", "Error installing minimal lxde on debian 11");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "lxqt" => {

                            systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter lxqt-core vlc ark ktorrent connman partitionmanager qpdfview pavucontrol -y", "Error installing lxqt minimal on debian 11");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "xfce" => {

                            systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter thunar xfce4-panel xfce4-pulseaudio-plugin xfce4-whiskermenu-plugin xfce4-session xfce4-settings xfce4-terminal pavucontrol mousepad thunar-archive-plugin evince xfconf xfdesktop4 xfwm4 adwaita-qt qt5ct xfce4-taskmanager xfce4-screenshooter --no-install-recommends -y", "Error installing xfce4 minimal on debian 11");
                            systemcommand_asroot("apt install vlc gnome-disk-utility xarchiver ristretto transmission -y", "Error to install more xfce4 packages in debian 11");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "gnome" => {

                            systemcommand_asroot("apt install weston gdm3 gnome-control-center gnome-software gnome-terminal gnome-tweaks nautilus adwaita-icon-theme gnome-system-monitor gnome-screenshot gnome-disk-utility --no-install-recommends -y", "Error installing gnome minimal on debian 11");
                            systemcommand_asuser("flatpak","install com.mattjakeman.ExtensionManager io.github.seadve.Kooha com.valvesoftware.Steam de.haeckerfelix.Fragments md.obsidian.Obsidian org.chromium.Chromium org.gimp.GIMP org.gnome.Boxes org.gnome.Builder org.gnome.Evince org.gnome.FileRoller org.gnome.Music org.gnome.TextEditor org.gnome.Totem org.gnome.eog org.gnome.seahorse.Application org.inkscape.Inkscape org.kde.kdenlive org.keepassxc.KeePassXC org.onlyoffice.desktopeditors","Error to install flatpak packages");
                            systemcommand_asroot("systemctl enable gdm -f", "Error enabling gdm3 on startup");

                        },

                        "cinnamon" => {

                            systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter cinnamon-core gnome-terminal eog totem evince gedit gnome-system-monitor gnome-screenshot file-roller transmission-gtk gnome-disk-utility --no-install-recommends -y", "Error installing cinnamon minimal on debian 11");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "mate" => {

                            systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter mate-desktop-environment gnome-disk-utility transmission-gtk file-roller totem -y", "Error installing minimal mate on debian 11");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "kdeplasma" => {

                            systemcommand_asroot("apt install weston sddm kde-plasma-desktop plasma-nm plasma-workspace-wayland systemsettings dolphin kwrite ark okular plasma-discover konsole ktorrent kde-spectacle gwenview -y", "Error installing minimal kde plasma on debian 11");
                            systemcommand_asroot("systemctl enable sddm -f", "Error enabling sddm on startup");

                        },

                        "bspwm" => {println!("Coming soon");exit(0);},

                        "cutefish" => {println!("Coming soon");exit(0);},
                        
                        _ => {println!("Internal error, system not found");}

                    }

                },

                "fedora" => {

                    match desktop {

                        "lxde" => {

                            systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter lxde-common lxdm openbox lxappearance lxsession lxterminal pcmanfm lxinput lxmenu-data lxpanel lxpolkit lxrandr lxtask xcompmgr xarchiver obconf network-manager-applet -y", "Error installing minimal lxde on fedora 35");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "lxqt" => {

                            systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora -y", "Error installing lxqt minimal on fedora 35");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");
                            systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                        },

                        "xfce" => {

                            systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter xfwm4 xfce4-session xfdesktop xfce4-settings xfce4-terminal xfce4-whiskermenu-plugin xfce4-power-manager network-manager-applet -y", "Error installing xfce4 minimal on fedora 35");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");
                            systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                        },

                        "gnome" => {

                            systemcommand_asroot("dnf install gdm gnome-shell nautilus gnome-console fedora-workstation-backgrounds gnome-console-nautilus -y", "Error installing gnome on fedora 35");
                            systemcommand_asuser("flatpak","install com.mattjakeman.ExtensionManager io.github.seadve.Kooha com.valvesoftware.Steam de.haeckerfelix.Fragments md.obsidian.Obsidian org.chromium.Chromium org.gimp.GIMP org.gnome.Boxes org.gnome.Builder org.gnome.Evince org.gnome.FileRoller org.gnome.Music org.gnome.TextEditor org.gnome.Totem org.gnome.eog org.gnome.seahorse.Application org.inkscape.Inkscape org.kde.kdenlive org.keepassxc.KeePassXC org.onlyoffice.desktopeditors","Error to install flatpak packages");
                            systemcommand_asroot("systemctl enable gdm -f", "Error enabling gdm on startup");
                            systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                        },

                        "cinnamon" => {

                            systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter cinnamon cinnamon-desktop cinnamon-session cinnamon-menus cinnamon-screensaver gnome-terminal cinnamon-translations muffin cinnamon-control-center cjs nemo nemo-fileroller -y", "Error installing cinnamon on fedora 35");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");
                            systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                        },

                        "mate" => {

                            systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter mate-desktop mate-control-center mate-screensaver mate-power-manager mate-screenshot mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja network-manager-applet -y", "Error installing mate in fedora 35");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");
                            systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                        },

                        "kdeplasma" => {

                            systemcommand_asroot("dnf install sddm plasma-desktop plasma-nm konsole plasma-discover dolphin kscreen ksysguard spectacle plasma-user-manager kcm_colors kcm-fcitx -y", "Error installing kde plasma on fedora 35");
                            systemcommand_asroot("systemctl enable sddm -f", "Error enabling sddm on startup");
                            systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                        },
                        
                        "bspwm" => {println!("Coming soon");exit(0);},
                        
                        "cutefish" => {println!("Coming soon");exit(0);},
                        
                        _ => {println!("Internal error, system not found");}

                    }

                }

                _ => {println!("Internal error, system not found");}

            }

        },

        "kauan" => {

            match system {

                "archlinux" => {

                    match desktop {

                        "lxde" => {

                            systemcommand_asroot("pacman -Syu xorg lxde lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver mousepad totem atril gnome-screenshot gnome-disk-utility bluez transmission-gtk --noconfirm", "Error installing minimal lxde on archlinux");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "lxqt" => {

                            systemcommand_asroot("pacman -Syu xorg lxqt lightdm lightdm-gtk-greeter adwaita-icon-theme xarchiver vlc --noconfirm", "Error installing minimal lxqt on archlinux");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "xfce" => {

                            systemcommand_asroot("pacman -Syu xorg lightdm lightdm-gtk-greeter xfce4-settings xfce4-pulseaudio-plugin adwaita-icon-theme exo garcon tumbler xfce4-panel xfce4-session xfce4-whiskermenu-plugin xfce4-terminal xfconf xfdesktop xfwm4 thunar file-roller --noconfirm", "Error installing xfce minimal on archlinux");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "gnome" => {

                            systemcommand_asroot("pacman -Syu gdm weston nautilus gnome-control-center adwaita-icon-theme gnome-disk-utility system-config-printer gnome-system-monitor gnome-calendar gnome-control-center gnome-software --noconfirm", "Error installing gnome minimal on archlinux");
                            systemcommand_asroot("systemctl enable gdm -f", "Error enabling gdm on startup");
                            systemcommand_asuser("flatpak","install com.mattjakeman.ExtensionManager io.github.seadve.Kooha com.valvesoftware.Steam de.haeckerfelix.Fragments md.obsidian.Obsidian org.chromium.Chromium org.gimp.GIMP org.gnome.Boxes org.gnome.Builder org.gnome.Evince org.gnome.FileRoller org.gnome.Music org.gnome.TextEditor org.gnome.Totem org.gnome.eog org.gnome.seahorse.Application org.inkscape.Inkscape org.kde.kdenlive org.keepassxc.KeePassXC org.onlyoffice.desktopeditors","Error to install flatpak packages");
                            install_aur("https://aur.archlinux.org/visual-studio-code-bin.git", "visual-studio-code-bin/");
                            install_aur("https://aur.archlinux.org/gnome-console.git", "gnome-console/");

                        },

                        "cinnamon" => {

                            systemcommand_asroot("pacman -Syu xorg lightdm lightdm-gtk-greeter cinnamon cinnamon-session cinnamon-desktop gnome-terminal cinnamon-control-center cinnamon-menus cinnamon-screensaver cinnamon-settings-daemon cinnamon-translations adwaita-icon-theme cjs muffin nemo nemo-fileroller file-roller --noconfirm", "Error installing cinnamon minimal on archlinux");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "mate" => {

                            systemcommand_asroot("pacman -Syu xorg lightdm lightdm-gtk-greeter mate-desktop adwaita-icon-theme mate-control-center mate-power-manager mate-screensaver mate-common mate-session-manager mate-settings-daemon mate-terminal network-manager-applet mate-panel marco caja --noconfirm", "Error installing minimal mate on archlinux");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "kdeplasma" => {

                            systemcommand_asroot("pacman -Syu weston sddm plasma-desktop plasma-nm konsole plasma-wayland-session kcm-fcitx kscreen ksysguard adwaita-icon-theme spectacle dolphin discover --noconfirm", "Error installing cinnamon minimal on archlinux");
                            systemcommand_asroot("systemctl enable sddm -f", "Error enabling lightdm on startup");

                        },

                        "bspwm" => {println!("Coming soon");exit(0);},

                        "cutefish" => {println!("Coming soon");exit(0);},

                        _ => {println!("Internal error, system not found");}

                    }

                },

                "debian" => {

                    match desktop {

                        "lxde" => {

                            systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter lxde-core lxterminal deluge file-roller mousepad gpicview gnome-disk-utility evince lxappearance pavucontrol lxsession-default-apps lxinput menu gnome-system-tools connman connman-gtk xscreensaver policykit-1 policykit-1-gnome xarchiver -y", "Error installing minimal lxde on debian 11");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "lxqt" => {

                            systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter lxqt-core vlc ark ktorrent connman partitionmanager qpdfview pavucontrol -y", "Error installing lxqt minimal on debian 11");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "xfce" => {

                            systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter thunar xfce4-panel xfce4-pulseaudio-plugin xfce4-whiskermenu-plugin xfce4-session xfce4-settings xfce4-terminal pavucontrol mousepad thunar-archive-plugin evince xfconf xfdesktop4 xfwm4 adwaita-qt qt5ct xfce4-taskmanager xfce4-screenshooter --no-install-recommends -y", "Error installing xfce4 minimal on debian 11");
                            systemcommand_asroot("apt install vlc gnome-disk-utility xarchiver ristretto transmission -y", "Error to install more xfce4 packages in debian 11");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "gnome" => {

                            systemcommand_asroot("apt install weston gdm3 gnome-control-center gnome-software gnome-terminal gnome-tweaks nautilus adwaita-icon-theme gnome-system-monitor gnome-screenshot gnome-disk-utility --no-install-recommends -y", "Error installing gnome minimal on debian 11");
                            systemcommand_asuser("flatpak","install com.mattjakeman.ExtensionManager io.github.seadve.Kooha com.valvesoftware.Steam de.haeckerfelix.Fragments md.obsidian.Obsidian org.chromium.Chromium org.gimp.GIMP org.gnome.Boxes org.gnome.Builder org.gnome.Evince org.gnome.FileRoller org.gnome.Music org.gnome.TextEditor org.gnome.Totem org.gnome.eog org.gnome.seahorse.Application org.inkscape.Inkscape org.kde.kdenlive org.keepassxc.KeePassXC org.onlyoffice.desktopeditors","Error to install flatpak packages");
                            systemcommand_asroot("systemctl enable gdm -f", "Error enabling gdm3 on startup");

                        },

                        "cinnamon" => {

                            systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter cinnamon-core gnome-terminal eog totem evince gedit gnome-system-monitor gnome-screenshot file-roller transmission-gtk gnome-disk-utility --no-install-recommends -y", "Error installing cinnamon minimal on debian 11");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "mate" => {

                            systemcommand_asroot("apt install xorg lightdm lightdm-gtk-greeter mate-desktop-environment gnome-disk-utility transmission-gtk file-roller totem -y", "Error installing minimal mate on debian 11");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "kdeplasma" => {

                            systemcommand_asroot("apt install weston sddm kde-plasma-desktop plasma-nm plasma-workspace-wayland systemsettings dolphin kwrite ark okular plasma-discover konsole ktorrent kde-spectacle gwenview -y", "Error installing minimal kde plasma on debian 11");
                            systemcommand_asroot("systemctl enable sddm -f", "Error enabling sddm on startup");

                        },

                        "bspwm" => {println!("Coming soon");exit(0);},

                        "cutefish" => {println!("Coming soon");exit(0);},
                        
                        _ => {println!("Internal error, system not found");}

                    }

                },

                "fedora" => {

                    match desktop {

                        "lxde" => {

                            systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter lxde-common lxdm openbox lxappearance lxsession lxterminal pcmanfm lxinput lxmenu-data lxpanel lxpolkit lxrandr lxtask xcompmgr xarchiver obconf network-manager-applet -y", "Error installing minimal lxde on fedora 35");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");

                        },

                        "lxqt" => {

                            systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter lxqt-about lxqt-archiver lxqt-config lxqt-notificationd lxqt-openssh-askpass lxqt-panel breeze-cursor-theme breeze-gtk breeze-icon-theme firewall-config network-manager-applet notification-daemon obconf openbox pcmanfm-qt qterminal lxqt-policykit lxqt-powermanagement lxqt-qtplugin lxqt-session lxqt-themes lxqt-themes-fedora -y", "Error installing lxqt minimal on fedora 35");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");
                            systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                        },

                        "xfce" => {

                            systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter xfwm4 xfce4-session xfdesktop xfce4-settings xfce4-terminal xfce4-whiskermenu-plugin xfce4-power-manager network-manager-applet -y", "Error installing xfce4 minimal on fedora 35");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");
                            systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                        },

                        "gnome" => {

                            systemcommand_asroot("dnf install gdm gnome-shell nautilus gnome-console fedora-workstation-backgrounds gnome-console-nautilus -y", "Error installing gnome on fedora 35");
                            systemcommand_asuser("flatpak","install com.mattjakeman.ExtensionManager io.github.seadve.Kooha com.valvesoftware.Steam de.haeckerfelix.Fragments md.obsidian.Obsidian org.chromium.Chromium org.gimp.GIMP org.gnome.Boxes org.gnome.Builder org.gnome.Evince org.gnome.FileRoller org.gnome.Music org.gnome.TextEditor org.gnome.Totem org.gnome.eog org.gnome.seahorse.Application org.inkscape.Inkscape org.kde.kdenlive org.keepassxc.KeePassXC org.onlyoffice.desktopeditors","Error to install flatpak packages");
                            systemcommand_asroot("systemctl enable gdm -f", "Error enabling gdm on startup");
                            systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                        },

                        "cinnamon" => {

                            systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter cinnamon cinnamon-desktop cinnamon-session cinnamon-menus cinnamon-screensaver gnome-terminal cinnamon-translations muffin cinnamon-control-center cjs nemo nemo-fileroller -y", "Error installing cinnamon on fedora 35");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");
                            systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                        },

                        "mate" => {

                            systemcommand_asroot("dnf install lightdm lightdm-gtk-greeter mate-desktop mate-control-center mate-screensaver mate-power-manager mate-screenshot mate-session-manager mate-settings-daemon mate-terminal mate-panel marco caja network-manager-applet -y", "Error installing mate in fedora 35");
                            systemcommand_asroot("systemctl enable lightdm -f", "Error enabling lightdm on startup");
                            systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                        },

                        "kdeplasma" => {

                            systemcommand_asroot("dnf install sddm plasma-desktop plasma-nm konsole plasma-discover dolphin kscreen ksysguard spectacle plasma-user-manager kcm_colors kcm-fcitx -y", "Error installing kde plasma on fedora 35");
                            systemcommand_asroot("systemctl enable sddm -f", "Error enabling sddm on startup");
                            systemcommand_asroot("systemctl set-default graphical.target", "Error enabling graphical mode boot");

                        },
                        
                        "bspwm" => {println!("Coming soon");exit(0);},
                        
                        "cutefish" => {println!("Coming soon");exit(0);},
                        
                        _ => {println!("Internal error, system not found");}

                    }

                },

                _ => {println!("Internal error, system not found");}

            }

        },

        _ => {println!("Internal error, user not found");}

    }

}

pub fn exec_installation(system: &str, desktop: &str, user: &str) {

    install_utils(system, user);
    install_desktop_in_system(system, desktop, user);
    remove_extra_files(system);
    systemcommand_asroot("reboot", "Error to restarting system");

}