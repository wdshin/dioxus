#![allow(non_snake_case)]
use dioxus_core as dioxus;
use dioxus::prelude::*;
use dioxus_web::WebsysRenderer;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(WebsysRenderer::start(Example))
}

static Example: FC<()> = |ctx, _props| {
    let (name, set_name) = use_state(&ctx, || "...?");

    ctx.render(rsx! {
        div { 
            class: "py-12 px-4 text-center w-full max-w-2xl mx-auto"
            span { 
                class: "text-sm font-semibold"
                "Dioxus Example: Jack and Jill"
            }
            h2 { 
                class: "text-5xl mt-2 mb-6 leading-tight font-semibold font-heading"   
                "Hello, {name}"
            }
            button {  
                class: "inline-block py-4 px-8 mr-6 leading-none text-white bg-indigo-600 hover:bg-indigo-900 font-semibold rounded shadow"
                onmouseover: move |_| set_name("jack") 
                "Jack!"
            }
            button {  
                class: "inline-block py-4 px-8 mr-6 leading-none text-white bg-indigo-600 hover:bg-indigo-900 font-semibold rounded shadow"
                onmouseover: move |_| set_name("jill")
                "Jill!"
            }
            button {  
                class: "inline-block py-4 px-8 mr-6 leading-none text-white bg-indigo-600 hover:bg-indigo-900 font-semibold rounded shadow"
                onmouseover: move |e| set_name("....")
                "Reset!"
            }
        }
    })
};