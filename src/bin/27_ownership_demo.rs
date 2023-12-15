struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("Pages = {:?}", book.pages);
}

fn display_rating(book: &Book) {
    println!("Rating = {:?}", book.rating);
}

fn main() {
    let book = Book {
        pages: 5,
        rating: 3,
    };
    display_page_count(&book);
    display_rating(&book);
}
