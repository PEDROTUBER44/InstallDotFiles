use std::{
    process::{
        Command // Importando a biblioteca para a execução de comandos do sistema operacional
    },
    env // Importando a biblioteca para capturar argumentos do usuario
};
mod utils;
mod texts;
mod lib;

fn main() {

    let args: Vec<String> = env::args().collect();
    let option = &args[1].trim();

    match &option[..] {

        "--install-arch-gnome-pedro" => {

            utils::install_and_configure_utils_on_archlinux_to_user_pedro();

            Command::new("sudo")
                .args(Some("pacman"))
                .args(Some("-Syu"))
                .args(Some("gimp"))
                .args(Some("inkscape"))
                .args(Some("gnome-boxes"))
                .args(Some("gnome-disk-utility"))
                .args(Some("gnome-music"))
                .args(Some("gedit"))
                .args(Some("fragments"))
                .args(Some("system-config-printer"))
                .args(Some("gnome-system-monitor"))
                .args(Some("gnome-calendar"))
                .args(Some("eog"))
                .args(Some("totem"))
                .args(Some("gnome-control-center"))
                .args(Some("--noconfirm"))
                .status()
                .expect("Erro ao instalar os aplicativos no archlinux");
            
            lib::text(texts::XDG_DATA_DIRS, "/home/pedro/.config/user-dirs.dirs");

            lib::create_dir("/home/pedro/Applications/");
            lib::create_dir("/home/pedro/Desktop/");
            lib::create_dir("/home/pedro/Documentos/");
            lib::create_dir("/home/pedro/Downloads/");
            lib::create_dir("/home/pedro/Imagens/");
            lib::create_dir("/home/pedro/Modelos/");
            lib::create_dir("/home/pedro/Músicas/");
            lib::create_dir("/home/pedro/Projetos/");
            lib::create_dir("/home/pedro/Público/");
            lib::create_dir("/home/pedro/Utilitarios/");
            lib::create_dir("/home/pedro/Vídeos/");
            lib::create_dir("/home/pedro/VirtualMachines/");

        },

        _ => {

            println!("Erro: Argumento invalido");
            
        }

    }

}