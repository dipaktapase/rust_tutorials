fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    let sum = 3 + 2;
    let value = 10 - 5;
    let division = 10 / 2;
    let mult = 10 * 5;

    let five = sub(8, 3);

    let rem = 6 % 3;
    let rem2 = 6 % 4;
    
    println!("Sum => {sum}, Value =>  {value} {mult} {division} Sub => {five} {rem} {rem2}");
}