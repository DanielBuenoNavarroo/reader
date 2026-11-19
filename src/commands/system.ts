import { Book } from "@/types";
import { invoke } from "@tauri-apps/api/core";

export function import_book(path: string): Promise<Book | null> {
  return invoke("import_book", { sourcePath: path });
}
