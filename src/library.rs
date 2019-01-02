use crate::book::Book;
use crate::borrower::Borrower;

pub fn add_item<'a, T: PartialEq>(x: &'a T, mut xs: Vec<&'a T>) -> Vec<&'a T> {
    if xs.contains(&x) {
        xs
    } else {
        xs.push(x);
        xs
    }
}

pub fn remove_book<'a>(bk: &Book, mut bks: Vec<&'a Book>) -> Vec<&'a Book> {
    bks.retain(|i| i != &bk);
    bks
}

//pub fn find_item<'a, T>(tgt: &str, coll: &'a Vec<&'a T>, f: &Fn(&T) -> &str) -> Option<&'a T> {
//    coll.iter().find(|&i| f(i) == tgt)
//}

//fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
//    shoes.into_iter()
//        .filter(|s| s.size == shoe_size)
//        .collect()
//}
//
// pub fn get_books_for_borrower(br: Borrower, bks: Vec<&Book>) -> Vec<&Book> {
//     bks.into_iter()
//         .filter(|&i| i.borrower == Some(br))
// }

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_item() {
        let br1 = Borrower {
            name: String::from("Borrower1"),
            max_books: 1,
        };
        let mut brs: Vec<&Borrower> = Vec::new();

        brs = add_item(&br1, brs);
        assert_eq!(
            vec![&Borrower {
                name: String::from("Borrower1"),
                max_books: 1,
            },],
            brs
        );

        let br1 = Borrower {
            name: String::from("Borrower1"),
            max_books: 1,
        };
        brs = add_item(&br1, brs);
        assert_eq!(
            vec![&Borrower {
                name: String::from("Borrower1"),
                max_books: 1,
            },],
            brs
        );

        let bk1 = Book {
            title: "Title1".to_owned(),
            author: String::from("Author1"),
            borrower: None,
        };
        let mut bks: Vec<&Book> = Vec::new();

        bks = add_item(&bk1, bks);
        assert_eq!(
            vec![&Book {
                title: "Title1".to_owned(),
                author: "Author1".to_owned(),
                borrower: None,
            },],
            bks
        );

        let bk1 = Book {
            title: "Title1".to_owned(),
            author: String::from("Author1"),
            borrower: None,
        };
        bks = add_item(&bk1, bks);
        assert_eq!(
            vec![&Book {
                title: "Title1".to_owned(),
                author: "Author1".to_owned(),
                borrower: None,
            },],
            bks
        );
    }

    #[test]
    fn test_remove_book() {
        let bk1 = Book {
            title: "Title1".to_owned(),
            author: "Author1".to_owned(),
            borrower: None,
        };
        let bk2 = Book {
            title: "Title2".to_owned(),
            author: "Author2".to_owned(),
            borrower: None,
        };
        let mut bks1: Vec<&Book> = vec![&bk1, &bk2];
        bks1 = remove_book(
            &Book {
                title: "Title1".to_owned(),
                author: "Author11".to_owned(),
                borrower: None,
            },
            bks1,
        );
        let bk3 = &Book {
            title: "Title2".to_owned(),
            author: "Author2".to_owned(),
            borrower: None,
        };
        let bks2 = vec![&bk3];
        //        assert_eq!(bks1, bks2);

        // bks1 = remove_book(
        //     Book {
        //         title: "Title1".to_owned(),
        //         author: "Author1".to_owned(),
        //         borrower: None,
        //     },
        //     bks1,
        // );
        // assert_eq!(bks1, bks2);
    }

    // #[test]
    // fn test_find_item() {
    //     let br1 = Borrower {
    //         name: String::from("Borrower1"),
    //         max_books: 1,
    //     };
    //     let br2 = Borrower {
    //         name: String::from("Borrower2"),
    //         max_books: 2,
    //     };

    //     let mut brl: Vec<Borrower> = Vec::new();
    //     brl.push(br1);
    //     brl.push(br2);

    //     let bk1 = Book {
    //         title: "Title1".to_owned(),
    //         author: "Author1".to_owned(),
    //         borrower: None,
    //     };

    //     let bk2 = Book {
    //         title: "Title2".to_owned(),
    //         author: "Author2".to_owned(),
    //         borrower: Some(Borrower {
    //             name: String::from("Borrower2"),
    //             max_books: 2,
    //         }),
    //     };

    //     let mut bkl: Vec<Book> = Vec::new();
    //     bkl.push(bk1);
    //     bkl.push(bk2);

    //     assert_eq!(
    //         find_item("Borrower1", &mut brl, &Borrower::get_name),
    //         Some(&Borrower {
    //             name: String::from("Borrower1"),
    //             max_books: 1,
    //         })
    //     );

    //     assert_eq!(
    //         find_item("Title2", &mut bkl, &Book::get_title),
    //         Some(&Book {
    //             title: "Title2".to_owned(),
    //             author: "Author2".to_owned(),
    //             borrower: Some(Borrower {
    //                 name: String::from("Borrower2"),
    //                 max_books: 2,
    //             }),
    //         })
    //     );

    //     assert_eq!(find_item("Borrower11", &mut brl, &Borrower::get_name), None);

    //     assert_eq!(find_item("Title22", &mut bkl, &Book::get_title), None);
    // }
}
