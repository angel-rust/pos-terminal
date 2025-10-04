use dioxus::prelude::*;
use crate::models::Order;

#[component]
pub fn Cart(
    order: Signal<Order>,
    on_checkout: EventHandler<()>,
    on_clear: EventHandler<()>,
) -> Element {
    let current_order = order.read();
    let is_empty = current_order.items.is_empty();

    rsx! {
        div {
            class: "cart-container",

            div {
                class: "cart-header",
                h2 { "Current Order" }
                if !is_empty {
                    button {
                        class: "clear-btn",
                        onclick: move |_| on_clear.call(()),
                        "Clear"
                    }
                }
            }

            div {
                class: "cart-items",
                if is_empty {
                    div {
                        class: "empty-cart",
                        p { "🛒 Cart is empty" }
                        p { class: "hint", "Add items to get started" }
                    }
                } else {
                    for item in current_order.items.iter() {
                        {
                            let product_id = item.product_id.clone();
                            let product_name = item.product_name.clone();
                            let price = item.price;
                            let quantity = item.quantity;

                            rsx! {
                                CartItem {
                                    key: "{product_id}",
                                    product_id: product_id.clone(),
                                    product_name: product_name,
                                    price: price,
                                    quantity: quantity,
                                    on_add: move |id: String| {
                                        let mut o = order.write();
                                        if let Some(item) = o.items.iter_mut().find(|i| i.product_id == id) {
                                            item.quantity += 1;
                                            o.calculate_totals();
                                        }
                                    },
                                    on_remove: move |id: String| {
                                        order.write().remove_item(&id);
                                    },
                                }
                            }
                        }
                    }
                }
            }

            if !is_empty {
                div {
                    class: "cart-summary",

                    div {
                        class: "summary-row",
                        span { "Subtotal:" }
                        span { "${current_order.subtotal:.2}" }
                    }
                    div {
                        class: "summary-row",
                        span { "Tax (8%):" }
                        span { "${current_order.tax:.2}" }
                    }
                    div {
                        class: "summary-row total",
                        span { "Total:" }
                        span { "${current_order.total:.2}" }
                    }

                    button {
                        class: "checkout-btn",
                        onclick: move |_| on_checkout.call(()),
                        "Checkout"
                    }
                }
            }
        }
    }
}

#[component]
fn CartItem(
    product_id: String,
    product_name: String,
    price: f64,
    quantity: u32,
    on_add: EventHandler<String>,
    on_remove: EventHandler<String>,
) -> Element {
    let id = product_id.clone();
    let id2 = product_id.clone();

    rsx! {
        div {
            class: "cart-item",

            div {
                class: "item-info",
                div { class: "item-name", "{product_name}" }
                div { class: "item-price", "${price:.2}" }
            }

            div {
                class: "item-controls",
                button {
                    class: "qty-btn",
                    onclick: move |_| on_remove.call(id.clone()),
                    "−"
                }
                span { class: "qty", "{quantity}" }
                button {
                    class: "qty-btn",
                    onclick: move |_| on_add.call(id2.clone()),
                    "+"
                }
            }

            div {
                class: "item-total",
                "${price * quantity as f64:.2}"
            }
        }
    }
}
