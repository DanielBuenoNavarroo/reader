use tauri_plugin_sql::{ Migration, MigrationKind };

pub fn get_migrations() -> Vec<Migration> {
  vec![
    Migration {
      version: 1,
      description: "create_books_table",
      sql: r#"
        CREATE TABLE books (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            author TEXT,
            path TEXT NOT NULL,
            imported_at INTEGER NOT NULL
        );
      "#,
      kind: MigrationKind::Up,
    },
    Migration {
      version: 2,
      description: "create_reading_progress_table",
      sql: r#"
        CREATE TABLE reading_progress (
            book_id TEXT PRIMARY KEY,
            chapter INTEGER DEFAULT 0,
            scroll_position REAL DEFAULT 0,
            FOREIGN KEY (book_id) REFERENCES books(id)
        );
      "#,
      kind: MigrationKind::Up,
    }
  ]
}
