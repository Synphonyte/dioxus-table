use dioxus::prelude::*;
use dioxus_table::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, TableData)]
#[table(tag = "table", row_class = "row-class")]
pub struct Hotel {
    pub id: i32,

    #[table(class = "text-end", title = "The Awesome Name")]
    pub name: String,

    #[table(renderer = "StarRenderer", class = "text-center")]
    pub rating: i32,

    pub city: String,

    #[table(skip)]
    pub internal: u32,
}

pub fn StarRenderer(cx: Scope<DefaultTableCellProps<i32>>) -> Element {
    let count = cx.props.value as usize;

    let mut stars = "".to_owned();
    for _ in 0..count {
        stars += "★";
    }
    for _ in count..5 {
        stars += "☆";
    }

    cx.render(rsx! {
        td {
            class: "{cx.props.class}",
            "{stars}"
        }
    })
}
