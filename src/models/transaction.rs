#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Transaction {
    pub cc_name: Option<String>,
    pub cc_number: Option<String>,
    pub cc_exp_month: Option<String>,
    pub cc_exp_year: Option<String>,
    pub cc_cvv: Option<String>,

    pub ach_name: Option<String>,
    pub ach_routing_number: Option<String>,
    pub ach_number: Option<String>,
    pub ach_bank_type: Option<String>,

    // address
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,

    pub signature: Option<String>,

    // honeypot
    pub payment_id: Option<String>,
}