#![allow(non_snake_case)]

mod hotel;

use dioxus_table::{TableHeadEvent, TableRowEvent};
use hotel::{Hotel, Table as HotelTable, SELECTED};
use web_sys;

use dioxus::prelude::*;
use gloo_net::http::Request;

fn main() {
    dioxus::web::launch(App);
}


fn App(cx: Scope) -> Element {
    let request = use_future(&cx, (), |_| async move {
        Request::get("test-data.json")
            .send()
            .await
            .expect("To work")
            .json::<Vec<Hotel>>()
            .await
    });

    let set_selected = use_set(&cx, SELECTED);

    cx.render(match request.value() {
        Some(Ok(items)) => rsx! {
            HotelTable {
                class: "table",
                items: items,
                onrowclick: move |evt: TableRowEvent<_, _>| {
                    set_selected(Some(evt.row_index));
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
