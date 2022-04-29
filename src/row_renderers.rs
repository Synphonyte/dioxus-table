#![allow(non_snake_case)]

use dioxus::events::MouseEvent;
use dioxus::prelude::*;

#[derive(Props)]
pub struct DefaultTableRowProps<'a, T: 'a> {
    #[props(default)]
    pub class: String,

    pub children: Element<'a>,

    #[props(default)]
    pub is_head: bool,

    pub index: usize,

    pub item: Option<&'a T>,

    #[props(default)]
    pub onclick: EventHandler<'a, TableRowEvent<'a, T, MouseEvent>>,
}

pub struct TableRowEvent<'a, T, E> {
    pub row_index: usize,
    pub item: &'a T,
    pub event: E,
}

pub fn DefaultTableRowRenderer<'a, T>(cx: Scope<'a, DefaultTableRowProps<'a, T>>) -> Element<'a> {
    let class = cx.props.class.clone();

    cx.render(rsx! {
        tr {
            class: "{class}",
            onclick: move |evt| {
                cx.props.onclick.call(TableRowEvent{
                    row_index: cx.props.index,
                    item: cx.props.item.expect("Every row should a have an item"),
                    event: evt,
                })
            },
            &cx.props.children
        }
    })
}
