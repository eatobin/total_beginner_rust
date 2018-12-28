use crate::book::Book;
use crate::borrower::Borrower;

pub fn add_item<T: PartialEq>(x: T, mut xs: Vec<T>) -> Vec<T> {
    if !xs.contains(&x) {
        xs.push(x);
        xs
    } else {
        xs
    }
}

#[test]
fn test_add_item() {
    let br1 = Borrower {
        name: String::from("Borrower1"),
        max_books: 1,
    };
    let mut brs: Vec<Borrower> = Vec::new();

    brs = add_item(br1, brs);
    assert_eq!(
        vec![Borrower {
            name: String::from("Borrower1"),
            max_books: 1,
        }, ],
        brs
    );

    brs = add_item(Borrower::new("Borrower1", 1), brs);
    assert_eq!(
        vec![Borrower {
            name: String::from("Borrower1"),
            max_books: 1,
        }, ],
        brs
    );

    let bk1 = Book {
        title: "Title1".to_owned(),
        author: String::from("Author1"),
        borrower: None,
    };
    let mut bks: Vec<Book> = Vec::new();

    bks = add_item(bk1, bks);
    assert_eq!(
        vec![Book {
            title: "Title1".to_owned(),
            author: "Author1".to_owned(),
            borrower: None,
        }, ],
        bks
    );

    bks = add_item(Book::new("Title1", "Author1"), bks);
    assert_eq!(
        vec![Book {
            title: "Title1".to_owned(),
            author: "Author1".to_owned(),
            borrower: None,
        }, ],
        bks
    );
}
