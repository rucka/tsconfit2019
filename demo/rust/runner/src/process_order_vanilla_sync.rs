use crate::api::*;
use crate::data::{get_book, get_order};

fn book_service(id: &String) -> Option<&'static Book> {
    get_book(id)
}

fn order_service(id: &String) -> Option<&'static Order> {
    get_order(id)
}

fn validation_service(order: &Order) -> Result<&Order, OrderNotValid> {
    match validate_order(order) {
        Ok(_) => Ok(order),
        Err(err) => Err(err),
    }
}

fn calculate_amount_service(order: &Order) -> f64 {
    let mut total = 0.0;
    for item in &order.items {
        let book = book_service(&item.book_id);
        match book {
            Some(b) => {
                total += item.quantity as f64 * b.price;
            }
            _ => {}
        };
    }
    total
}

fn place_order_service(order: &Order) -> PlacedOrderResult {
    let amount = calculate_amount_service(order);
    Ok(OrderSuccessful::new(amount))
}

pub struct VanillaProcessorSync {}

impl SyncProcessor for VanillaProcessorSync {
    fn process(&self, order_id: &String) -> Result<f64, ()> {
        match order_service(order_id) {
            Some(order) => {
                let validation = validation_service(&order);
                match validation {
                    Ok(order) => match place_order_service(order) {
                        Ok(res) => Ok(res.amount),
                        Err(_) => Err(()),
                    },
                    _ => Err(()),
                }
            }
            _ => Err(()),
        }
    }
}

impl VanillaProcessorSync {
    pub fn processor() -> &'static dyn SyncProcessor {
        &(VanillaProcessorSync {}) as &dyn SyncProcessor
    }
}

pub fn process_syncv_direct(order_id: &String) -> Result<f64, ()> {
    match order_service(order_id) {
        Some(order) => {
            let validation = validation_service(&order);
            match validation {
                Ok(order) => match place_order_service(order) {
                    Ok(res) => Ok(res.amount),
                    Err(_) => Err(()),
                },
                _ => Err(()),
            }
        }
        _ => Err(()),
    }
}
