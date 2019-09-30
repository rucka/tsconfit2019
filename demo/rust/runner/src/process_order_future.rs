use crate::api::*;
use crate::data::{get_book, get_order};
use futures::prelude::*;

async fn book_service(id: &String) -> Option<&'static Book> {
    get_book(id)
}

async fn order_service(id: &String) -> Option<&'static Order> {
    get_order(id)
}

async fn validation_service<'a>(order: Option<&'a Order>) -> ValidationResult<'a> {
    match order {
        Some(o) => validate_order(o),
        None => Err(OrderNotValid::BookNotExists),
    }
}

async fn calculate_amount_service(order: &Order) -> (f64, &Order) {
    let mut total = 0.0;
    for item in &order.items {
        let book = book_service(&item.book_id).await;
        match book {
            Some(b) => {
                total += item.quantity as f64 * b.price;
            }
            _ => {}
        };
    }
    (total, order)
}

async fn place_order_service(order: &Order) -> PlacedOrderResult {
    Ok(OrderSuccessful::new(
        calculate_amount_service(order).await.0,
    ))
}

pub struct FutureProcessor {}

impl AsyncProcessor for FutureProcessor {
    fn process(&self, order_id: &'static String) -> ProcessResult {
        Box::pin(
            order_service(order_id)
                .then(validation_service)
                .and_then(place_order_service)
                .and_then(|result| futures::future::ready(Ok(result.amount)))
                .map_err(|_| ()),
        )
    }
}

impl FutureProcessor {
    pub fn processor() -> &'static dyn AsyncProcessor {
        &(FutureProcessor {}) as &dyn AsyncProcessor
    }
}

pub fn process_future_direct(order_id: &'static String) -> impl Future<Output = Result<f64, ()>> {
    order_service(order_id)
        .then(validation_service)
        .and_then(place_order_service)
        .and_then(|result| futures::future::ready(Ok(result.amount)))
        .map_err(|_| ())
}
