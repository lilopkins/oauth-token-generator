use base64::{prelude::BASE64_STANDARD, Engine};
use dioxus::prelude::*;
use dioxus_primitives::label::Label;

#[component]
pub fn Form() -> Element {
    let mut client_id = use_signal(|| String::new());
    let mut client_secret = use_signal(|| String::new());
    let token = use_memo(move || BASE64_STANDARD.encode(format!("{client_id}:{client_secret}")));

    rsx! {
        main { display: "flex", flex_direction: "column", gap: "1rem",
            div {
                h1 { "OAuth 2 Token Generator" }
                p {
                    "All data is processed locally in your browser. If you "
                    "are technical, you can "
                    a { href: "https://github.com/lilopkins/oauth-token-generator",

                        "view the source code"
                    }
                    "."
                }
            }

            div { display: "flex", flex_direction: "column", gap: ".5rem",
                Label { class: "label", html_for: "clientID", "Client ID" }

                input {
                    class: "input",
                    id: "clientID",
                    placeholder: "Enter your client ID",
                    value: "{client_id}",
                    oninput: move |event| client_id.set(event.value()),
                }
            }

            div { display: "flex", flex_direction: "column", gap: ".5rem",
                Label { class: "label", html_for: "clientSecret", "Client Secret" }

                input {
                    class: "input",
                    id: "clientSecret",
                    placeholder: "Enter your client secret",
                    value: "{client_secret}",
                    oninput: move |event| client_secret.set(event.value()),
                }
            }

            div { display: "flex", flex_direction: "column", gap: ".5rem",
                Label { class: "label", html_for: "token", "Token" }

                input {
                    class: "input",
                    id: "token",
                    readonly: true,
                    value: "{token}",
                }
            }
        }
    }
}
