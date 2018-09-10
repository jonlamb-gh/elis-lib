// TODO - proper types for address, numbers, etc

#[derive(Clone, Debug)]
pub struct SiteInfo {
    site_name: String,
    address: String,
    phone_number: String,
    fax_number: String,
    sales_tax: f64,
}

impl Default for SiteInfo {
    fn default() -> Self {
        SiteInfo {
            site_name: String::from("NA"),
            address: String::from("NA"),
            phone_number: String::from("NA"),
            fax_number: String::from("NA"),
            sales_tax: 0.0,
        }
    }
}

impl SiteInfo {
    pub fn site_name(&self) -> &str {
        &self.address
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    pub fn fax_number(&self) -> &str {
        &self.fax_number
    }

    pub fn sales_tax(&self) -> f64 {
        self.sales_tax
    }
}
