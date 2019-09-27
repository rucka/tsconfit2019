use crate::api::*;
use crate::data::{get_book, get_order};

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

async fn process_async(order_id: &'static String) -> Result<f64, ()> {
    let order = order_service(order_id).await;
    let order = validation_service(order).await.map_err(|_| ())?;
    Ok(place_order_service(order).await.map_err(|_| ())?.amount)
}

pub struct IdiomaticProcessor {}

impl AsyncProcessor for IdiomaticProcessor {
    fn process(&self, order_id: &'static String) -> ProcessResult {
        Box::pin(process_async(order_id))
    }
}

impl IdiomaticProcessor {
    pub fn processor() -> &'static dyn AsyncProcessor {
        &(IdiomaticProcessor {}) as &dyn AsyncProcessor
    }
}
