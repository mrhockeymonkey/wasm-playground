#[allow(warnings)]
mod bindings;

use bindings::component::books_api_dep::books;

fn main() {

    println!("Hello, world!");
    let books = books::get_books();

    dbg!(books);
    println!("fin");
}
