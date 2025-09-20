use dioxus::prelude::*;
use dioxus_router::prelude::*;

use components::{Deal, Footer};
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/GENERATED_tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { class: "h-screen w-screen bg-background-primary flex flex-col items-center justify-start overflow-auto",
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }

            div {
                class: "grow",
                Deal {}
            }

            Footer {}
        }
    }
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    App {}
}

#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .into_iter()
        .map(|r| r.to_string())
        .collect())
}
