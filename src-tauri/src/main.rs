use entity::prelude::Task;
use entity::task;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set};
use tauri_svelte_todo_app::establish_connection;

#[tauri::command]
async fn add_task(text: &str) -> Result<Vec<serde_json::Value>, String> {
    let db = establish_connection().await.unwrap();

    let task = task::ActiveModel {
        text: Set(text.to_string()),
        ..Default::default()
    };

    let _ = task.insert(&db).await.unwrap();

    // return all the data
    get_all_tasks().await
}

#[tauri::command]
async fn get_all_tasks() -> Result<Vec<serde_json::Value>, String> {
    let db = establish_connection().await.unwrap();
    let tasks = Task::find()
        .order_by_asc(task::Column::Id)
        .into_json()
        .all(&db)
        .await
        .unwrap();
    Ok(tasks)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_task, get_all_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
