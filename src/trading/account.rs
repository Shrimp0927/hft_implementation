use serde;

#[derive(Debug)]
pub enum AccountStatus {
    Active,
    Onboarding,
    SubmissionFailed,
    Submitted,
    AccountUpdated,
    ApprovalPending,
    Rejected,
    Other,
}

impl<'de> serde::Deserialize<'de> for AccountStatus {
    fn deserialize<D: serde::Deserializer<'de>>(status_string: D) -> Result<Self, D::Error> {
        let status_string = String::deserialize(status_string)?;
        match status_string.as_str() {
            "ONBOARDING" => Ok(AccountStatus::Onboarding),
            "SUBMISSION_FAILED" => Ok(AccountStatus::SubmissionFailed),
            "SUBMITTED" => Ok(AccountStatus::Submitted),
            "ACCOUNT_UPDATED" => Ok(AccountStatus::AccountUpdated),
            "APPROVAL_PENDING" => Ok(AccountStatus::ApprovalPending),
            "ACTIVE" => Ok(AccountStatus::Active),
            "REJECTED" => Ok(AccountStatus::Rejected),
            _other => Ok(AccountStatus::Other),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Account {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "account_number")]
    pub account_number: AccountStatus,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "crypto_status")]
    pub crypto_status: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "cash")]
    pub cash: String,
    #[serde(rename = "non_marginable_buying_power")]
    pub non_marginable_buying_power: String,
    #[serde(rename = "accrued_fees")]
    pub accrued_fees: String,
    #[serde(rename = "pending_transfer_in")]
    pub pending_transfer_in: String,
    #[serde(rename = "pending_transfer_out")]
    pub pending_transfer_out: String,
    #[serde(rename = "pattern_day_trader")]
    pub pattern_day_trader: bool,
    #[serde(rename = "trade_suspended_by_user")]
    pub trade_suspended_by_user: bool,
    #[serde(rename = "trading_blocked")]
    pub trading_blocked: bool,
    #[serde(rename = "transfers_blocked")]
    pub transfers_blocked: bool,
    #[serde(rename = "account_blocked")]
    pub account_blocked: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "shorting_enabled")]
    pub shorting_enabled: bool,
    #[serde(rename = "long_market_value")]
    pub long_market_value: String,
    #[serde(rename = "short_market_value")]
    pub short_market_value: String,
    #[serde(rename = "equity")]
    pub equity: String,
    #[serde(rename = "last_equity")]
    pub last_equity: String,
    #[serde(rename = "multiplier")]
    pub multiplier: String,
    #[serde(rename = "buying_power")]
    pub buying_power: String,
    #[serde(rename = "initial_margin")]
    pub initial_margin: String,
    #[serde(rename = "maintenance_margin")]
    pub maintenance_margin: String,
    #[serde(rename = "sma")]
    pub sma: String,
    #[serde(rename = "daytrade_count")]
    pub daytrade_count: i32,
    #[serde(rename = "last_maintenance_margin")]
    pub last_maintenance_margin: String,
    #[serde(rename = "daytrading_buying_power")]
    pub daytrading_buying_power: String,
    #[serde(rename = "regt_buying_power")]
    pub regt_buying_power: String,
    #[serde(default)]
    #[serde(flatten)]
    extra: serde_json::Value,
}
