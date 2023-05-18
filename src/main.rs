#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::content::Json;
use rocket::request::Form;

#[derive(FromForm, Debug)]
struct Book {
  title: String,
  author: String,
  isbn: String
}

#[post("/book", data = "<book_form>")]
fn new_book(book_form: Form<Book>) -> String {
    let book: Book = book_form.into_inner();
    let mut dummy_db: Vec<Book> = Vec::new();
    dummy_db.push(book);
    format!("Book added successfully: {:?}", dummy_db)
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json("{
        'status': 'success',
        'message': 'Hello API!'
    }")
}

fn main() {
    rocket::ignite()
      .mount("/api", routes![hello])
      .launch();
}
