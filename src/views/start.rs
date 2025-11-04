use dioxus::prelude::*;


#[component]
pub fn Start() -> Element {
    rsx! {
        div { id: "start",
            h1 { "Brooker Iteration 1" }
            p { "Here you can check on stocks" }
            p { "Enter a stock symbol to check on it" }
        }
    }
}