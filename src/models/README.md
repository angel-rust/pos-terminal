# Models Module

This module contains the core data models for Trezza Terminal.

## Structure

### Product (`product.rs`)
Represents items available for sale in the system.

**Fields:**
- `id`: Unique identifier (UUID)
- `name`: Product name
- `price`: Price in decimal format
- `category`: Product category (Beverage, Food, Retail, Service)

**Methods:**
- `new(name, price, category)`: Create a new product with generated UUID

### Order (`order.rs`)
Manages shopping cart and order state.

**Fields:**
- `id`: Unique order identifier (UUID)
- `items`: Vector of order items
- `status`: Order status (Pending, Completed, Cancelled)
- `created_at`: Timestamp when order was created
- `payment`: Optional payment information

**Methods:**
- `new()`: Create a new empty order
- `add_item(id, name, price)`: Add item to order or increment quantity
- `remove_item(index)`: Remove item at index
- `update_quantity(index, quantity)`: Update item quantity
- `total()`: Calculate order total
- `complete_payment(payment)`: Mark order as completed with payment
- `clear()`: Reset order to empty state

### Payment (`payment.rs`)
Handles payment processing and methods.

**Payment Methods:**
- Cash
- Card
- Mobile

**Fields:**
- `method`: Payment method used
- `amount`: Payment amount
- `timestamp`: When payment was processed

**Methods:**
- `new(method, amount)`: Create a new payment record

## Usage

```rust
use models::{Product, ProductCategory, Order, Payment, PaymentMethod};

// Create a product
let product = Product::new(
    "Coffee".to_string(),
    3.50,
    ProductCategory::Beverage
);

// Create an order
let mut order = Order::new();
order.add_item(product.id.clone(), product.name.clone(), product.price);

// Process payment
let payment = Payment::new(PaymentMethod::Card, order.total());
order.complete_payment(payment);
```
