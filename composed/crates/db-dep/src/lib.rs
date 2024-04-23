#[allow(warnings)]
mod bindings;

use bindings::exports::component::db_dep::get::Guest;

struct Component;

impl Guest for Component {
    fn get_one(table: String, id: u32) -> String {
        todo!()
    }

    fn get_all(table: String) -> Vec<String> {
        todo!()
    }
}

bindings::export!(Component with_types_in bindings);
