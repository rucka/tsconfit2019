use crate::api::*;
use crate::data::{get_book, get_order};
use futures::prelude::*;

async fn book_service(id: &String) -> Option<&'static Book> {
    get_book(id)
}

async fn order_service(id: &String) -> Option<&'static Order> {
    get_order(id)
}

async fn validation_service(order: Option<&Order>) -> Result<&Order, OrderNotValid> {
    match order {
        Some(o) => match validate_order(o) {
            Ok(_) => Ok(o),
            Err(err) => Err(err),
        },
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

pub struct FpProcessor {}

impl AsyncProcessor for FpProcessor {
    fn process(&self, order_id: &'static String) -> ProcessResult {
        Box::pin(
            order_service(order_id)
                .then(|order| validation_service(order))
                .and_then(|validated| place_order_service(validated))
                .and_then(|result| futures::future::ready(Ok(result.amount)))
                .map_err(|_| ()),
        )
    }
}

impl FpProcessor {
    pub fn processor() -> &'static dyn AsyncProcessor {
        &(FpProcessor {}) as &dyn AsyncProcessor
    }
}

pub fn process_fp_direct(order_id: &'static String) -> impl Future<Output = Result<f64, ()>> {
    order_service(order_id)
        .then(|order| validation_service(order))
        .and_then(|validated| place_order_service(validated))
        .and_then(|result| futures::future::ready(Ok(result.amount)))
        .map_err(|_| ())
}
