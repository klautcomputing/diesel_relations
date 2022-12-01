use diesel::prelude::*;

use crate::schema::books;
use crate::schema::pages;
use crate::schema::authors;
use crate::schema::books_authors;

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = authors)]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = authors)]
pub struct NewAuthor<'a> {
    pub name: &'a str,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(Book))]
#[diesel(belongs_to(Author))]
#[diesel(table_name = books_authors)]
pub struct BooksAuthor {
    pub id: i32,
    pub book_id: i32,
    pub author_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = books_authors)]
pub struct NewBooksAuthor {
    pub book_id: i32,
    pub author_id: i32,
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = books)]
pub struct Book {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook<'a> {
    pub title: &'a str,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[diesel(belongs_to(Book))]
#[diesel(table_name = pages)]
pub struct Page {
    pub id: i32,
    pub page_number: i32,
    pub content: String,
    pub book_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = pages)]
pub struct NewPage<'a> {
    pub page_number: i32,
    pub content: &'a str,
    pub book_id: i32,
}
