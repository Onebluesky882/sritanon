use std::io::BufRead;
use std::sync::Mutex;

static CHILD_PID: Mutex<Option<u32>> = Mutex::new(None);

#[tauri::command]
async fn start_transcription(app: tauri::AppHandle) -> Result<(), String> {
    let binary_path = app
        .path()
        .resource_dir()
        .map_err(|e| e.to_string())?
        .join("binaries/transcriber");

    println!("Binary path: {:?}", binary_path);

    let app_clone = app.clone();

    std::thread::spawn(move || {
        let mut child = match std::process::Command::new(&binary_path)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to spawn: {}", e);
                return;
            }
        };

        *CHILD_PID.lock().unwrap() = Some(child.id());

        let stdout = child.stdout.take().unwrap();
        let reader = std::io::BufReader::new(stdout);

        for line in reader.lines() {
            if let Ok(text) = line {
                if !text.is_empty() {
                    println!("Transcript: {}", text);
                    app_clone.emit("transcript", text).unwrap();
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
async fn stop_transcription() -> Result<(), String> {
    if let Some(pid) = *CHILD_PID.lock().unwrap() {
        unsafe {
            libc::kill(pid as i32, libc::SIGTERM);
        }
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_transcription,
            stop_transcription
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}