// TODO - proper types for address, numbers, etc

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerInfo {
    name: String,
    address: String,
    phone_number: String,
    notes: String,
}

impl CustomerInfo {
    pub fn new() -> Self {
        Self {
            name: String::from("NAME"),
            address: String::new(),
            phone_number: String::from("PHONE NUMBER"),
            notes: String::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    pub fn notes(&self) -> &str {
        &self.notes
    }
}
