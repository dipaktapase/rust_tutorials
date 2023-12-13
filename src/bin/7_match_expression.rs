// Add logic to program
// similar to if else
// Exhaustive -> All options must be accounted for
// match works on expression not statements
// statements end a line, in comma case it ends particular expression

// match will checked by the complier

fn main() {
    let some_int = 33;
    match some_int {
        1 => println!("its 1"), // Don't add (semicolon); instead use comma (,)
        2 => println!("its 2"), 
        3 => println!("its 3"),
        _ => println!("its something else"),
    }
}

// IMP
// prefer match over else..if when working with a single variable
// match consider all possiblilities 
// More robust code
// Use underscore (_) to match "anything else"