use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod model;
pub mod schema;

use crate::model::*;
use crate::schema::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn new_author(conn: &mut PgConnection, name: &str) -> Author {
    let new_author = NewAuthor { name };
    diesel::insert_into(authors::table)
        .values(&new_author)
        .get_result(conn)
        .expect("Error saving author")
}

pub fn new_book(conn: &mut PgConnection, title: &str) -> Book {
    let new_book = NewBook { title };
    diesel::insert_into(books::table)
        .values(&new_book)
        .get_result(conn)
        .expect("Error saving book")
}

pub fn new_books_author(conn: &mut PgConnection, book_id: i32, author_id: i32) -> BooksAuthor {
    let new_books_author = NewBooksAuthor { book_id, author_id };
    diesel::insert_into(books_authors::table)
        .values(&new_books_author)
        .get_result(conn)
        .expect("Error saving BooksAuthor")
}
pub fn new_page(conn: &mut PgConnection, page_number: i32, content: &str, book_id: i32) -> Page {
    let new_page = NewPage {
        page_number,
        content,
        book_id,
    };
    diesel::insert_into(pages::table)
        .values(&new_page)
        .get_result(conn)
        .expect("Error saving page")
}

fn main() {
    let conn = &mut establish_connection();

    // create a book
    let momo = new_book(conn, "Momo");

    // a page in that book
    let page_1 = new_page(conn, 1, "In alten, alten Zeiten ...", momo.id);
    // a second page
    let page_2 = new_page(conn, 2, "den prachtvollen Theatern...", momo.id);

    // get pages for the book
    let pages = Page::belonging_to(&momo)
        .inner_join(books::table)
        .select(pages::all_columns)
        .load::<Page>(conn)
        .expect("Error loading pages");
    // the data is the same we put in
    assert_eq!(&page_1, pages.get(0).unwrap());
    assert_eq!(&page_2, pages.get(1).unwrap());

    // get a book from a page
    let book_maybe = books::table
        .find(page_2.book_id)
        .first::<Book>(conn)
        .expect("Error loading book");
    assert_eq!(book_maybe, momo);

    // create an author
    let michael_ende = new_author(conn, "Michael Ende");

    // let's add the author to the already created book
    new_books_author(conn, momo.id, michael_ende.id);

    // create a second author
    let astrid_lindgren = new_author(conn, "Astrid Lindgren");
    let pippi = new_book(conn, "Pippi Långstrump");
    new_books_author(conn, pippi.id, astrid_lindgren.id);

    // now that both have a single book, let's add a third book, an imaginary collaboration
    let collaboration = new_book(conn, "Pippi and Momo");
    new_books_author(conn, collaboration.id, astrid_lindgren.id);
    new_books_author(conn, collaboration.id, michael_ende.id);

    // get authors for the collaboration
    let authors = BooksAuthor::belonging_to(&collaboration)
        .inner_join(authors::table)
        .select(authors::all_columns)
        .load::<Author>(conn)
        .expect("Error loading authors");
    println!("{:?}", authors);

    // get all of Astrid Lindgren's books
    let books = BooksAuthor::belonging_to(&astrid_lindgren)
        .inner_join(books::table)
        .select(books::all_columns)
        .load::<Book>(conn)
        .expect("Error loading books");
    println!("{:?}", books);
}
