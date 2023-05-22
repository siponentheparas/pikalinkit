// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::path::PathBuf;
use std::fs::{self, DirBuilder};
use std::vec;
use serde::{Serialize, Deserialize};
use serde_json;

fn main() {
    first_install();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_links_json, add_link])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Serialize, Deserialize)]
struct Link {
    name: String,
    url: String
}

// add links to file to remember
#[tauri::command]
fn add_link(name: String, url: String) {
    let mut links = get_links();
    let link: Link = Link { name: name, url: url };
    links.push(link);
    let json = serde_json::to_string(&links).unwrap();
    let mut data_file: PathBuf = dirs::home_dir().unwrap();
    data_file.push(".siponen/links.json");
    fs::write(data_file, json).unwrap();
}

// gets links so rust can understand them
fn get_links() -> Vec<Link> {
    let mut data_file: PathBuf = dirs::home_dir().unwrap();
    data_file.push(".siponen/links.json");
    let contents = fs::read_to_string(data_file).unwrap();
    let links: Vec<Link> = serde_json::from_str(&contents).unwrap();
    return links
}

// gets links so frontend can understand them
#[tauri::command]
fn get_links_json() -> serde_json::Value {
    let mut data_file: PathBuf = dirs::home_dir().unwrap();
    data_file.push(".siponen/links.json");
    let contents = fs::read_to_string(data_file).unwrap();
    let links: Vec<Link> = serde_json::from_str(&contents).unwrap();
    return serde_json::json!(&links)
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
        let example_link: Link = Link { name: "google".to_string(), url: "google.com".to_string() };
        let example: Vec<Link> = vec![example_link];
        let json = serde_json::to_string(&example).unwrap();
        fs::write(file, json).unwrap();
        println!("Links data file created\n");
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
    add_link("kakka".to_string(), "riveria.fi".to_string());
}
