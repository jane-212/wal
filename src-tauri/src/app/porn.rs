use serde_json::{json, Value};
use tauri::command;
use tauri::State;

use crate::app::state::AppState;

#[command]
pub async fn porn_hot(
    page: u32,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    if page == 0 || page >= 4 {
        return Err("page of hot should be bigger than 0 and smaller than 4".to_string());
    }

    let videos = state
        .porn()
        .hot(page)
        .await
        .map_err(|err| err.to_string())?;

    Ok(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "videos": videos
        },
    }))
}
