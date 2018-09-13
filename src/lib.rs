// TODO - update style, function naming scheme
// new/default usage
// https://doc.rust-lang.org/1.0.0/style/style/naming/README.html#avoid-redundant-prefixes-[rfc-356]
// https://doc.rust-lang.org/1.0.0/style/style/naming/README.html#getter/setter-methods-[rfc-344]
// use of super::type vs module::type for local modules

extern crate chrono;
extern crate dimensioned as dim;
extern crate rustbreak;
#[macro_use]
extern crate serde_derive;
extern crate serde;
pub extern crate steel_cent;

pub use dim::ucum;
pub use rustbreak::RustbreakError;

mod billable_item;
mod board_dimensions;
mod customer_info;
mod database;
mod invoice;
mod invoice_summary;
pub mod lumber;
mod money_serde;
mod order_info;
mod site_info;

pub use self::billable_item::BillableItem;
pub use self::board_dimensions::BoardDimensions;
pub use self::customer_info::CustomerInfo;
pub use self::database::{database_from_path, Database, DatabaseData};
pub use self::invoice::Invoice;
pub use self::invoice_summary::InvoiceSummary;
pub use self::order_info::OrderInfo;
pub use self::site_info::{SalesTaxReader, SiteInfo};

pub type OrderNumber = u32;
