use crate::data::orders::*;
use crate::data::stock::*;
use crate::data::*;
use crate::data::*;
use crate::models::Order;
use crate::models::Stock;
use futures::StreamExt;
use serde_derive::{Deserialize, Serialize};
use serde_json;

pub fn complete_fulfill_order(id: i32) -> Result<(), ()> {
    let connection = get_connection();

    // Get Order amount
    let order = orders::fulfill_order(&connection, id);

    let result = match order {
        Some(o) => {
            let stocks = get_stock(&connection, id);

            if stocks.len() == 1 {
                // if order amount is <= stock amount

                if o.amount <= stocks[0].amount {
                    //      Decrement stock amount by order amount
                    increment_stock(&connection, stocks[0].id, stocks[0].amount);

                    //      Set order to fulfilled.
                    orders::fulfill_order(&connection, o.id);
                }
            }
            Ok(())
        }
        None => Err(()),
    };

    result
}
