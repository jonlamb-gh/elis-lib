// TODO - proper types for address, numbers, etc

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SiteInfo {
    site_name: String,
    address: String,
    phone_number: String,
    fax_number: String,
    sales_tax: f64,
}

impl SiteInfo {
    pub fn new(
        site_name: String,
        address: String,
        phone_number: String,
        fax_number: String,
        sales_tax: f64,
    ) -> Self {
        Self {
            site_name,
            address,
            phone_number,
            fax_number,
            sales_tax,
        }
    }

    pub fn site_name(&self) -> &str {
        &self.site_name
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

impl Default for SiteInfo {
    fn default() -> SiteInfo {
        SiteInfo {
            site_name: String::from("SITE NAME"),
            address: String::from("ADDRESS"),
            phone_number: String::from("PHONE NUMBER"),
            fax_number: String::from("FAX NUMBER"),
            sales_tax: 0.0,
        }
    }
}
