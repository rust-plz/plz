use clap::{Arg, Command};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
    section: String,
    priority: String,
    architecture: String,
    origin: String,
    installed_size: String,
    download_size: String,
    maintainer: String,
    description: String,
    homepage: String,
    depends: Vec<String>,
    #[serde(default)]
    recommends: Vec<String>,
    #[serde(default)]
    suggests: Vec<String>,
    installed: bool,
}

fn load_packages() -> Result<Vec<Package>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("test_packages.json")?;
    let packages: Vec<Package> = serde_json::from_str(&content)?;
    Ok(packages)
}

fn list_packages(packages: &[Package]) {
    println!("{}", "Available Packages:".bold().blue());
    println!();
    
    for package in packages {
        let status = if package.installed {
            "[installed]".green()
        } else {
            "[available]".yellow()
        };
        
        println!(
            "{} {} {} - {}",
            package.name.bold(),
            package.version.dimmed(),
            status,
            package.description
        );
    }
}

fn show_package_info(packages: &[Package], name: &str) {
    if let Some(package) = packages.iter().find(|p| p.name == name) {
        println!("{}: {}", "Package".bold(), package.name);
        println!("{}: {}", "Version".bold(), package.version);
        println!("{}: {}", "Section".bold(), package.section);
        println!("{}: {}", "Origin".bold(), package.origin);
        println!("{}: {}", "Maintainer".bold(), package.maintainer);
        println!("{}: {}", "Homepage".bold(), package.homepage);
        println!("{}: {}", "Installed Size".bold(), package.installed_size);
        println!("{}: {}", "Download Size".bold(), package.download_size);
        println!("{}: {}", "Description".bold(), package.description);
        
        if package.installed {
            println!("{}: {}", "Status".bold(), "Installed".green());
        } else {
            println!("{}: {}", "Status".bold(), "Not installed".yellow());
        }
        
        if !package.depends.is_empty() {
            println!("{}: {}", "Depends".bold(), package.depends.join(", "));
        }
        
        if !package.recommends.is_empty() {
            println!("{}: {}", "Recommends".bold(), package.recommends.join(", "));
        }
        
        if !package.suggests.is_empty() {
            println!("{}: {}", "Suggests".bold(), package.suggests.join(", "));
        }
    } else {
        println!("{}: Package '{}' not found", "Error".red().bold(), name);
    }
}

fn main() {
    let matches = Command::new("plz")
        .about("Please - A polite package manager")
        .version("0.1.0")
        .subcommand(
            Command::new("list")
                .about("List available packages")
        )
        .subcommand(
            Command::new("show")
                .about("Show detailed information about a package")
                .arg(
                    Arg::new("package")
                        .help("Package name")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            Command::new("install")
                .about("Install a package")
                .arg(
                    Arg::new("package")
                        .help("Package name")
                        .required(true)
                        .index(1)
                )
        )
        .get_matches();

    let packages = match load_packages() {
        Ok(packages) => packages,
        Err(e) => {
            eprintln!("{}: Failed to load packages: {}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    };

    match matches.subcommand() {
        Some(("list", _)) => {
            list_packages(&packages);
        }
        Some(("show", sub_matches)) => {
            let package_name = sub_matches.get_one::<String>("package").unwrap();
            show_package_info(&packages, package_name);
        }
        Some(("install", sub_matches)) => {
            let package_name = sub_matches.get_one::<String>("package").unwrap();
            println!("{} would install package: {}", "Please".green().bold(), package_name);
            println!("(Installation not implemented yet)");
        }
        _ => {
            println!("{}", "Please specify a command. Use --help for usage information.".yellow());
        }
    }
}
