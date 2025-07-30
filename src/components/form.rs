use base64::{prelude::BASE64_STANDARD, Engine};
use dioxus::prelude::*;

#[component]
pub fn Form() -> Element {
    let mut client_id = use_signal(|| String::new());
    let mut client_secret = use_signal(|| String::new());
    let token = use_memo(move || BASE64_STANDARD.encode(format!("{client_id}:{client_secret}")));

    rsx! {
        main { class: "flex flex-col items-center gap-6 p-7 md:flex-row md:gap-8",

            h1 { "OAuth 2 Token Generator" }
            p {
                "All data is processed locally in your browser. If you "
                "are technical, you can "
                a { href: "https://github.com/lilopkins/oauth-token-generator",

                    "view the source code"
                }
                "."
            }

            hr {}

            label { r#for: "clientID", "Client ID" }
            input {
                id: "clientID",
                value: "{client_id}",
                oninput: move |event| client_id.set(event.value()),
            }

            label { r#for: "clientSecret", "Client Secret" }
            input {
                id: "clientSecret",
                value: "{client_secret}",
                oninput: move |event| client_secret.set(event.value()),
            }

            label { r#for: "token", "Token" }
            input { id: "token", readonly: true, value: "{token}" }
        }
    }
}
