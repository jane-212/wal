use serde_json::{json, Value};
use tauri::command;

#[command]
pub async fn get_videos() -> Result<Value, String> {
    Ok(json!({
        "videos": [
            {
                "cover": "https://i.rotriza.com/fc2-ppv-1158812/cover.jpg?class=thumbnail",
                "title": "【个人拍摄】被两根肉棒沾上精液的丝滑柔嫩肌肤太妖娆的美熟女老婆",
                "time": "0:33:49",
                "preview": "https://i.rotriza.com/fc2-ppv-1158812/preview.mp4",
            }
        ],
    }))
}
