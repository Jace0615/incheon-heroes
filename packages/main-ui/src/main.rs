pub mod assets;
pub mod components;
pub mod config;
pub mod pages;
pub mod route;

use crate::route::Route;
use by_components::theme::{ColorTheme, TextColorTheme};
use dioxus::prelude::*;
use dioxus_popup::PopupService;

fn main() {
    let conf = config::get();
    dioxus_logger::init(conf.log_level).expect("failed to init logger");
    tracing::debug!("config: {:?}", conf);

    dioxus_aws::launch(app);
}

fn setup_color_theme() {
    use_context_provider(|| ColorTheme {
        background: "white".to_string(),
        text: TextColorTheme {
            primary: "black".to_string(),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn app() -> Element {
    setup_color_theme();
    PopupService::init();

    rsx! {
        document::Title { "Incheon Heroes" }
        document::Meta { name: "description", content: "" }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0",
        }

        document::Link { href: "https://fonts.googleapis.com", rel: "preconnect" }
        document::Link {
            crossorigin: "false",
            href: "https://fonts.gstatic.com",
            rel: "preconnect",
        }
        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@100..900&family=Russo+One&display=swap",
            rel: "stylesheet",
        }
        document::Link {
            href: "https://cdn.jsdelivr.net/gh/fonts-archive/Pretendard/Pretendard.css",
            r#type: "text/css",
            rel: "stylesheet",
        }

        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@100..900&family=Noto+Color+Emoji&family=Russo+One&display=swap",
            rel: "stylesheet",
        }
        document::Link { id: "favicon", rel: "icon", href: "{assets::FAVICON}" }
        document::Link { rel: "stylesheet", href: asset!("/public/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/public/tailwind.css") }

        document::Script { src: "https://cdn.tailwindcss.com/3.4.16" }

        Router::<Route> {}
    }
}

#[cfg(feature = "server")]
mod api {
    use dioxus::fullstack::prelude::*;
    use server_fn::codec::{GetUrl, Json};

    #[server(endpoint = "/version", input=GetUrl, output=Json)]
    pub async fn version() -> Result<String, ServerFnError> {
        Ok(match option_env!("VERSION") {
            Some(version) => match option_env!("COMMIT") {
                Some(commit) => format!("{}-{}", version, commit),
                None => format!("{}", version),
            },
            None => match option_env!("DATE") {
                Some(date) => date.to_string(),
                None => "unknown".to_string(),
            },
        }
        .to_string())
    }
}
