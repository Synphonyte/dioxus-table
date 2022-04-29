#![allow(non_snake_case)]

use dioxus::events::MouseEvent;
use dioxus::prelude::*;
use std::fmt::Display;

#[derive(Props, PartialEq)]
pub struct DefaultTableCellProps<T: PartialEq> {
    #[props(default)]
    pub class: &'static str,

    pub value: T,

    pub precision: Option<usize>,
}

pub fn DefaultTableCellRenderer<T: Display + PartialEq>(
    cx: Scope<DefaultTableCellProps<T>>,
) -> Element {
    let text = match cx.props.precision {
        Some(precision) => format!("{:.precision$}", cx.props.value),
        None => format!("{}", cx.props.value),
    };

    cx.render(rsx! {
        td {
            class: "{cx.props.class}",
            "{text}"
        }
    })
}

pub struct TableHeadEvent<E> {
    pub event: E,
    pub column_index: usize,
    pub field: String,
}

#[derive(Props)]
pub struct DefaultTableHeaderProps<'a> {
    #[props(default)]
    pub class: &'a str,

    #[props(default)]
    pub onclick: EventHandler<'a, TableHeadEvent<MouseEvent>>,

    pub field: &'a str,
    pub column_index: usize,

    pub children: Element<'a>,
}

pub fn DefaultTableHeaderRenderer<'a>(cx: Scope<'a, DefaultTableHeaderProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        th {
            class: "{cx.props.class}",
            onclick: move |evt| {
                 cx.props.onclick.call(TableHeadEvent{
                    column_index: cx.props.column_index,
                    field: cx.props.field.to_owned(),
                    event: evt,
                 })
            },
            &cx.props.children
        }
    })
}
