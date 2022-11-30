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

fn main() {
    let conn = &mut establish_connection();

    let new_book = NewBook {
        title: "Momo",
        author: "Michael Ende",
    };
    let book: Book = diesel::insert_into(books::table)
        .values(&new_book)
        .get_result(conn)
        .expect("Error saving book");

    let new_page_1 = NewPage {
        page_number: 1,
        content: "In alten, alten Zeiten ...",
        book_id: book.id,
    };
    let page_1: Page = diesel::insert_into(pages::table)
        .values(&new_page_1)
        .get_result(conn)
        .expect("Error saving page 1");

    let new_page_2 = NewPage {
        page_number: 2,
        content: "den prachtvollen Theatern...",
        book_id: book.id,
    };
    let page_2: Page = diesel::insert_into(pages::table)
        .values(&new_page_2)
        .get_result(conn)
        .expect("Error saving page 2");

    // get pages for a book
    let pages = Page::belonging_to(&book)
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
    assert_eq!(book_maybe, book);
}
