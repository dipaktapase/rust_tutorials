struct GroceryItem {
    stock: i32,
    price: f64, // 64 bit floating point number
}

fn main() {
    let cereal = GroceryItem {
        stock: 10,
        price: 2.99,
    };

    println!("stock {:?}", cereal.stock);
    println!("price {:?}", cereal.price);
}