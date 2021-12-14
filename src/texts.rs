pub static DNF: &str = "[main]
gpgcheck=1
installonly_limit=3
clean_requirements_on_remove=True
best=False
skip_if_unavailable=True
fastestmirror=True
max_parallel_downloads=7
defaultyes=True
install_weak_deps=false
";

pub static ZSHRC: &str = r#"export PATH=$PATH:$HOME/.bin/
export ZSH="/home/pedro/.oh-my-zsh"
ZSH_THEME="alanpeabody"
# ZSH_THEME_RANDOM_CANDIDATES=( "robbyrussell" "agnoster" )
plugins=(git)
source $ZSH/oh-my-zsh.sh
"#;

pub static VSCODE: &str = "[code]
name=Visual Studio Code
baseurl=https://packages.microsoft.com/yumrepos/vscode
enabled=1
gpgcheck=1
gpgkey=https://packages.microsoft.com/keys/microsoft.asc
";
