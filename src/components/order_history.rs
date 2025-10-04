use dioxus::prelude::*;
use crate::models::Order;

#[component]
pub fn OrderHistory(orders: Signal<Vec<Order>>) -> Element {
    let all_orders = orders.read();

    rsx! {
        div {
            class: "order-history-container",

            div {
                class: "history-header",
                h2 { "Order History" }
                div {
                    class: "history-stats",
                    div {
                        class: "stat",
                        span { class: "stat-label", "Total Orders:" }
                        span { class: "stat-value", "{all_orders.len()}" }
                    }
                    div {
                        class: "stat",
                        span { class: "stat-label", "Total Revenue:" }
                        span { class: "stat-value",
                            "${all_orders.iter().map(|o| o.total).sum::<f64>():.2}"
                        }
                    }
                }
            }

            div {
                class: "history-list",
                if all_orders.is_empty() {
                    div {
                        class: "empty-history",
                        p { "📋 No orders yet" }
                    }
                }
                for (idx, order) in all_orders.iter().enumerate().rev() {
                    OrderHistoryItem {
                        key: "{idx}",
                        order_id: order.id.clone(),
                        created_at: format!("{}", order.created_at.format("%b %d, %Y %I:%M %p")),
                        items_count: order.items.len(),
                        total: order.total,
                        status: order.status.as_str().to_string(),
                        items_display: order.items.iter().map(|item| {
                            format!("{} × {}: ${:.2}", item.product_name, item.quantity, item.price * item.quantity as f64)
                        }).collect::<Vec<_>>(),
                        subtotal: order.subtotal,
                        tax: order.tax,
                        payment_info: order.payment.as_ref().map(|p| {
                            format!("{} {} - Processed: {}", p.method.icon(), p.method.as_str(), p.processed_at.format("%I:%M %p"))
                        }),
                    }
                }
            }
        }
    }
}

#[component]
fn OrderHistoryItem(
    order_id: String,
    created_at: String,
    items_count: usize,
    total: f64,
    status: String,
    items_display: Vec<String>,
    subtotal: f64,
    tax: f64,
    payment_info: Option<String>,
) -> Element {
    let mut expanded = use_signal(|| false);
    let status_class = format!("order-status status-{}", status.to_lowercase());

    rsx! {
        div {
            class: "history-item",

            div {
                class: "history-item-header",
                onclick: move |_| expanded.set(!expanded()),

                div {
                    class: "order-info",
                    div { class: "order-id", "Order #{&order_id[..8]}" }
                    div { class: "order-time", "{created_at}" }
                }

                div {
                    class: "order-summary",
                    span { class: "order-items", "{items_count} items" }
                    span { class: "order-total", "${total:.2}" }
                    span {
                        class: "{status_class}",
                        "{status}"
                    }
                }

                span {
                    class: "expand-icon",
                    {if expanded() { "▼" } else { "▶" }}
                }
            }

            {expanded().then(|| rsx! {
                div {
                    class: "history-item-details",

                    div {
                        class: "order-items-list",
                        for item_str in items_display.iter() {
                            div {
                                class: "detail-item",
                                span { "{item_str}" }
                            }
                        }
                    }

                    div {
                        class: "order-totals",
                        div {
                            class: "total-row",
                            span { "Subtotal:" }
                            span { "${subtotal:.2}" }
                        }
                        div {
                            class: "total-row",
                            span { "Tax:" }
                            span { "${tax:.2}" }
                        }
                        div {
                            class: "total-row total",
                            span { "Total:" }
                            span { "${total:.2}" }
                        }
                    }

                    {payment_info.as_ref().map(|info| rsx! {
                        div {
                            class: "payment-info",
                            span { "{info}" }
                        }
                    })}
                }
            })}
        }
    }
}
