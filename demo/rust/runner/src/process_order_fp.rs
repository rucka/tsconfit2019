use crate::api::*;
use crate::data::{get_book, get_order};
use futures::future::*;
use std::pin::Pin;

fn composer<T1: 'static, T2: 'static, E: 'static>(
    input: Pin<Box<dyn Future<Output = Result<T1, E>>>>,
    transform: &'static impl Fn(T1) -> Pin<Box<dyn Future<Output = Result<T2, E>>>>,
) -> Pin<Box<dyn Future<Output = Result<T2, E>>>> {
    Box::pin(input.and_then(transform))
}

fn compose_two<T1: 'static, T2: 'static, T3: 'static, E: 'static>(
    transform1: &'static impl Fn(T1) -> Pin<Box<dyn Future<Output = Result<T2, E>>>>,
    transform2: &'static impl Fn(T2) -> Pin<Box<dyn Future<Output = Result<T3, E>>>>,
) -> impl Fn(Pin<Box<dyn Future<Output = Result<T1, E>>>>) -> Pin<Box<dyn Future<Output = Result<T3, E>>>>
{
    move |x| composer(composer(x, transform1), transform2)
}

macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

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

pub struct FpProcessor {}

impl AsyncProcessor for FpProcessor {
    fn process(&self, order_id: &'static String) -> ProcessResult {
        Box::pin(
            compose!(&validation_service, &place_order_service)(order_service(order_id))
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
    compose!(&validation_service, &place_order_service)(order_service(order_id)).map_err(|_| ())
}
