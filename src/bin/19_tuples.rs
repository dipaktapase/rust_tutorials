// It's a type of Record -> Record can be a line of information on a paper
// It's a way to access each piece of data within that record
// Stores data anonymously, NO need to name the fields
// Useful to return pairs of data from fucntions
// can be destructured easily into variables

enum Access {
    Full,
}

fn one_two_three() -> (i32, i32, i32) {
    // tuples are surrounded in parentheses
    (1, 2, 3)
}

fn main() {
    let numbers = one_two_three();
    let (x, y, z) = one_two_three();
    println!("{:?}, {:?}", x, numbers.0);
    println!("{:?}, {:?}", y, numbers.1);
    println!("{:?}, {:?}", z, numbers.2);

    let (employees, access) = ("Jakes", Access::Full); // Here we can mix different types of data
    println!("{employees}");
}

// Allows for anonymous data access
// Use struct when more than 2 or 3 fields
