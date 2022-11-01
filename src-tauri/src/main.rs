#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use entity::prelude::Task;
use entity::task;
use sea_orm::{ActiveModelTrait, DeleteResult, EntityTrait, ModelTrait, QueryOrder, Set};
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
async fn update_task(task_id: i32, is_done: bool) -> Result<Vec<serde_json::Value>, String> {
    let db = establish_connection().await.unwrap();

    let task = task::Entity::find_by_id(task_id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    let mut task: task::ActiveModel = task.into();
    task.is_done = Set(is_done);
    let _: task::Model = task.update(&db).await.unwrap();

    get_all_tasks().await
}

#[tauri::command]
async fn delete_task(task_id: i32) -> Result<Vec<serde_json::Value>, String> {
    let db = establish_connection().await.unwrap();

    let task = task::Entity::find_by_id(task_id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    let res: DeleteResult = task.delete(&db).await.unwrap();
    assert_eq!(res.rows_affected, 1);

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
        .invoke_handler(tauri::generate_handler![
            add_task,
            get_all_tasks,
            delete_task,
            update_task,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
