// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use systemstat::{Platform, System};
use tauri::Builder;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn hello(name: &str) -> String {
    let system = System::new();
    let networks = system.networks().unwrap();
    let mut mac_addresses = String::new();

    for (name, network) in networks.iter() {
        for addr in &network.addrs {
            if let systemstat::data::IpAddr::V4(ipv4) = addr.addr {
                if ipv4.is_private() {
                    mac_addresses.push_str(&format!("{}: {:?}\n", name, addr));
                }
            }
        }
    }

    format!("Hello, {}! Network Addresses:\n{}", name, mac_addresses)
}

#[tauri::command]
fn get_list() -> Vec<String> {
    let mut open_interfaces = Vec::new();

    open_interfaces.push("1".to_string());
    open_interfaces.push("2".to_string());

    open_interfaces
}

fn main() {
    Builder::default()
        .invoke_handler(tauri::generate_handler![hello, get_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
