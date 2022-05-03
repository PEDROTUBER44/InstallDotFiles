use std::{
    process::{
        exit // Importing the standard exit library to exit the program
    },
    env // Importing the standard env library to capture user arguments
};
mod utils;


fn main() {

    let args: Vec<String> = env::args().collect();
    let option = &args[1].trim();
    let user = &args[2].trim();

    match &option[..] {

        "--install-arch-lxde" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("archlinux","lxde","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("archlinux","lxde","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-arch-lxqt" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("archlinux","lxqt","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("archlinux","lxqt","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-arch-xfce" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("archlinux","xfce","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("archlinux","xfce","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-arch-gnome" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("archlinux","gnome","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("archlinux","gnome","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-arch-cinnamon" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("archlinux","cinnamon","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("archlinux","cinnamon","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-arch-mate" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("archlinux","mate","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("archlinux","mate","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-arch-kdeplasma" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("archlinux","kdeplasma","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("archlinux","kdeplasma","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-arch-bspwm" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("archlinux","bspwm","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("archlinux","bspwm","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-arch-cutefish" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("archlinux","cutefish","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("archlinux","cutefish","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },


        "--install-debian-lxde" => {
            
            match &user[..] {

                "pedro" => {

                    utils::exec_installation("debian","lxde","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("debian","lxde","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-debian-lxqt" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("debian","lxqt","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("debian","lxqt","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-debian-xfce" => {
        
            match &user[..] {

                "pedro" => {

                    utils::exec_installation("debian","xfce","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("debian","xfce","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }
        
        },

        "--install-debian-gnome" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("debian","gnome","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("debian","gnome","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-debian-cinnamon" => {
        
            match &user[..] {

                "pedro" => {

                    utils::exec_installation("debian","cinnamon","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("debian","cinnamon","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-debian-mate" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("debian","mate","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("debian","mate","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-debian-kdeplasma" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("debian","kdeplasma","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("debian","kdeplasma","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-debian-bspwm" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("debian","bspwm","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("debian","bspwm","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-debian-cutefish" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("debian","cutefish","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("debian","cutefish","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },


        "--install-fedora-lxde" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("fedora","lxde","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("fedora","lxde","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-fedora-lxqt" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("fedora","lxqt","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("fedora","lxqt","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-fedora-xfce" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("fedora","xfce","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("fedora","xfce","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-fedora-gnome" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("fedora","gnome","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("fedora","gnome","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-fedora-cinnamon" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("fedora","cinnamon","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("fedora","cinnamon","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-fedora-mate" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("fedora","mate","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("fedora","mate","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-fedora-kdeplasma" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("fedora","kdeplasma","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("fedora","kdeplasma","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-fedora-bspwm" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("fedora","bspwm","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("fedora","bspwm","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },

        "--install-fedora-cutefish" => {

            match &user[..] {

                "pedro" => {

                    utils::exec_installation("fedora","cutefish","pedro");
                    exit(0);

                },

                "kauan" => {

                    utils::exec_installation("fedora","cutefish","kauan");
                    exit(0);

                },

                _ => {println!("Internal Error: User not found");}
            }

        },
        

        _ => {println!("Enter a valid argument");}

    }

}