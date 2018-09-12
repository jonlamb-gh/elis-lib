// TODOs
// - proper phone number type

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
    name: String,
    phone_number: String,
}
