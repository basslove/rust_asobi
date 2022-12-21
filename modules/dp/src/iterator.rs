use anyhow::{anyhow, bail, Result};

trait Iterable {
    fn iterator(&self) -> Box<dyn Iterator>;
}

trait Iterator {
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Result<Book>;
}

#[derive(Debug, Clone)]
struct Book {
    name: String,
}

impl Book {
    fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
struct BookShelf {
    books: Vec<Book>,
    last: usize,
}

impl BookShelf {
    fn new(max_size: usize) -> Self {
        Self {
            last: 0,
            books: Vec::with_capacity(max_size),
        }
    }

    fn get_book_at(&self, index: usize) -> Book {
        self.books[index].clone()
    }

    fn append_book(&mut self, book: &Book) {
        self.books.push(book.clone());
        self.last += 1;
    }

    fn get_length(&self) -> usize {
        self.last
    }
}

impl Iterable for BookShelf {
    fn iterator(&self) -> Box<dyn Iterator> {
        Box::new(BookShelterIterator::new(self))
    }
}

struct BookShelterIterator {
    book_shelf: BookShelf,
    index: usize,
}

impl BookShelterIterator {
    fn new(book_shelf: &BookShelf) -> Self {
        Self {
            book_shelf: book_shelf.clone(),
            index: 0,
        }
    }
}

impl Iterator for BookShelterIterator {
    fn has_next(&self) -> bool {
        self.index < self.book_shelf.get_length()
    }

    fn next(&mut self) -> Result<Book> {
        if !self.has_next() {
            bail!(anyhow!("data not found"));
        }
        let book = self.book_shelf.get_book_at(self.index);
        self.index += 1;

        Ok(book)
    }
}

pub fn execute() {
    println!("iterator");

    let mut book_shelf = BookShelf::new(4);
    book_shelf.append_book(&Book::new("no.1"));
    book_shelf.append_book(&Book::new("no.2"));
    book_shelf.append_book(&Book::new("no.3"));
    book_shelf.append_book(&Book::new("no.4"));

    let mut it = book_shelf.iterator();
    while it.has_next() {
        let book = it.next().unwrap();
        println!("{:?}", book.get_name());
    }
    println!()
}
