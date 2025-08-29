use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime,
};

#[tauri::command]
async fn set_autostart(_app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    use std::process::Command;
    
    // 현재 실행 파일 경로 가져오기
    let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe_path_str = exe_path.to_string_lossy().to_string();
    
    if enabled {
        // 레지스트리에 시작 프로그램 등록
        #[cfg(target_os = "windows")]
        {
            let output = Command::new("reg")
                .args(&[
                    "add",
                    r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                    "/v",
                    "AlarmTimer",
                    "/t",
                    "REG_SZ",
                    "/d",
                    &format!("\"{}\" --autostart", exe_path_str),
                    "/f"
                ])
                .output()
                .map_err(|e| e.to_string())?;
                
            if !output.status.success() {
                return Err("Failed to add to startup".to_string());
            }
        }
    } else {
        // 레지스트리에서 시작 프로그램 제거
        #[cfg(target_os = "windows")]
        {
            let _ = Command::new("reg")
                .args(&[
                    "delete",
                    r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                    "/v",
                    "AlarmTimer",
                    "/f"
                ])
                .output();
        }
    }
    
    Ok(())
}

#[tauri::command]
async fn get_autostart(_app: tauri::AppHandle) -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        let output = Command::new("reg")
            .args(&[
                "query",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                "/v",
                "AlarmTimer"
            ])
            .output()
            .map_err(|e| e.to_string())?;
            
        Ok(output.status.success())
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(false)
    }
}

fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let quit_i = MenuItemBuilder::with_id("quit", "종료").build(app)?;
    let show_i = MenuItemBuilder::with_id("show", "창 보이기").build(app)?;
    let hide_i = MenuItemBuilder::with_id("hide", "창 숨기기").build(app)?;
    
    let menu = MenuBuilder::new(app)
        .items(&[&show_i, &hide_i, &quit_i])
        .build()?;

    let _ = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("알람 타이머")
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![set_autostart, get_autostart])
        .setup(|app| {
            // 트레이 아이콘 생성
            create_tray(app.handle())?;
            
            // 명령줄 인수 확인
            let args: Vec<String> = std::env::args().collect();
            if args.contains(&"--autostart".to_string()) {
                // 시작 프로그램으로 실행된 경우 창 숨기기
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            
            Ok(())
        })
        .on_window_event(|window, event| {
            // 창 닫기 버튼 클릭 시 숨기기만 하고 종료하지 않음
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
