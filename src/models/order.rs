use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::payment::Payment;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Order {
    pub id: String,
    pub items: Vec<OrderItem>,
    pub subtotal: f64,
    pub tax: f64,
    pub total: f64,
    pub payment: Option<Payment>,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderItem {
    pub product_id: String,
    pub product_name: String,
    pub price: f64,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Pending,
    Paid,
    Refunded,
    Cancelled,
}

impl Order {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            items: Vec::new(),
            subtotal: 0.0,
            tax: 0.0,
            total: 0.0,
            payment: None,
            status: OrderStatus::Pending,
            created_at: Utc::now(),
        }
    }

    pub fn add_item(&mut self, product_id: String, product_name: String, price: f64) {
        if let Some(item) = self.items.iter_mut().find(|i| i.product_id == product_id) {
            item.quantity += 1;
        } else {
            self.items.push(OrderItem {
                product_id,
                product_name,
                price,
                quantity: 1,
            });
        }
        self.calculate_totals();
    }

    pub fn remove_item(&mut self, product_id: &str) {
        if let Some(pos) = self.items.iter().position(|i| i.product_id == product_id) {
            let item = &mut self.items[pos];
            if item.quantity > 1 {
                item.quantity -= 1;
            } else {
                self.items.remove(pos);
            }
        }
        self.calculate_totals();
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.calculate_totals();
    }

    pub fn calculate_totals(&mut self) {
        self.subtotal = self.items.iter().map(|i| i.price * i.quantity as f64).sum();
        self.tax = self.subtotal * 0.08; // 8% tax
        self.total = self.subtotal + self.tax;
    }

    pub fn complete_payment(&mut self, payment: Payment) {
        self.payment = Some(payment);
        self.status = OrderStatus::Paid;
    }
}

impl OrderStatus {
    pub fn as_str(&self) -> &str {
        match self {
            OrderStatus::Pending => "Pending",
            OrderStatus::Paid => "Paid",
            OrderStatus::Refunded => "Refunded",
            OrderStatus::Cancelled => "Cancelled",
        }
    }
}
