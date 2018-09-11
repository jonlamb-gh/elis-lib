use steel_cent::currency::USD;
use steel_cent::Money;

#[derive(Clone, PartialEq, Debug)]
pub struct InvoiceSummary {
    total_pieces: usize,
    estimated_shipping_cost: Money,
    sub_total_cost: Money,
    sales_tax_cost: Money,
    total_cost: Money,
}

impl Default for InvoiceSummary {
    fn default() -> Self {
        InvoiceSummary {
            total_pieces: 0,
            estimated_shipping_cost: Money::zero(USD),
            sub_total_cost: Money::zero(USD),
            sales_tax_cost: Money::zero(USD),
            total_cost: Money::zero(USD),
        }
    }
}

impl InvoiceSummary {
    pub fn new(
        total_pieces: usize,
        estimated_shipping_cost: Money,
        sub_total_cost: Money,
        sales_tax_cost: Money,
        total_cost: Money,
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

    pub fn estimated_shipping_cost(&self) -> &Money {
        &self.estimated_shipping_cost
    }

    pub fn sub_total_cost(&self) -> &Money {
        &self.sub_total_cost
    }

    pub fn sales_tax_cost(&self) -> &Money {
        &self.sales_tax_cost
    }

    pub fn total_cost(&self) -> &Money {
        &self.total_cost
    }
}
