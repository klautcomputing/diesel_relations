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
    let page_ids = Page::belonging_to(book).select(pages::book_id);
    let pages = pages::table
        .filter(pages::id.eq_any(page_ids))
        .load::<Page>(conn)
        .expect("could not load pages");
}
