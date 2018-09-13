use super::{InvoiceSummary, OrderInfo, OrderNumber, SalesTaxReader};
use billable_item::BillableItem;
use lumber::FobCostReader;
use money_serde::MoneyDef;

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

    pub fn summary<T>(&self, fob_reader: &T) -> InvoiceSummary
    where
        T: FobCostReader + SalesTaxReader,
    {
        InvoiceSummary::new(
            self.total_pieces(),
            self.estimated_shipping_cost,
            self.sub_total_cost(fob_reader),
            self.sales_tax_cost(fob_reader),
            self.total_cost(fob_reader),
        )
    }

    pub fn add_billable_item(&mut self, item: BillableItem) {
        self.items.push(item);
    }

    pub fn remove_billable_item(&mut self, item_index: usize) {
        self.items.remove(item_index);
    }

    pub fn clear_billable_items(&mut self) {
        self.items.clear();
    }

    pub fn get_billable_item_mut(&mut self, item_index: usize) -> &mut BillableItem {
        &mut self.items[item_index]
    }

    pub fn billable_items(&self) -> &[BillableItem] {
        &self.items
    }

    pub fn total_pieces(&self) -> usize {
        let mut sum: usize = 0;

        for i in &self.items {
            sum += i.quantity();
        }

        sum
    }

    pub fn sub_total_cost<T: FobCostReader>(&self, fob_reader: &T) -> Money {
        let mut sum = Money::zero(USD);

        for i in &self.items {
            sum = sum + i.cost(fob_reader);
        }

        sum = sum + self.estimated_shipping_cost;

        sum
    }

    // TODO - result check traits?
    pub fn sales_tax_cost<T>(&self, fob_reader: &T) -> Money
    where
        T: FobCostReader + SalesTaxReader,
    {
        let sales_tax: f64 = fob_reader.sales_tax(self.order_info.site_name());

        self.sub_total_cost(fob_reader) * sales_tax
    }

    pub fn total_cost<T>(&self, fob_reader: &T) -> Money
    where
        T: FobCostReader + SalesTaxReader,
    {
        self.sub_total_cost(fob_reader) + self.sales_tax_cost(fob_reader)
    }
}
