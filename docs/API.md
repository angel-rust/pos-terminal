# API Documentation

## Models

### Product

#### Structure
```rust
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub category: ProductCategory,
}
```

#### ProductCategory
```rust
pub enum ProductCategory {
    Beverage,
    Food,
    Retail,
    Service,
}
```

#### Methods

##### `new(name: String, price: f64, category: ProductCategory) -> Self`
Creates a new product with a generated UUID.

**Parameters:**
- `name`: Product name
- `price`: Product price (decimal)
- `category`: Product category

**Returns:** New Product instance

**Example:**
```rust
let product = Product::new(
    "Espresso".to_string(),
    3.50,
    ProductCategory::Beverage
);
```

---

### Order

#### Structure
```rust
pub struct Order {
    pub id: String,
    pub items: Vec<OrderItem>,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub payment: Option<Payment>,
}
```

#### OrderItem
```rust
pub struct OrderItem {
    pub product_id: String,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}
```

#### OrderStatus
```rust
pub enum OrderStatus {
    Pending,
    Completed,
    Cancelled,
}
```

#### Methods

##### `new() -> Self`
Creates a new empty order with Pending status.

**Returns:** New Order instance

##### `add_item(product_id: String, name: String, price: f64)`
Adds an item to the order or increments quantity if already exists.

**Parameters:**
- `product_id`: Unique product identifier
- `name`: Product name
- `price`: Product price

##### `remove_item(index: usize)`
Removes an item at the specified index.

**Parameters:**
- `index`: Index of item to remove

##### `update_quantity(index: usize, quantity: u32)`
Updates the quantity of an item.

**Parameters:**
- `index`: Index of item to update
- `quantity`: New quantity (removes if 0)

##### `total() -> f64`
Calculates the total cost of all items in the order.

**Returns:** Total price as f64

##### `complete_payment(payment: Payment)`
Marks the order as completed with payment information.

**Parameters:**
- `payment`: Payment details

##### `clear()`
Removes all items and resets the order.

**Example:**
```rust
let mut order = Order::new();
order.add_item(
    "uuid-123".to_string(),
    "Coffee".to_string(),
    3.50
);
let total = order.total();
```

---

### Payment

#### Structure
```rust
pub struct Payment {
    pub method: PaymentMethod,
    pub amount: f64,
    pub timestamp: DateTime<Utc>,
}
```

#### PaymentMethod
```rust
pub enum PaymentMethod {
    Cash,
    Card,
    Mobile,
}
```

#### Methods

##### `new(method: PaymentMethod, amount: f64) -> Self`
Creates a new payment record with current timestamp.

**Parameters:**
- `method`: Payment method used
- `amount`: Payment amount

**Returns:** New Payment instance

**Example:**
```rust
let payment = Payment::new(
    PaymentMethod::Card,
    25.50
);
```

---

## Components

### ProductGrid

#### Props
```rust
#[component]
fn ProductGrid(
    products: Signal<Vec<Product>>,
    selected_category: Signal<Option<ProductCategory>>,
    on_product_click: EventHandler<Product>
) -> Element
```

**Parameters:**
- `products`: Signal containing product list
- `selected_category`: Signal for category filter
- `on_product_click`: Callback when product is clicked

---

### Cart

#### Props
```rust
#[component]
fn Cart(
    order: Signal<Order>,
    on_checkout: EventHandler<()>,
    on_clear: EventHandler<()>
) -> Element
```

**Parameters:**
- `order`: Signal containing current order
- `on_checkout`: Callback for checkout action
- `on_clear`: Callback for clear cart action

---

### PaymentModal

#### Props
```rust
#[component]
fn PaymentModal(
    order: Signal<Order>,
    show: Signal<bool>,
    on_complete: EventHandler<Payment>,
    on_cancel: EventHandler<()>
) -> Element
```

**Parameters:**
- `order`: Signal containing order to pay
- `show`: Signal controlling visibility
- `on_complete`: Callback when payment completes
- `on_cancel`: Callback when payment is cancelled

---

### OrderHistory

#### Props
```rust
#[component]
fn OrderHistory(
    orders: Signal<Vec<Order>>
) -> Element
```

**Parameters:**
- `orders`: Signal containing completed orders list

---

### ProductManager

#### Props
```rust
#[component]
fn ProductManager(
    products: Signal<Vec<Product>>,
    on_add: EventHandler<Product>,
    on_delete: EventHandler<String>
) -> Element
```

**Parameters:**
- `products`: Signal containing product list
- `on_add`: Callback when product is added
- `on_delete`: Callback when product is deleted (receives product ID)

---

## State Management

### Signals
All state in Trezza Terminal is managed through Dioxus signals:

```rust
let mut products = use_signal(|| vec![]);
let mut current_order = use_signal(Order::new);
let mut dark_mode = use_signal(|| true);
```

### Reading Signal Values
```rust
let value = signal();
```

### Writing Signal Values
```rust
signal.set(new_value);
signal.write().method();
```

### Example: Adding to Cart
```rust
on_product_click: move |product: Product| {
    current_order.write().add_item(
        product.id.clone(),
        product.name.clone(),
        product.price,
    );
}
```
