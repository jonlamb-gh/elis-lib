use super::OrderNumber;
use customer_info::CustomerInfo;
use invoice::Invoice;
use lumber::{Lumber, LumberType};
use site_info::SiteInfo;

use rustbreak::deser::Bincode;
use rustbreak::FileDatabase;
use rustbreak::RustbreakError;
use std::collections::HashMap;
use std::path;
use steel_cent::Money;

// result?
pub trait SiteSalesTaxProvider {
    fn sales_tax(&self, site_name: &str) -> f64;
}

// result?
pub trait LumberFobCostProvider {
    fn fob_cost(&self, lumber_type: &LumberType) -> Money;
}

pub type Database = FileDatabase<DatabaseData, Bincode>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseData {
    pub site_info: SiteInfo,
    pub lumber_types: HashMap<String, Lumber>,
    pub customers: HashMap<String, CustomerInfo>,
    pub invoices: HashMap<OrderNumber, Invoice>,
}

impl DatabaseData {
    pub fn new() -> Self {
        DatabaseData {
            site_info: SiteInfo::default(),
            invoices: HashMap::new(),
            customers: HashMap::new(),
            lumber_types: HashMap::new(),
        }
    }

    pub fn next_free_order_number(&self) -> OrderNumber {
        let mut next_num: OrderNumber = 0;
        for order_num in self.invoices.keys() {
            if *order_num > next_num {
                next_num = *order_num;
            }
        }

        next_num + 1
    }
}

pub fn database_from_path<S>(path: S) -> Result<Database, RustbreakError>
where
    S: AsRef<path::Path>,
{
    FileDatabase::from_path(path, DatabaseData::new())
}
