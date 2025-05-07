use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Counter() -> Element {
    let num_res = use_resource(counter_server)?;
    //let num = use_server_future(counter_server)?.unwrap().unwrap();

    //let num = num_res

    //let num = 45;

    rsx! {
        h1 { class: "bg-teal-500", "Hello, this is the counter test!" }

        main { class: "grid place-items-center",
            div { class: "flex flex-col gap-4 p-6 border-2 border-slate-400/20" ,
                p { class: "text-2xl font-bold mb-2", "{num}" }

                button { class: "rounded-lg p-6 py-2 font-bold border-[1px]", "+1" }
                button { class: "rounded-lg p-6 py-2 font-bold border-[1px]", "-1" }
            }
        }
    }
}

#[server]
async fn counter_server() -> Result<i32, ServerFnError> {
    use crate::dbmagic::{Count, DB};
    // The body of server function like this comment are only included on the server. If you have any server-only logic like
    // database queries, you can put it here. Any imports for the server function should either be imported inside the function
    // or imported under a `#[cfg(feature = "server")]` block.

    //let counter: Option<Count> = DB.select(("counter", "c")).await?.ok_or(Err);
    Ok(42)
}

#[server]
async fn update_counter(delta: i32) -> Result<i32, ServerFnError> {
    Ok(42 + delta)
}
