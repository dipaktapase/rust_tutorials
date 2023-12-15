// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Green,
    Blue,
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("it's red"),
        Color::Blue => println!("it's blue"),
        Color::Green => println!("it's green"),
    }
}

fn main() {
    print_color(Color::Red);
    print_color(Color::Blue);
    print_color(Color::Green);
}
