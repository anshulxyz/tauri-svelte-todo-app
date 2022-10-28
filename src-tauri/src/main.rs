use entity::task;
use sea_orm::{ActiveModelTrait, Set};
use tauri_svelte_todo_app::establish_connection;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn add_task(text: &str) -> Result<serde_json::Value, ()> {
    // format!("Hello, {}! You've been greeted from Rust!", text)
    let db = establish_connection().await.unwrap();

    let task = task::ActiveModel {
        text: Set(text.to_string()),
        ..Default::default()
    };

    let task = task.insert(&db).await.unwrap();
    Ok(serde_json::to_value(task).unwrap())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
