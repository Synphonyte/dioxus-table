#![allow(non_snake_case)]

mod hotel;

use dioxus_table::{TableHeadEvent, TableRowEvent};
use hotel::{Hotel, Table as HotelTable};
use web_sys;

use dioxus::prelude::*;
use reqwest;

fn main() {
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    let request = use_future(&cx, (), |_| async move {
        reqwest::get("http://localhost:8080/test-data.json") // TODO : relative uris don't work. maybe we don't need it either.
            .await
            .expect("To workr")
            .json::<Vec<Hotel>>()
            .await
    });

    cx.render(match request.value() {
        Some(Ok(items)) => rsx! {
            HotelTable {
                class: "table",
                items: items,
                onrowclick: move |evt: TableRowEvent<_, _>| {
                    web_sys::console::log_1(&format!("Row {}", evt.row_index).into());
                },
                onheadclick: move |evt: TableHeadEvent<_>| {
                    web_sys::console::log_1(&format!("Head {} '{}'", evt.column_index, evt.field).into());
                },
            }
        },
        Some(Err(e)) => rsx! {
            div {
                h1 { "Error"}
                p { "{e}" }
            }
        },
        _ => rsx! {
             "Loading..."
        },
    })
}
