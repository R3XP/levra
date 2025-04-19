use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Counter() -> Element {
    //let num = use_server_future(future);

    rsx! {
        h1 { class: "bg-teal-500 ", "Hello, this is the counter test!" }

        div { class: "bg",
            "lol"
        }
    }
}

//#[server]
//async fn counter_server(input: String) -> Result<String, ServerFnError> {
//    use crate::dbmagic::{Count, DB};
//    // The body of server function like this comment are only included on the server. If you have any server-only logic like
//    // database queries, you can put it here. Any imports for the server function should either be imported inside the function
//    // or imported under a `#[cfg(feature = "server")]` block.
//
//    let counter: Option<Count> = DB.select(("counter", "c")).await?.ok_or(Err);
//    Ok(input)
//}
