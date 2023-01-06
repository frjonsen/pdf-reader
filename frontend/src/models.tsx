export interface Document {
  id: string;
  name: string;
  current_page: number;
}

export interface Bookmark {
  id: string;
  document: string;
  added_on: string;
  page: number;
  deleted_on: string;
  description: string;
}
