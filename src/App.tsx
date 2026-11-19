import "./App.css";
import { Button } from "./components/ui/button";
import { open } from "@tauri-apps/plugin-dialog";
import { import_book } from "./commands/system";
import { db } from "./db";
import { Book } from "./types";
import { useEffect, useState } from "react";

function App() {
  const [books, setBooks] = useState<Book[]>([]);
  const getAllBooks = async () => {
    const _books = await db.select<Book[]>(`
      SELECT
        id,
        title,
        author,
        path,
        imported_at as importedAt
      FROM books
      ORDER BY imported_at DESC
    `);

    setBooks(_books);
  };

  useEffect(() => {
    getAllBooks();
  }, []);

  const importBook = async () => {
    const file = await open({
      multiple: false,
      directory: false,
    });

    if (file?.trim() === "" || !file) return;

    const book: Book | null = await import_book(file);

    if (!book) return;

    console.log("Book: ", book);

    await db.execute(
      `
      INSERT INTO books (
        id,
        title,
        author,
        path,
        imported_at
      )
      VALUES (?, ?, ?, ?, ?)
      `,
      [book.id, book.title, book.author, book.path, book.importedAt],
    );

    setBooks((prev) => [book, ...prev]);
  };
  return (
    <main className="">
      <Button onClick={importBook}>Añadir libro</Button>
      <div className="mt-4">
        {books &&
          books.map((book) => (
            <div key={book.id} className="border rounded-md p-4 mb-2">
              <h2 className="font-semibold">{book.title}</h2>

              <p className="text-sm text-muted-foreground">{book.author}</p>
            </div>
          ))}
      </div>
    </main>
  );
}

export default App;
