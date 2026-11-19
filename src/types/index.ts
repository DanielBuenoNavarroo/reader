export type Book = {
  id: string;
  title: string;
  author: string;
  path: string;
  progress?: number;
  coverPath?: string;
  importedAt?: number;
};
