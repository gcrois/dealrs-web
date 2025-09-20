use dioxus::prelude::*;
use chrono::{DateTime, Utc};

#[component]
pub fn Footer() -> Element {
    let git_commit = option_env!("VERGEN_GIT_SHA").unwrap_or("unknown");
    let git_link = format!("https://github.com/gcrois/dealrs-web/commit/{}", git_commit);

    let build_timestamp = option_env!("VERGEN_BUILD_TIMESTAMP").unwrap_or("unknown");
    let build_timestamp = DateTime::parse_from_rfc3339(build_timestamp)
        .map(|dt| dt.with_timezone(&Utc).format("%Y-%m-%d %H:%M UTC").to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    rsx! {
            footer { class: "bg-gray-800 text-white p-2 w-full sticky bottom-0 flex flex-row justify-center",
                div {
                    class: "w-72 text-center",
                    "Built on "
                    a { class: "underline", href: "{git_link}", target: "_blank", "{build_timestamp}" }
                }
                div {
                    class: "w-72 text-center border-l border-r",
                    "Built with "
                    a { class: "underline", href: "https://crates.io/crates/dealrs", target: "_blank", "dealrs" },
                    " and "
                    a { class: "underline", href: "https://dioxuslabs.com/", target: "_blank", "Dioxus" },
                }
                div {
                    class: "w-72 text-center border-gray-600 px-4",
                    a { class: "underline", href: "https://kerenel.itch.io/pixelart-cards", target: "_blank", "Pixel Art Cards" }
                }

            }
        }
}
