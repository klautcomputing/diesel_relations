-- Your SQL goes here
CREATE TABLE books_authors (
  id SERIAL PRIMARY KEY,
  book_id SERIAL REFERENCES books(id),
  author_id SERIAL REFERENCES authors(id)
);
