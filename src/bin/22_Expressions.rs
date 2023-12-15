// Rust is an Expression based language
// Most value are evaluated and return some value 
// Can be used for nesting logic
// Expression allows nested logic - Best to not use two or more levels

enum Menu {
    Burger,
    Fries,
    Drink,
}

fn main() {
    let paid = true;
    let item = Menu:: Drink;
    let drink_type = "water";
    let order_placed = match item {
        Menu::Drink => {
            if drink_type == "water" {
                true
            } else {
                false
            }
        }
        _ => true,
    };
}