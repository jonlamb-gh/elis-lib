use super::OrderNumber;

use chrono::prelude::*;
use dim::ucum;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderInfo {
    customer_name: String,
    confirms_with: String,
    order_date: DateTime<Utc>,
    shipment_date: DateTime<Utc>,
    order_number: OrderNumber,
    weight_estimate: ucum::Gram<f64>,
    site_name: String,
    will_call: bool,
}

impl OrderInfo {
    pub fn new(order_number: OrderNumber) -> Self {
        Self {
            customer_name: String::from("SELECT CUSTOMER"),
            confirms_with: String::new(),
            order_date: Utc::now(),
            shipment_date: Utc::now(),
            order_number,
            weight_estimate: ucum::Gram::new(0.0),
            site_name: String::from("SELECT SITE"),
            will_call: false,
        }
    }

    pub fn customer_name(&self) -> &str {
        &self.customer_name
    }

    pub fn set_customer_name(&mut self, customer_name: String) {
        self.customer_name = customer_name;
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

    pub fn site_name(&self) -> &str {
        &self.site_name
    }

    pub fn set_site_name(&mut self, site_name: String) {
        self.site_name = site_name;
    }

    pub fn will_call(&self) -> bool {
        self.will_call
    }
}

impl Default for OrderInfo {
    fn default() -> OrderInfo {
        OrderInfo {
            customer_name: String::from("SELECT CUSTOMER"),
            confirms_with: String::new(),
            order_date: Utc::now(),
            shipment_date: Utc::now(),
            order_number: 0,
            weight_estimate: ucum::Gram::new(0.0),
            site_name: String::from("SELECT SITE"),
            will_call: false,
        }
    }
}
