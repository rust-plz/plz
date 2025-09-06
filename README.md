# plz
A polite, Rust-powered Arch Linux package manager.  
*For Arch newbies that dont wanna touch pacman just yet.*

---

## What & Why
plz (aka “please”) is a lightweight package manager inspired by pacman.  
It’s designed for **Arch newbies** who want to explore the world of Arch Linux without immediately summoning the gods.  

⚠️ WIP: still under development. Use responsibly!  

---

## Features
- Upgrade all packages with a polite prompt: `plz upgrade all`  
- Written in Rust for speed and safety  
- Optional `sudo` usage to prevent accidental chaos  
- Beginner-friendly commands with polite messaging  
- No Windows. Only Arch. Only power.  

---

## Installation
Clone the repo and build with Rust:

```bash
git clone https://github.com/username/plz.git
cd plz
cargo build --release
sudo cp target/release/plz /usr/local/bin/
```

---

## Usage
Upgrade all packages:

```bash
sudo plz upgrade all
```

Install a package:

```bash
plz install <package_name>
```

Uninstall a package:

```bash
plz remove \<package_name\>
```

---

## Contributing
plz is a work in progress. Contributions, bug reports, and ideas are welcome!

---

made by isoextension
in the Ferrie toolkit
