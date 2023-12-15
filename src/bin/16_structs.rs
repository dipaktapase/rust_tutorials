// struct => structure
// It's a data type contins multiple pieces of data
// They are all or nothing
// Each piece of data called a field
// similar data can be grouped together in your program and travel together to parts of your code.

struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

fn main() {
    let my_box = ShippingBox {
        depth: 3,
        width: 5,
        height: 10,
    };

    let tall = my_box.height;
    let width = my_box.width;
    let depth = my_box.depth;
    println!("this box is tall: {tall}");
    println!("this box is width: {width}");
    println!("this box is depth: {depth}");
}

// All fields must be present to create a structure
// Fields can be accessed using a dot (.)
