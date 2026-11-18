use epub::doc::EpubDoc;
use serde::Serialize;

#[derive(Serialize)]
pub struct BookInfo {
    pub title: String,
    pub author: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Serialize)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub content: String,
}

#[tauri::command]
pub fn open_book(path: String) -> Result<BookInfo, String> {
    // Abrir EPUB
    let mut doc = EpubDoc::new(&path).map_err(|e| format!("Error abriendo EPUB: {:?}", e))?;

    // Obtener metadatos
    let title = doc
        .mdata("title")
        .map(|m| m.value.clone())
        .unwrap_or_default();

    let author = doc
        .mdata("creator")
        .map(|m| m.value.clone())
        .unwrap_or_default();

    // Obtener capítulos
    let mut chapters = Vec::new();
    let mut index = 1;

    loop {
        if let Some((content, _mime)) = doc.get_current_str() {
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

    Ok(BookInfo {
        title,
        author,
        chapters,
    })
}
