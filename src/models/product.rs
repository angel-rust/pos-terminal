use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub category: ProductCategory,
    pub image_url: Option<String>,
    pub stock: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProductCategory {
    Food,
    Beverage,
    Retail,
    Service,
}

impl Product {
    pub fn new(name: String, price: f64, category: ProductCategory) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            price,
            category,
            image_url: None,
            stock: 100,
        }
    }
}

impl ProductCategory {
    pub fn as_str(&self) -> &str {
        match self {
            ProductCategory::Food => "Food",
            ProductCategory::Beverage => "Beverage",
            ProductCategory::Retail => "Retail",
            ProductCategory::Service => "Service",
        }
    }

    pub fn color(&self) -> &str {
        match self {
            ProductCategory::Food => "#FF6B6B",
            ProductCategory::Beverage => "#4ECDC4",
            ProductCategory::Retail => "#95E1D3",
            ProductCategory::Service => "#FFA07A",
        }
    }
}
