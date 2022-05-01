use dioxus::prelude::*;
use dioxus_table::*;
use serde::{Deserialize, Serialize};

pub static SELECTED: Atom<Option<usize>> = |_| None;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, TableData)]
#[table(tag = "table", row_class = "row-class", dyn_row_classes)]
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

impl Hotel {
    fn row_classes<T>(&self, index: usize, cx: Scope<T>) -> Vec<String> {
        let selected_index = use_read(&cx, SELECTED);

        if let Some(selected_index) = *selected_index {
            if index == selected_index {
                return vec!["bg-sky-200".to_owned()];
            }
        }

        vec![]
    }
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
