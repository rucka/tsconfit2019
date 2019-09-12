use crate::api::*;
use lazy_static::*;
use std::collections::HashMap;
use std::str::FromStr;

fn s(text: &str) -> String {
    String::from_str(text).unwrap()
}

lazy_static! {
    pub static ref BOOKS: HashMap<String, Book> = {
        let mut m = HashMap::new();
        m.insert(
            s("1"),
            Book::new("Positioning: The Battle for Your Mind", "Al Reis", 12.98),
        );
        m.insert(
            s("2"),
            Book::new(
                "Start With Why: How Great Leaders Inspire Everyone to Take Action",
                "Simon Sinek",
                11.51,
            ),
        );
        m.insert(
            s("3"),
            Book::new(
                "Pitch Anything: An Innovative Method for Presenting, Persuading, and Winning the Deal",
                "Oren Klaff",
                19.23),
        );
        m
    };
    pub static ref ORDERS: HashMap<String, Order> = {
        let mut m = HashMap::new();
        m.insert(s("1"), Order::new(2019, 1, 1, &[]));
        m.insert(
            s("2"),
            Order::new(
                2019,
                1,
                2,
                &[OrderLine::new("1", 10), OrderLine::new("3", 27)],
            ),
        );
        m.insert(
            s("3"),
            Order::new(
                2019,
                1,
                3,
                &[OrderLine::new("2", 7), OrderLine::new("3", 5)],
            ),
        );
        m.insert(
            s("4"),
            Order::new(
                2019,
                1,
                4,
                &[
                    OrderLine::new("3", 11),
                    OrderLine::new("1", 23),
                    OrderLine::new("2", 2),
                ],
            ),
        );
        m.insert(s("5"), Order::new(2019, 1, 5, &[OrderLine::new("4", 3)]));
        m
    };
}
