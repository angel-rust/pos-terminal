mod models;
mod components;

use dioxus::prelude::*;
use models::{Product, ProductCategory, Order, Payment};
use components::{ProductGrid, Cart, PaymentModal, OrderHistory, ProductManager};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut products = use_signal(|| vec![
        Product::new("Espresso".to_string(), 3.50, ProductCategory::Beverage),
        Product::new("Cappuccino".to_string(), 4.50, ProductCategory::Beverage),
        Product::new("Latte".to_string(), 5.00, ProductCategory::Beverage),
        Product::new("Croissant".to_string(), 3.75, ProductCategory::Food),
        Product::new("Bagel".to_string(), 2.50, ProductCategory::Food),
        Product::new("Sandwich".to_string(), 8.50, ProductCategory::Food),
        Product::new("Salad".to_string(), 9.75, ProductCategory::Food),
        Product::new("Muffin".to_string(), 3.25, ProductCategory::Food),
        Product::new("T-Shirt".to_string(), 24.99, ProductCategory::Retail),
        Product::new("Mug".to_string(), 12.99, ProductCategory::Retail),
        Product::new("Consultation".to_string(), 50.00, ProductCategory::Service),
    ]);

    let mut current_order = use_signal(Order::new);
    let mut order_history = use_signal(|| Vec::<Order>::new());
    let selected_category = use_signal(|| None);
    let mut show_payment_modal = use_signal(|| false);
    let mut current_view = use_signal(|| View::POS);
    let mut dark_mode = use_signal(|| true);
    let mut sidebar_open = use_signal(|| false);

    rsx! {
        style { {include_str!("../styles.css")} }

        div {
            class: if dark_mode() { "app-container dark-mode" } else { "app-container" },

            // Header
            header {
                class: "app-header",
                div {
                    class: "header-brand",
                    h1 { "TREZZA TERMINAL" }
                }

                button {
                    class: "menu-btn",
                    onclick: move |_| sidebar_open.set(!sidebar_open()),
                    "MENU"
                }
            }

            // Sidebar
            div {
                class: if sidebar_open() { "sidebar open" } else { "sidebar" },

                div {
                    class: "sidebar-header",
                    h2 { "Menu" }
                    button {
                        class: "close-sidebar-btn",
                        onclick: move |_| sidebar_open.set(false),
                        "CLOSE"
                    }
                }

                nav {
                    class: "sidebar-nav",
                    button {
                        class: if current_view() == View::POS { "sidebar-btn active" } else { "sidebar-btn" },
                        onclick: move |_| {
                            current_view.set(View::POS);
                            sidebar_open.set(false);
                        },
                        "POS"
                    }
                    button {
                        class: if current_view() == View::History { "sidebar-btn active" } else { "sidebar-btn" },
                        onclick: move |_| {
                            current_view.set(View::History);
                            sidebar_open.set(false);
                        },
                        "History"
                    }
                    button {
                        class: if current_view() == View::Products { "sidebar-btn active" } else { "sidebar-btn" },
                        onclick: move |_| {
                            current_view.set(View::Products);
                            sidebar_open.set(false);
                        },
                        "Products"
                    }

                    div { class: "sidebar-divider" }

                    button {
                        class: "sidebar-btn",
                        onclick: move |_| dark_mode.set(!dark_mode()),
                        if dark_mode() { "Light Mode" } else { "Dark Mode" }
                    }
                }
            }

            // Sidebar overlay
            if sidebar_open() {
                div {
                    class: "sidebar-overlay",
                    onclick: move |_| sidebar_open.set(false),
                }
            }

            // Main Content
            main {
                class: "main-content",

                match current_view() {
                    View::POS => rsx! {
                        div {
                            class: "pos-view",

                            div {
                                class: "products-section",
                                ProductGrid {
                                    products,
                                    selected_category,
                                    on_product_click: move |product: Product| {
                                        current_order.write().add_item(
                                            product.id.clone(),
                                            product.name.clone(),
                                            product.price,
                                        );
                                    },
                                }
                            }

                            div {
                                class: "cart-section",
                                Cart {
                                    order: current_order,
                                    on_checkout: move |_| {
                                        show_payment_modal.set(true);
                                    },
                                    on_clear: move |_| {
                                        current_order.write().clear();
                                    },
                                }
                            }
                        }
                    },
                    View::History => rsx! {
                        OrderHistory {
                            orders: order_history,
                        }
                    },
                    View::Products => rsx! {
                        ProductManager {
                            products,
                            on_add: move |product| {
                                products.write().push(product);
                            },
                            on_delete: move |id| {
                                products.write().retain(|p| p.id != id);
                            },
                        }
                    },
                }
            }

            // Payment Modal
            PaymentModal {
                order: current_order,
                show: show_payment_modal,
                on_complete: move |payment: Payment| {
                    let mut order = current_order.write();
                    order.complete_payment(payment);
                    order_history.write().push(order.clone());
                    *order = Order::new();
                    show_payment_modal.set(false);
                },
                on_cancel: move |_| {
                    show_payment_modal.set(false);
                },
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum View {
    POS,
    History,
    Products,
}
