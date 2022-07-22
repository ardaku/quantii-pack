//! Format:
//! appname.qapp
//! \- app.toml
//! \- icon.svg
//!
//! `qapp` files are equivalent
//! to
//!
//! app.toml format:
//! ```toml
//! [app]
//! name = "appname"
//! repo = "https://link-to-repo/repo.git"
//! version = "0.1.0"
//! author = ["John Doe", "Jane Doe"]
//! description = "A simple app"
//! icon = "icon.svg"
//! other_metadata = "..."
//!```

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::all)]
#![allow(clippy::print_stdout)]
#![allow(clippy::print_stderr)]
#![allow(clippy::too_many_lines)]

use std::fs::{File, OpenOptions};
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};
use url::Url;

struct App(String, String, String, Vec<String>, String, PathBuf, String);
impl App {
    fn construct(&self) -> String {
        let mut app = String::new();
        app.push_str(&format!(
            r#"
[app]
name = "{}"
repo = "{}"
version = "{}"
author = ["{}"]
description = "{}"
icon = "{}"
other_metadata = "[{}]"
            "#,
            self.0,
            self.1,
            self.2,
            self.3.join("\", \""),
            self.4,
            self.5.to_str().unwrap(),
            self.6
        ));
        app
    }
}

fn main() {
    println!("Welcome to the interactive Quantii Application packager.");
    print!("Name of application: ");
    let mut app_name = String::new();
    stdout().flush().expect("Failed to flush stdout");
    stdin()
        .read_line(&mut app_name)
        .expect("Failed to read line");
    app_name = app_name.trim().to_owned();
    println!("Repository link: ");
    let mut repo_link_raw = String::new();
    stdout().flush().expect("Failed to flush stdout");
    stdin()
        .read_line(&mut repo_link_raw)
        .expect("Failed to read line");
    repo_link_raw = repo_link_raw.trim().to_owned();
    let repo_link = Url::parse(&repo_link_raw).expect("Failed to parse link");
    println!("Version of application: ");
    let mut app_version = String::new();
    stdout().flush().expect("Failed to flush stdout");
    stdin()
        .read_line(&mut app_version)
        .expect("Failed to read line");
    app_version = app_version.trim().to_owned();
    verify_version(&app_version);
    println!("Author(s) of application, seperated by commas: ");
    let mut app_authors_raw = String::new();
    stdout().flush().expect("Failed to flush stdout");
    stdin()
        .read_line(&mut app_authors_raw)
        .expect("Failed to read line");
    app_authors_raw = app_authors_raw.trim().to_owned();
    let app_authors = app_authors_raw.split(',').map(str::trim).map(ToOwned::to_owned).collect();
    println!("Description of application: ");
    let mut app_description = String::new();
    stdout().flush().expect("Failed to flush stdout");
    stdin()
        .read_line(&mut app_description)
        .expect("Failed to read line");
    app_description = app_description.trim().to_owned();
    println!("File to icon of application: ");
    let mut app_icon_raw = String::new();
    stdout().flush().expect("Failed to flush stdout");
    stdin()
        .read_line(&mut app_icon_raw)
        .expect("Failed to read line");
    app_icon_raw = app_icon_raw.trim().to_owned();
    let app_icon = PathBuf::from(app_icon_raw);
    println!("Other metadata of application, seperated by commas: ");
    let mut app_other_metadata = String::new();
    stdout().flush().expect("Failed to flush stdout");
    stdin()
        .read_line(&mut app_other_metadata)
        .expect("Failed to read line");
    app_other_metadata = app_other_metadata.trim().to_owned().split(',').collect();
    println!("Creating application...");
    let app = App(
        app_name,
        repo_link.to_string(),
        app_version,
        app_authors,
        app_description,
        app_icon,
        app_other_metadata,
    );

    let mut open_options = OpenOptions::new();
    open_options.write(true);
    open_options.create_new(true);
    let mut file: File;
    let app_toml = app.construct();
    if Path::new("app.toml").exists() {
        println!("app.toml already exists. Overwrite? (Y/n)");
        println!("How to continue?");
        println!("1. Overwrite");
        println!("2. Abort");
        println!("3. Different filename");
        let mut input = String::new();
        open_options.create_new(false);
        loop {
            stdout().flush().expect("Failed to flush stdout");
            stdin().read_line(&mut input).expect("Failed to read line");
            input = input.trim().to_owned();
            match input.parse::<i32>().unwrap() {
                1 => {
                    println!("Overwriting app.toml...");
                    open_options.truncate(true);
                    break;
                }
                2 => {
                    println!("Aborting...");
                    return;
                }
                3 => {
                    println!("Enter new filename: ");
                    let mut new_filename = String::new();
                    stdout().flush().expect("Failed to flush stdout");
                    stdin()
                        .read_line(&mut new_filename)
                        .expect("Failed to read line");
                    new_filename = new_filename.trim().to_owned();
                    open_options = std::fs::OpenOptions::new();
                    open_options.write(true).create_new(true);
                    file = open_options
                        .open(new_filename)
                        .expect("Failed to open file");
                    file.write_all(app_toml.as_bytes())
                        .expect("Failed to write to file");
                    println!("Done!");
                    return;
                }
                _ => {
                    println!("Invalid input");
                    continue;
                }
            }
        }
    }
    file = open_options.open("app.toml").expect("Failed to open file");
    file.write_all(app_toml.as_bytes())
        .expect("Failed to write to file");
    println!("Done!");
}

fn verify_version(version: &str) {
    let mut version_split = version.split('.');
    let major = version_split.next().unwrap();
    let minor = version_split.next().unwrap();
    let patch = version_split.next().unwrap();
    assert!(
        !(major.parse::<u32>().is_err()
            || minor.parse::<u32>().is_err()
            || patch.parse::<u32>().is_err()),
        "Version must be in the format of major.minor.patch"
    );
}
