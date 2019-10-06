use crate::api::*;
use crate::data::{get_book, get_order};
use futures::future::*;
use std::pin::Pin;

fn book_service(
    id: &String,
) -> Pin<Box<dyn Future<Output = Result<&'static Book, OrderNotValid>>>> {
    Box::pin(ready(match get_book(id) {
        Some(b) => Ok(b),
        None => Err(OrderNotValid::BookNotExists),
    }))
}

fn order_service(
    id: &String,
) -> Pin<Box<dyn Future<Output = Result<&'static Order, OrderNotValid>>>> {
    Box::pin(ready(match get_order(id) {
        Some(b) => Ok(b),
        None => Err(OrderNotValid::BookNotExists),
    }))
}

fn validation_service(
    order: &'static Order,
) -> Pin<Box<dyn Future<Output = Result<&'static Order, OrderNotValid>>>> {
    Box::pin(ready(validate_order(order)))
}

fn calculate_amount_service(
    order: &'static Order,
) -> Pin<Box<dyn Future<Output = Result<f64, OrderNotValid>>>> {
    Box::pin(async move {
        let mut total = 0.0;
        for item in &order.items {
            let book = book_service(&item.book_id).await;
            match book {
                Ok(b) => {
                    total += item.quantity as f64 * b.price;
                }
                _ => {}
            };
        }
        Ok(total)
    })
}

fn place_order_service(
    order: &'static Order,
) -> Pin<Box<dyn Future<Output = Result<f64, OrderNotValid>>>> {
    calculate_amount_service(order)
}

pub struct FutureDynProcessor {}

impl AsyncProcessor for FutureDynProcessor {
    fn process(&self, order_id: &'static String) -> ProcessResult {
        Box::pin(
            order_service(order_id)
                .and_then(validation_service)
                .and_then(place_order_service)
                .map_err(|_| ()),
        )
    }
}

impl FutureDynProcessor {
    pub fn processor() -> &'static dyn AsyncProcessor {
        &(FutureDynProcessor {}) as &dyn AsyncProcessor
    }
}

pub fn process_future_dyn_direct(
    order_id: &'static String,
) -> impl Future<Output = Result<f64, ()>> {
    order_service(order_id)
        .and_then(validation_service)
        .and_then(place_order_service)
        .map_err(|_| ())
}
