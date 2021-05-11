#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct VaccineFee {
    pub vaccine: String,
    pub fee: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Center {
    pub center_id: u64,
    pub name: String,
    pub address: String,
    pub state_name: String,
    pub district_name: String,
    pub block_name: String,
    pub pincode: u64,
    pub lat: i64,
    pub long: i64,
    pub from: String,
    pub to: String,
    pub fee_type: String,
    pub sessions: Vec<Session>,
    pub vaccine_fees: Option<Vec<VaccineFee>>, // Not all centers provide this field
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Session {
    pub session_id: String,
    pub date: String,
    pub available_capacity: u64,
    pub min_age_limit: u64,
    pub vaccine: String,
    pub slots: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Root {
    pub centers: Vec<Center>,
}