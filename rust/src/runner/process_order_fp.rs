use crate::runner::api::*;
use crate::runner::data::{get_book, get_order};
use async_trait::async_trait;
// use futures::future::*;
// use futures::FutureExt;

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
    Ok(OrderSuccessful::new(
        calculate_amount_service(order).await.0,
    ))
}

/*
fn process(
    order_id: &String,
) -> std::pin::Pin<std::boxed::Box<dyn Future<Output = f64> + std::marker::Send>> {
    order_service(order_id)
        .then(|order| validation_service(order.unwrap()))
        .then(|validation_result| match validation_result {
            Ok(order) => (place_order_service(order).then(|res| match res {
                Ok(success) => futures::future::ready(success.amount).left_future(),
                Err(_) => futures::future::ready(0.0).right_future(),
            }))
            .left_future(),
            Err(_) => futures::future::ready(0.0).right_future(),
        })
        .boxed()

    //place_order_service(order)
    //   .and_then(|orderResult| futures::future::ready(orderResult.amount))
    //   .or_else(|err| futures::future::ready(0.0))

    //.or_else(|err| futures::future::ready(0.0))
    //.map(|validationResult| match validationResult {
    //    Ok(order) => place_order_service(order).map(|res| match res {
    //        Ok(_res) => 0.0,
    //        Err(_) => 0.0,
    //    }),
    //    Err(_) => 0.0,
    //});
}
*/

pub struct FpProcessor {}

#[async_trait]
impl AsyncProcessor for FpProcessor {
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

impl FpProcessor {
    pub fn processor() -> &'static dyn AsyncProcessor {
        &(FpProcessor {}) as &dyn AsyncProcessor
    }
}
