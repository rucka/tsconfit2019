use crate::api::*;
use crate::data::{get_book, get_order};

async fn book_service(id: &String) -> Option<&'static Book> {
    get_book(id)
}

async fn order_service(id: &String) -> Option<&'static Order> {
    get_order(id)
}

async fn validation_service(order: &Order) -> Result<&Order, OrderNotValid> {
    validate_order(order)
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
    let result = calculate_amount_service(order).await;
    Ok(OrderSuccessful::new(result.0))
}

pub async fn process_vanilla_direct(order_id: &String) -> Result<f64, ()> {
    match order_service(order_id).await {
        Some(order) => {
            let validation = validation_service(&order).await;
            match validation {
                Ok(order) => match place_order_service(order).await {
                    Ok(res) => Ok(res.amount),
                    Err(_) => Err(()),
                },
                _ => Err(()),
            }
        }
        _ => Err(()),
    }
}

pub struct VanillaProcessor {}

impl AsyncProcessor for VanillaProcessor {
    fn process(&self, order_id: &'static String) -> ProcessResult {
        Box::pin(process_vanilla_direct(order_id))
    }
}

impl VanillaProcessor {
    pub fn processor() -> &'static dyn AsyncProcessor {
        &(VanillaProcessor {}) as &dyn AsyncProcessor
    }
}
