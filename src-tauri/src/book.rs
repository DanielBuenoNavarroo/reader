use std::fs;
use epub::doc::EpubDoc;
use serde::Serialize;
use std::time::{ SystemTime, UNIX_EPOCH };
use tauri::Manager;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportedBook {
  pub id: String,
  pub title: String,
  pub author: String,
  pub path: String,
  pub imported_at: i64,
}

#[derive(Serialize)]
pub struct BookInfo {
  pub title: String,
  pub author: String,
  pub first_page: String,
}

#[derive(Serialize)]
pub struct Chapter {
  pub id: String,
  pub title: String,
  pub content: String,
}

#[tauri::command]
pub fn import_book(app: tauri::AppHandle, source_path: String) -> Result<ImportedBook, String> {
  let app_data = app
    .path()
    .app_data_dir()
    .map_err(|e| e.to_string())?;

  let books_dir = app_data.join("books");

  fs::create_dir_all(&books_dir).map_err(|e| e.to_string())?;

  let id = Uuid::new_v4().to_string();

  let target_path = books_dir.join(format!("{}.epub", id));

  fs::copy(&source_path, &target_path).map_err(|e| e.to_string())?;

  let doc = EpubDoc::new(&target_path).map_err(|e| e.to_string())?;

  let title = doc
    .mdata("title")
    .map(|m| m.value.clone())
    .unwrap_or_default();

  let author = doc
    .mdata("creator")
    .map(|m| m.value.clone())
    .unwrap_or_default();

  let imported_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

  Ok(ImportedBook {
    id,
    title,
    author,
    path: target_path.to_string_lossy().to_string(),
    imported_at,
  })
}

#[tauri::command]
pub fn get_chapters(path: String) -> Result<Vec<Chapter>, String> {
  let mut doc = EpubDoc::new(&path).map_err(|e| format!("Error abriendo EPUB: {:?}", e))?;

  let mut chapters = Vec::new();
  let mut index = 1;

  loop {
    if let Some((content, _)) = doc.get_current_str() {
      chapters.push(Chapter {
        id: index.to_string(),
        title: format!("Capítulo {}", index),
        content,
      });

      index += 1;
    }

    if !doc.go_next() {
      break;
    }
  }

  Ok(chapters)
}
