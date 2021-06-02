#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub centers: Vec<Center>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Center {
    #[serde(rename = "center_id")]
    pub center_id: u64,
    pub name: String,
    pub address: String,
    #[serde(rename = "state_name")]
    pub state_name: String,
    #[serde(rename = "district_name")]
    pub district_name: String,
    #[serde(rename = "block_name")]
    pub block_name: String,
    pub pincode: u64,
    pub lat: i64,
    pub long: i64,
    pub from: String,
    pub to: String,
    #[serde(rename = "fee_type")]
    pub fee_type: String,
    pub sessions: Vec<Session>,
    #[serde(rename = "vaccine_fees")]
    #[serde(default)]
    pub vaccine_fees: Vec<VaccineFee>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    #[serde(rename = "session_id")]
    pub session_id: String,
    pub date: String,
    #[serde(rename = "available_capacity")]
    pub available_capacity: f64,
    #[serde(rename = "min_age_limit")]
    pub min_age_limit: u64,
    pub vaccine: String,
    pub slots: Vec<String>,
    #[serde(rename = "available_capacity_dose1")]
    pub available_capacity_dose1: i64,
    #[serde(rename = "available_capacity_dose2")]
    pub available_capacity_dose2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaccineFee {
    pub vaccine: String,
    pub fee: String,
}

pub const USER_AGENT: &str =
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:15.0) Gecko/20100101 Firefox/15.0.1";

// List of monitored districts
pub const MONITORED_DISTRICTS: [(u16, &str); 82] = [
    (8, "Visakhapatnam"),
    (49, "Kamrup Metropolitan"),
    (64, "Sonitpur"),
    (74, "Araria"),
    (86, "Muzaffarpur"),
    (108, "Chandigarh"),
    (109, "Raipur"),
    (142, "West Delhi"),
    (145, "East Delhi"),
    (146, "North Delhi"),
    (150, "South West Delhi"),
    (151, "North Goa"),
    (152, "South Goa"),
    (154, "Ahmedabad"),
    (187, "Panchkula"),
    (188, "Gurgaon"),
    (192, "Ambala"),
    (195, "Panipat"),
    (199, "Faridabad"),
    (202, "Rewari"),
    (212, "Sirmaur"),
    (265, "Bangalore Urban"),
    (266, "Mysore"),
    (269, "Dakshina Kannada"),
    (281, "Uttar Kannada"),
    (286, "Udupi"),
    (294, "BBMP"),
    (296, "Thiruvananthapuram"),
    (297, "Kannur"),
    (300, "Pathanamthitta"),
    (301, "Alappuzha"),
    (302, "Malappuram"),
    (303, "Thrissur"),
    (304, "Kottayam"),
    (307, "Ernakulam"),
    (308, "Palakkad"),
    (312, "Bhopal"),
    (313, "Gwalior"),
    (316, "Rewa"),
    (348, "Guna"),
    (362, "Betul"),
    (363, "Pune"),
    (365, "Nagpur"),
    (376, "Satara"),
    (390, "Jalgaon"),
    (391, "Ahmednagar"),
    (392, "Thane"),
    (393, "Raigad"),
    (395, "Mumbai"),
    (397, "Aurangabad"),
    (446, "Khurda"),
    (453, "Sundargarh"),
    (457, "Cuttack"),
    (494, "Patiala"),
    (496, "SAS Nagar"),
    (501, "Bikaner"),
    (502, "Jodhpur"),
    (505, "Jaipur I"),
    (507, "Ajmer"),
    (512, "Alwar"),
    (513, "Sikar"),
    (521, "Chittorgarh"),
    (523, "Bhilwara"),
    (530, "Churu"),
    (571, "Chennai"),
    (581, "Hyderabad"),
    (624, "Prayagraj"),
    (650, "Gautam Buddha Nagar"),
    (651, "Ghaziabad"),
    (664, "Kanpur Nagar"),
    (670, "Lucknow"),
    (676, "Meerut"),
    (679, "Muzaffarnagar"),
    (689, "Shamli"),
    (696, "Varanasi"),
    (697, "Dehradun"),
    (709, "Nainital"),
    (721, "Howrah"),
    (725, "Kolkata"),
    (773, "Jamnagar Corporation"),
    (775, "Rajkot Corporation"),
    (777, "Vadodara Corporation"),
];
