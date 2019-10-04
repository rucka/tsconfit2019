use crate::api::*;
use crate::data::{get_book, get_order};
use futures::future::*;
use futures::prelude::*;

/*
macro_rules! compose_future_result_tail {
    ( $last:expr ) => { and_then($last) };
    ( $head:expr, $($tail:expr), +) => {
        and_then($head) . compose_future_result_tail!($($tail),+)
    };
}

macro_rules! compose_future_result {
    ( $unique:expr ) => { $unique };
    ( $head:expr, $($tail:expr), +) => {
        $head . compose_future_result_tail!($($tail),+)
    };
}
*/

/*
fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}
*/

fn book_service(id: &String) -> impl Future<Output = Result<&'static Book, OrderNotValid>> {
    ready(match get_book(id) {
        Some(b) => Ok(b),
        None => Err(OrderNotValid::BookNotExists),
    })
}

fn order_service(id: &String) -> impl Future<Output = Result<&'static Order, OrderNotValid>> {
    ready(match get_order(id) {
        Some(b) => Ok(b),
        None => Err(OrderNotValid::BookNotExists),
    })
}

fn validation_service(
    order: &'static Order,
) -> impl Future<Output = Result<&'static Order, OrderNotValid>> {
    ready(validate_order(order))
}

fn calculate_amount_service(
    order: &'static Order,
) -> impl Future<Output = Result<f64, OrderNotValid>> {
    async move {
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
    }
}

fn place_order_service(order: &'static Order) -> impl Future<Output = Result<f64, OrderNotValid>> {
    calculate_amount_service(order)
}

pub struct FpProcessor {}

impl AsyncProcessor for FpProcessor {
    fn process(&self, order_id: &'static String) -> ProcessResult {
        Box::pin(
            order_service(order_id)
                .and_then(validation_service)
                .and_then(place_order_service)
                .map_err(|_| ())
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
    /*
    compose_future_result!(
        order_service(order_id),
        &validation_service,
        &place_order_service,
        &map_order_amount
    )
    */
    order_service(order_id)
        .and_then(validation_service)
        .and_then(place_order_service)
        .map_err(|_| ())
}
