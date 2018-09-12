use super::{InvoiceSummary, OrderInfo, OrderNumber};
use money_serde::MoneyDef;

use billable_item::BillableItem;
use steel_cent::currency::USD;
use steel_cent::Money;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invoice {
    order_info: OrderInfo,
    items: Vec<BillableItem>,
    #[serde(with = "MoneyDef")]
    estimated_shipping_cost: Money,
}

impl Invoice {
    pub fn new(order_number: OrderNumber) -> Self {
        Self {
            order_info: OrderInfo::new(order_number),
            items: Vec::<BillableItem>::new(),
            estimated_shipping_cost: Money::zero(USD),
        }
    }

    pub fn order_info(&self) -> &OrderInfo {
        &self.order_info
    }
    
    pub fn mut_order_info(&mut self) -> &mut OrderInfo {
        &mut self.order_info
    }

    pub fn summary(&self) -> InvoiceSummary {
        InvoiceSummary::new(
            self.total_pieces(),
            self.estimated_shipping_cost,
            self.sub_total_cost(),
            self.sales_tax_cost(),
            self.total_cost(),
        )
    }

    pub fn add_billable_item(&mut self, item: BillableItem) {
        self.items.push(item);
    }

    pub fn remove_billable_item(&mut self, item_index: usize) {
        self.items.remove(item_index);
    }

    pub fn remove_billable_items(&mut self) {
        self.items.clear();
    }

    // TODO - use a consistent naming convention
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

    pub fn sub_total_cost(&self) -> Money {
        let mut sum = Money::zero(USD);

        for i in &self.items {
            sum = sum + i.cost();
        }

        sum = sum + self.estimated_shipping_cost;

        sum
    }

    // TODO - sales tax provider, hard-coded to 8.8%
    pub fn sales_tax_cost(&self) -> Money {
        self.sub_total_cost() * 0.088
    }

    pub fn total_cost(&self) -> Money {
        self.sub_total_cost() + self.sales_tax_cost()
    }
}
