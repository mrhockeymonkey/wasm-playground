#[allow(warnings)]
mod bindings;

use bindings::exports::component::books_api_dep::{books, authors};
use bindings::component::db_dep::get;
use crate::bindings::exports::component::books_api_dep::books::Book;

struct Component;

impl books::Guest for Component {
    fn get_books() -> Vec<Book> {
        let book_jsons = get::get_all("book");

        vec![
            Book { name: "The Call of Cthulhu".to_string(), author: "H. P. Lovecraft".to_string(), description: "".to_string() }
        ]
    }

    fn get_book(id: u32) -> books::Book {
        let book_json = get::get_one("books", id);
        Book { name: "The Call of Cthulhu".to_string(), author: "H. P. Lovecraft".to_string(), description: "".to_string() }
    }
}

impl authors::Guest for Component {
    fn get_authors() -> Vec<String> {
        get::get_all("author");
        todo!()
    }

    fn get_author(id: u32) -> String {
        get::get_one("author", id);
        todo!()
    }
}

bindings::export!(Component with_types_in bindings);
