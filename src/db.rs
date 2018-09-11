use super::{Invoice, OrderNumber};

use rustbreak::deser::Bincode;
use rustbreak::FileDatabase;
use rustbreak::RustbreakError;
use std::collections::HashMap;
use std::path;

// TODO - customer struct and db
// TODO vec<> or top level struct?

//type InvoiceDB = FileDatabase<HashMap<OrderNumber, Invoice>, Bincode>;

// TODO - lumber type needs re-worked to be populated by the db
//type LumberTypeDB = FileDatabase<Vec<LumberType>, Bincode>;

pub type Database = FileDatabase<DatabaseData, Bincode>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseData {
    invoices: HashMap<OrderNumber, Invoice>,
}

impl DatabaseData {
    pub fn new() -> Self {
        DatabaseData {
            invoices: HashMap::new(),
        }
    }
}

pub fn from_path<S>(path: S) -> Result<Database, RustbreakError>
where
    S: AsRef<path::Path>,
{
    FileDatabase::from_path(path, DatabaseData::new())
}
