#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Book{
    title: String,
    author: String,
    borrowed: RefCell<bool>
}

struct Library{
    books: Vec<RefCell<Book>>
}

impl Book{
    fn new(title: String, author: String) -> Self{
        Book{title, author, borrowed: RefCell::new(false)}
    }
}

impl Library{
    fn new() -> Self{
        Library{books: Vec::new()}
    }

    fn add_book(&mut self, book: Book){
        self.books.push(RefCell::new(book));
    }

    fn borrow_book(&self, book_title: &str) -> Option<()>{
        let book = self.books.iter().find(|x| (**x).borrow().title == book_title.to_string())?;
        let b = book.borrow_mut();
        if *b.borrowed.borrow() == true{
            None
        } else {
            b.borrowed.replace(true);
            Some(())
        }
    }

    fn return_book(&self, book_title: &str) -> Option<()>{
        let book = self.books.iter().find(|x| (**x).borrow().title == book_title.to_string())?;
        book.borrow_mut().borrowed.replace(false);
        Some(())
    }
}


struct Tree{
    val: i32,
    children: Vec<Rc<RefCell<Tree>>>
}

impl Tree{
    fn new(val: i32) -> Self{
        Tree{val, children: Vec::new()}
    }



    fn add_child(&mut self, parent: i32, child: i32){
        // Rc::new(RefCell::new())
        if self.val == parent{
            self.children.push(Rc::new(RefCell::new(Tree::new(child))));
            return;
        }
        for child_rc in &self.children {
            // let mut child_tree = RefCell::borrow_mut(child_rc);
            let mut child_tree = RefCell::borrow_mut(child_rc);
            child_tree.add_child(parent, child);
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_all(){
        let mut library = Library::new();
        let book1 = Book::new("Literally 1984".to_string(), "George Orwell".to_string());
        let book2 = Book::new("To Kill a Mockingbird".to_string(), "Harper Lee".to_string());
        library.add_book(book1.clone());
        library.add_book(book2.clone());

        assert_eq!(library.borrow_book("Literally 1984"), Some(()));
        assert_eq!(*library.books[0].borrow().borrowed.borrow(), true);
        assert_eq!(library.borrow_book("Literally 1984"), None);
        assert_eq!(library.return_book("Literally 1984"), Some(()));
        assert_eq!(*library.books[0].borrow().borrowed.borrow(), false);
    }

    #[test]
    fn test_tree(){
        let mut tree = Tree::new(1);
        tree.add_child(1, 2);
        tree.add_child(1, 3);
        tree.add_child(1, 4);
        tree.add_child(2, 5);
        tree.add_child(2, 6);
        tree.add_child(3, 7);
        assert_eq!(tree.children.len(), 3);
    }
}