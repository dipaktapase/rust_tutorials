// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter


struct GroceryItems{
    id: i32,     
    quantity: i32,
}

fn display_quantity(item: &&GroceryItems) {
    println!("quantity {:?}",  item_quantity)
}

fn main() {
    Let my_items = GroceryItems {
        quantity: 3,
        id: 99,
    };
    display_quantity(&my_items);
    display.written
}