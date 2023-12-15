fn main() {
    // Tuple using single variable
    let coord = (2, 3); // tuples with value 2 and 3, they get assigned to coord variable
    println!("{:?}, {:?}", coord.0, coord.1);

    // This way we assign x, y to tuples, here we don't need to use dot notation
    let (x, y) = (2, 3);
    println!("{:?}, {:?}", x, y);

    let (name, age) = ("Emma", 20);
    println!("{:?}, {:?}", name, age);

    // Bad example
    let favorite = ("red", 14, "TX", "pizza", "Tv Show", "home");
    let state = favorite.2;
    let food = favorite.3;
    println!("{state}, {food}");

    // To avoid this use Destructuring and give each tuple a different name
    let (color, number, state, food, show, place) = ("red", 14, "TX", "pizza", "Tv Show", "home");
    println!("{state}, {food}, {color}, {number}, {show}, {place}");
}
