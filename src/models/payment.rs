use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Payment {
    pub method: PaymentMethod,
    pub amount: f64,
    pub processed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentMethod {
    Cash,
    CreditCard,
    DebitCard,
    MobilePay,
}

impl Payment {
    pub fn new(method: PaymentMethod, amount: f64) -> Self {
        Self {
            method,
            amount,
            processed_at: Utc::now(),
        }
    }
}

impl PaymentMethod {
    pub fn as_str(&self) -> &str {
        match self {
            PaymentMethod::Cash => "Cash",
            PaymentMethod::CreditCard => "Credit Card",
            PaymentMethod::DebitCard => "Debit Card",
            PaymentMethod::MobilePay => "Mobile Pay",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            PaymentMethod::Cash => "💵",
            PaymentMethod::CreditCard => "💳",
            PaymentMethod::DebitCard => "💳",
            PaymentMethod::MobilePay => "📱",
        }
    }
}
