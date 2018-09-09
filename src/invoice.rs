use billable_item::BillableItem;
use chrono::prelude::*;
use dim::ucum;
use steel_cent::currency::USD;
use steel_cent::SmallMoney;

#[derive(Clone, PartialEq, Debug)]
pub struct OrderInfo {
    // TODO - customer lookup/db
    customer: String,
    confirms_with: String,
    order_date: DateTime<Utc>,
    shipment_date: DateTime<Utc>,
    // TODO - proper type
    order_number: u32,
    weight_estimate: ucum::Gram<f64>,
    will_call: bool,
}

#[derive(Clone, Debug)]
pub struct Invoice {
    order_info: OrderInfo,
    items: Vec<BillableItem>,
    estimated_shipping_cost: SmallMoney,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Summary {
    total_pieces: usize,
    estimated_shipping_cost: SmallMoney,
    sub_total_cost: SmallMoney,
    sales_tax_cost: SmallMoney,
    total_cost: SmallMoney,
}

impl Invoice {
    pub fn new() -> Self {
        Self {
            order_info: OrderInfo::default(),
            items: Vec::<BillableItem>::new(),
            // TODO
            estimated_shipping_cost: SmallMoney::zero(USD),
        }
    }

    pub fn order_info(&self) -> &OrderInfo {
        &self.order_info
    }

    pub fn summary(&self) -> Summary {
        Summary {
            total_pieces: self.total_pieces(),
            estimated_shipping_cost: self.estimated_shipping_cost,
            sub_total_cost: self.sub_total_cost(),
            sales_tax_cost: self.sales_tax_cost(),
            total_cost: self.total_cost(),
        }
    }

    pub fn add_billable_item(&mut self, item: BillableItem) {
        self.items.push(item);
    }

    pub fn remove_billable_item(&mut self, item_index: usize) {
        self.items.remove(item_index);
    }

    pub fn get_mut_billable_item(&mut self, item_index: usize) -> &mut BillableItem {
        &mut self.items[item_index]
    }

    pub fn items(&self) -> &[BillableItem] {
        &self.items
    }

    pub fn total_pieces(&self) -> usize {
        let mut sum: usize = 0;

        for i in self.items().iter() {
            sum += i.quantity();
        }

        sum
    }

    pub fn sub_total_cost(&self) -> SmallMoney {
        let mut sum = SmallMoney::zero(USD);

        for i in &self.items {
            sum = sum + i.cost();
        }

        sum = sum + self.estimated_shipping_cost;

        sum
    }

    // TODO - sales tax provider, hard-coded to 8.8%
    pub fn sales_tax_cost(&self) -> SmallMoney {
        self.sub_total_cost() * 0.088
    }

    pub fn total_cost(&self) -> SmallMoney {
        self.sub_total_cost() + self.sales_tax_cost()
    }
}

impl Default for OrderInfo {
    fn default() -> Self {
        OrderInfo {
            customer: String::from("NEW CUSTOMER"),
            confirms_with: String::new(),
            order_date: Utc::now(),
            shipment_date: Utc::now(),
            // TODO - proper type
            order_number: 1,
            weight_estimate: ucum::Gram::new(0.0),
            will_call: false,
        }
    }
}

impl Default for Summary {
    fn default() -> Self {
        Summary {
            total_pieces: 0,
            estimated_shipping_cost: SmallMoney::zero(USD),
            sub_total_cost: SmallMoney::zero(USD),
            sales_tax_cost: SmallMoney::zero(USD),
            total_cost: SmallMoney::zero(USD),
        }
    }
}
