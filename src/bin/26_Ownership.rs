// memeory must be manage in some way to prevent memory leaks
// memory leask => when program failed to track which memory is being used so then reserve new pieces of memory
// Rust utilizes an "ownership" model to manage memory
// the owner of memory is responsible for cleaning up the memory
// owner in rust is simply a function
// memory can either be "moved" or "borrowed" from the owner.

// Example - Move
enum Light {
    Bright,
    Dull,
}

fn display_light(light: Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

fn main() {
    // first when the function is called dull moved to the display_light() function and then dull is get deleted from the main
    // function and when we call the display_light() function again, the dull variable is already deleted. So it'll give an error.
    let dull = Light::Dull;
    display_light(dull);
    // display_light(dull);
}

// Example - Borrow

// fn main() {
// let dull = Light::Dull;
// display_light(&dull); // & using & It indicate we are borrowing data.// Also refer to a reference.
// display_light(&dull); // We can call that function again because we are uing & which shows that we are borrowing
// }

// Default behaviour is to "move" memory to a new owner
// User an ampersand (&) to allow code to "borrow" memory
