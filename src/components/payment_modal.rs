use dioxus::prelude::*;
use crate::models::{Order, Payment, PaymentMethod};

#[component]
pub fn PaymentModal(
    order: Signal<Order>,
    show: Signal<bool>,
    on_complete: EventHandler<Payment>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut selected_method = use_signal(|| PaymentMethod::Cash);
    let current_order = order.read().clone();

    if !show() {
        return rsx! { div {} };
    }

    rsx! {
        div {
            class: "modal-overlay",
            onclick: move |_| on_cancel.call(()),

            div {
                class: "modal-content payment-modal",
                onclick: move |e| e.stop_propagation(),

                div {
                    class: "modal-header",
                    h2 { "Payment" }
                    button {
                        class: "close-btn",
                        onclick: move |_| on_cancel.call(()),
                        "✕"
                    }
                }

                div {
                    class: "modal-body",

                    div {
                        class: "payment-summary",
                        h3 { "Order Summary" }
                        div {
                            class: "summary-items",
                            for item in current_order.items.iter() {
                                div {
                                    class: "summary-item",
                                    span { "{item.product_name} × {item.quantity}" }
                                    span { "${item.price * item.quantity as f64:.2}" }
                                }
                            }
                        }
                        div {
                            class: "summary-total",
                            span { "Total:" }
                            span { class: "total-amount", "${current_order.total:.2}" }
                        }
                    }

                    div {
                        class: "payment-methods",
                        h3 { "Select Payment Method" }

                        div {
                            class: "methods-grid",
                            PaymentMethodButton {
                                method: PaymentMethod::Cash,
                                selected: selected_method() == PaymentMethod::Cash,
                                on_select: move |m| selected_method.set(m),
                            }
                            PaymentMethodButton {
                                method: PaymentMethod::CreditCard,
                                selected: selected_method() == PaymentMethod::CreditCard,
                                on_select: move |m| selected_method.set(m),
                            }
                            PaymentMethodButton {
                                method: PaymentMethod::DebitCard,
                                selected: selected_method() == PaymentMethod::DebitCard,
                                on_select: move |m| selected_method.set(m),
                            }
                            PaymentMethodButton {
                                method: PaymentMethod::MobilePay,
                                selected: selected_method() == PaymentMethod::MobilePay,
                                on_select: move |m| selected_method.set(m),
                            }
                        }
                    }
                }

                div {
                    class: "modal-footer",
                    button {
                        class: "btn-secondary",
                        onclick: move |_| on_cancel.call(()),
                        "Cancel"
                    }
                    button {
                        class: "btn-primary",
                        onclick: move |_| {
                            let payment = Payment::new(selected_method(), current_order.total);
                            on_complete.call(payment);
                        },
                        "Complete Payment"
                    }
                }
            }
        }
    }
}

#[component]
fn PaymentMethodButton(
    method: PaymentMethod,
    selected: bool,
    on_select: EventHandler<PaymentMethod>,
) -> Element {
    let method_clone = method.clone();

    rsx! {
        button {
            class: if selected { "payment-method-btn selected" } else { "payment-method-btn" },
            onclick: move |_| on_select.call(method_clone.clone()),

            div {
                class: "method-icon",
                "{method.icon()}"
            }
            div {
                class: "method-name",
                "{method.as_str()}"
            }
        }
    }
}
