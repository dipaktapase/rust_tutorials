// Repetition in program or Looping
// loop => infinite loop
// while => conditional loop

// Loop
let mut b = 0;
loop {
    if b == 5 {
        break;
    }
    println!("{b}");
    b += 1;
}

// While loop
let mut c = 1;
while c != 15 {
    println!("{c}");
    c += 1;
}

fn main () {}