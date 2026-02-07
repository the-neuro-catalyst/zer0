use tauri::AppHandle;

use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};

pub fn setup_menu(handle: &AppHandle) -> tauri::Result<()> {
    let file_menu = Submenu::with_items(
        handle,
        "File",
        true,
        &[&MenuItem::with_id(handle, "quit", "Quit ZERO Inspector", true, Some("CmdOrCtrl+Q"))?],
    )?;

    let edit_menu = Submenu::with_items(
        handle,
        "Edit",
        true,
        &[
            &PredefinedMenuItem::undo(handle, None)?,
            &PredefinedMenuItem::redo(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::cut(handle, None)?,
            &PredefinedMenuItem::copy(handle, None)?,
            &PredefinedMenuItem::paste(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::select_all(handle, None)?,
        ],
    )?;

    let menu = Menu::with_items(handle, &[&file_menu, &edit_menu])?;
    handle.set_menu(menu)?;
    Ok(())
}
