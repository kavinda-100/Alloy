use colored::*;

pub fn print_banner() {
    let banner = r#"
    ___    __    __    ____  __  __
   /   |  / /   / /   / __ \ \ \/ /
  / /| | / /   / /   / / / /  \  / 
 / ___ |/ /___/ /___/ /_/ /   / /  
/_/  |_/_____/_____/\____/   /_/   
    "#;

    println!("{}", banner.cyan().bold());
    println!(
        "{} {}",
        "◈".cyan(),
        "The Smart Contract Type Fusion Tool".italic().white()
    );
    println!(
        "{}\n",
        "------------------------------------------".bright_black()
    );
}
