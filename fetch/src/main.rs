extern crate colored;
use colored::Colorize;

// https://fontawesome.com/icons/ follow this link for terminal icons
// ctrl+v u<code> to display in text form

fn main() {
    let username = "sol";
    let hostname = "Arco";
    let distro = "Arch Linux";
    let shell = "nushell";
    let terminal = "foot";
    let wm = "sway";

    println!(
        "\n  {}@{}",
        username.yellow().bold(),
        hostname.yellow().bold()
    );
    println!(
        "  {}~ {}",
        " Distro  ".green().bold(),
        distro.green().bold()
    );
    println!("  {}~ {}", " Shell   ".cyan().bold(), shell.cyan().bold());
    println!(
        "  {}~ {}",
        " Term    ".blue().bold(),
        terminal.blue().bold()
    );
    println!(
        "  {}~ {}",
        " WM      
        "
        .purple()
        .bold(),
        wm.purple().bold()
    );
    println!(
        "  {}{}{}{}{}{}",
        "▅▅▅".red(),
        "▅▅▅".yellow(),
        "▅▅▅".green(),
        "▅▅▅".cyan(),
        "▅▅▅".blue(),
        "▅▅▅".purple()
    );
    println!("");
}
