use crate::api::*;
use crate::data::{get_book, get_order};
use fp_core::compose;
use fp_core::compose::compose_two;
use futures::future::*;
use futures::prelude::*;
use std::future::Future;

fn book_service(id: &String) -> impl Future<Output = Option<&'static Book>> {
    ready(get_book(id))
}

fn order_service(id: &String) -> impl Future<Output = Option<&'static Order>> {
    ready(get_order(id))
}

fn validation_service(
    order: impl Future<Output = Option<&'static Order>>,
) -> impl Future<Output = Result<&'static Order, OrderNotValid>> {
    order.map(|order| match order {
        Some(o) => validate_order(o),
        None => Err(OrderNotValid::BookNotExists),
    })
}

fn calculate_amount_service(
    order: &'static Order,
) -> impl Future<Output = Result<f64, OrderNotValid>> {
    async move {
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
        Ok(total)
    }
}

fn place_order_service(
    order: impl Future<Output = Result<&'static Order, OrderNotValid>>,
) -> impl Future<Output = Result<f64, OrderNotValid>> {
    order.and_then(calculate_amount_service)
}

fn map_order_amount(
    order_result: impl Future<Output = Result<f64, OrderNotValid>>,
) -> impl Future<Output = Result<f64, ()>> {
    order_result.map_err(|_| ())
}

pub struct FpProcessor {}

impl AsyncProcessor for FpProcessor {
    fn process(&self, order_id: &'static String) -> ProcessResult {
        Box::pin(compose!(
            order_service,
            validation_service,
            place_order_service,
            map_order_amount
        )(order_id))
    }
}

impl FpProcessor {
    pub fn processor() -> &'static dyn AsyncProcessor {
        &(FpProcessor {}) as &dyn AsyncProcessor
    }
}

pub fn process_fp_direct(order_id: &'static String) -> impl Future<Output = Result<f64, ()>> {
    compose!(
        order_service,
        validation_service,
        place_order_service,
        map_order_amount
    )(order_id)
}
