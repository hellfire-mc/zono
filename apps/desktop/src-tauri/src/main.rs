#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use zono::rspc::RouterBuilder;

#[tokio::main]
async fn main() {
    let router = RouterBuilder::default().build();

    tauri::Builder::default()
        .plugin(rspc::integrations::tauri::plugin(router.into(), || ()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
