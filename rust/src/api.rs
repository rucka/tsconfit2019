use crate::data::{BOOKS, ORDERS};
use std::str::FromStr;

#[derive(Clone)]
pub struct Book {
    name: String,
    author: String,
    price: f64,
}

impl Book {
    pub fn new(name: &str, author: &str, price: f64) -> Book {
        Book {
            name: String::from_str(name).unwrap(),
            author: String::from_str(author).unwrap(),
            price,
        }
    }
}

#[derive(Clone)]
pub struct OrderLine {
    book_id: String,
    quantity: i32,
}

impl OrderLine {
    pub fn new(book_id: &str, quantity: i32) -> OrderLine {
        OrderLine {
            book_id: String::from_str(book_id).unwrap(),
            quantity,
        }
    }
}

#[derive(Clone)]
pub struct Order {
    date: chrono::NaiveDateTime,
    items: Vec<OrderLine>,
}

impl Order {
    pub fn new(year: i32, month: u32, day: u32, items: &[OrderLine]) -> Order {
        Order {
            date: chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_ymd(year, month, day),
                chrono::NaiveTime::from_hms(0, 0, 0),
            ),
            items: items.to_vec(),
        }
    }
}

pub enum OrderNotValid {
    NoItems,
    BookNotExists,
}

pub type ValidationResult = Result<(), OrderNotValid>;

#[derive(Clone, Copy)]
pub struct OrderSuccessful {
    pub amount: f64,
}
impl OrderSuccessful {
    pub fn new(amount: f64) -> OrderSuccessful {
        OrderSuccessful { amount }
    }
}

pub type PlaceOrderResult = Result<OrderSuccessful, OrderNotValid>;

pub fn validate_order(order: &Order) -> ValidationResult {
    if order.items.len() == 0 {
        return Err(OrderNotValid::NoItems);
    }
    for item in &order.items {
        if !BOOKS.contains_key(&item.book_id) {
            return Err(OrderNotValid::BookNotExists);
        }
    }
    Ok(())
}
