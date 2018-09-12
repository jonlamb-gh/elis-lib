use super::OrderNumber;

use chrono::prelude::*;
use dim::ucum;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderInfo {
    // TODO - customer lookup/db
    customer: String,
    confirms_with: String,
    order_date: DateTime<Utc>,
    shipment_date: DateTime<Utc>,
    order_number: OrderNumber,
    weight_estimate: ucum::Gram<f64>,
    will_call: bool,
    // TODO - Site, also populates Site Info page
}

impl Default for OrderInfo {
    fn default() -> Self {
        OrderInfo {
            customer: String::from("NEW CUSTOMER"),
            confirms_with: String::new(),
            order_date: Utc::now(),
            shipment_date: Utc::now(),
            order_number: 0,
            weight_estimate: ucum::Gram::new(0.0),
            will_call: false,
        }
    }
}

impl OrderInfo {
    pub fn new(order_number: OrderNumber) -> Self {
        Self {
            customer: String::from("NEW CUSTOMER"),
            confirms_with: String::new(),
            order_date: Utc::now(),
            shipment_date: Utc::now(),
            order_number,
            weight_estimate: ucum::Gram::new(0.0),
            will_call: false,
        }
    }

    pub fn customer(&self) -> &str {
        &self.customer
    }

    pub fn confirms_with(&self) -> &str {
        &self.confirms_with
    }

    pub fn order_date(&self) -> &DateTime<Utc> {
        &self.order_date
    }

    pub fn shipment_date(&self) -> &DateTime<Utc> {
        &self.shipment_date
    }

    pub fn order_number(&self) -> OrderNumber {
        self.order_number
    }

    pub fn weight_estimate(&self) -> &ucum::Gram<f64> {
        &self.weight_estimate
    }

    pub fn will_call(&self) -> bool {
        self.will_call
    }
}
