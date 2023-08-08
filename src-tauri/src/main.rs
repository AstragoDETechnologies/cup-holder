// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eject::{device::Device, discovery::cd_drives};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn open_cd_drives() {
    for path in cd_drives() {
        let drive = Device::open(path).unwrap();
        drive.eject().unwrap();
    }
}

#[tauri::command]
async fn close_cd_drives() {
    for path in cd_drives() {
        let drive = Device::open(path).unwrap();
        drive.retract().unwrap();
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_cd_drives, close_cd_drives])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
