use crate::api::*;
use crate::data::{get_book, get_order};
use async_trait::async_trait;

async fn book_service(id: &String) -> Option<&'static Book> {
    get_book(id)
}

async fn order_service(id: &String) -> Option<&'static Order> {
    get_order(id)
}

async fn validation_service(order: &Order) -> Result<&Order, OrderNotValid> {
    match validate_order(order) {
        Ok(_) => Ok(order),
        Err(err) => Err(err),
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
    let result = calculate_amount_service(order).await;
    Ok(OrderSuccessful::new(result.0))
}

pub struct VanillaProcessor {}

#[async_trait]
impl AsyncProcessor for VanillaProcessor {
    async fn process(&self, order_id: &String) -> f64 {
        let order = order_service(order_id).await;
        match order {
            Some(order) => {
                let validation = validation_service(&order).await;
                match validation {
                    Ok(_) => match place_order_service(order).await {
                        Ok(res) => res.amount,
                        Err(_) => 0.0,
                    },
                    _ => 0.0,
                }
            }
            _ => 0.0,
        }
    }
}

impl VanillaProcessor {
    pub fn processor() -> &'static dyn AsyncProcessor {
        &(VanillaProcessor {}) as &dyn AsyncProcessor
    }
}
