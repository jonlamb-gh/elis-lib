use steel_cent::currency::USD;
use steel_cent::SmallMoney;

#[derive(Clone, PartialEq, Debug)]
pub struct InvoiceSummary {
    total_pieces: usize,
    estimated_shipping_cost: SmallMoney,
    sub_total_cost: SmallMoney,
    sales_tax_cost: SmallMoney,
    total_cost: SmallMoney,
}

impl Default for InvoiceSummary {
    fn default() -> Self {
        InvoiceSummary {
            total_pieces: 0,
            estimated_shipping_cost: SmallMoney::zero(USD),
            sub_total_cost: SmallMoney::zero(USD),
            sales_tax_cost: SmallMoney::zero(USD),
            total_cost: SmallMoney::zero(USD),
        }
    }
}

impl InvoiceSummary {
    pub fn new(
        total_pieces: usize,
        estimated_shipping_cost: SmallMoney,
        sub_total_cost: SmallMoney,
        sales_tax_cost: SmallMoney,
        total_cost: SmallMoney,
    ) -> Self {
        InvoiceSummary {
            total_pieces,
            estimated_shipping_cost,
            sub_total_cost,
            sales_tax_cost,
            total_cost,
        }
    }

    pub fn total_pieces(&self) -> usize {
        self.total_pieces
    }

    pub fn estimated_shipping_cost(&self) -> &SmallMoney {
        &self.estimated_shipping_cost
    }

    pub fn sub_total_cost(&self) -> &SmallMoney {
        &self.sub_total_cost
    }

    pub fn sales_tax_cost(&self) -> &SmallMoney {
        &self.sales_tax_cost
    }

    pub fn total_cost(&self) -> &SmallMoney {
        &self.total_cost
    }
}
