// TODO - move the reader trait declarations into this scope?
// name them *provider?

use super::{lumber::Lumber, CustomerInfo, Invoice, OrderNumber, SiteInfo};

use rustbreak::deser::Bincode;
use rustbreak::FileDatabase;
use rustbreak::RustbreakError;
use std::collections::HashMap;
use std::path;

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
            site_info: SiteInfo::new(),
            invoices: HashMap::new(),
            customers: HashMap::new(),
            lumber_types: HashMap::new(),
        }
    }
}

pub fn database_from_path<S>(path: S) -> Result<Database, RustbreakError>
where
    S: AsRef<path::Path>,
{
    FileDatabase::from_path(path, DatabaseData::new())
}
