// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::path::PathBuf;
use std::fs::{self, DirBuilder};
use serde::{Serialize, Deserialize};
use serde_json;

fn main() {
    first_install();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Serialize, Deserialize)]
struct Link {
    name: String,
    url: String
}

// fn add_link(name: String, url: String) {

// }

fn get_links() -> Vec<Link> {
    
}

// If program was installed first time this function creates the data directory and data file for links
fn first_install() {
    let mut data_dir: PathBuf = dirs::home_dir().unwrap();
    data_dir.push(".siponen");
    if !data_dir.exists() {
        DirBuilder::new().recursive(true).create(&data_dir).unwrap();
        println!("Program data directory created");
    }
    let mut file: PathBuf = data_dir.clone();
    file.push("links.json");
    if !file.exists() {
        fs::write(file, "").unwrap();
        println!("Links data file created\n");
    }
}
