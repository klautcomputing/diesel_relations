use diesel::prelude::*;

use crate::schema::books;
use crate::schema::pages;

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = books)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub author: &'a str,
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
