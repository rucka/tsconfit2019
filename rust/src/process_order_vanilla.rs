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

async fn calculate_amount_service(order: &Order) -> f64 {
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
    total
}

async fn place_order_service(order: &Order) -> PlaceOrderResult {
    let amount = calculate_amount_service(order).await;
    Ok(OrderSuccessful::new(amount))
}

pub struct VanillaProcessOrder {}

#[async_trait]
impl Processor for VanillaProcessOrder {
    async fn process(&self, _order_id: &String) -> () {
        ()
    }
}
#[async_trait]
impl ProcessOrder for VanillaProcessOrder {
    async fn process(&self, order_id: &String) -> PlaceOrderResult {
        let order = get_order(order_id);
        match order {
            None => Err(OrderNotValid::NoItems),
            Some(o) => match validate_order(o) {
                Ok(_) => Ok(OrderSuccessful::new(0.0)),
                Err(err) => Err(err),
            },
        }
    }
}

impl VanillaProcessOrder {
    pub fn process_order() -> &'static dyn ProcessOrder {
        &(VanillaProcessOrder {}) as &dyn ProcessOrder
    }
    pub fn processor() -> &'static dyn Processor {
        &(VanillaProcessOrder {}) as &dyn Processor
    }
}
