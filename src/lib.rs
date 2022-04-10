use std::{
    io::{
        prelude::{
            Write // Importado o escritor padrão para escrever em arquivos
        },
    },
    process::{
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

pub fn create_dir(path: &str) {

    let bool: bool = Path::new(path).is_dir();
    
    match bool {

        true => {

            let remove_dir = fs::remove_dir(path);

            match remove_dir {

                Err(e) => {

                    println!("Não foi possivel remover o arquivo: {}  Erro: {}",path,e);

                },

                Ok(_) => {

                    println!("Pasta removida com sucesso");

                    let create = fs::create_dir(path);

                    match create {

                        Err(e) => {

                            println!("Erro ao criar o diretorio: {}  Erro: {}",path,e);
                            exit(1);

                        },

                        Ok(_) => {

                            println!("Diretorio: {} Criado com sucesso",path);

                        }

                    }

                }

            };

        },

        false => {

            let create = fs::create_dir(path);

            match create {

                Err(e) => {
                    println!("Erro ao criar o diretorio: {}  Erro: {}",path,e);
                    exit(1);
                },

                Ok(_) => {
                    println!("Diretorio: {} Criado com sucesso",path);
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