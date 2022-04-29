# Dioxus Table

Make data-driven table rendering easy with Dioxus

## Quickstart

Attach the derive macro `TableData` to a struct that represents a row of a table.

```rust
// in mod hotel:

#[derive(PartialEq, TableData)]
pub struct Hotel {
    #[table(class = "text-end")] // right align numbers
    pub id: i32,

    #[table(title = "Hotel Name")] // custom title
    pub name: String,

    pub city: String,
}
```

This generates a Dioxus component ready to be used in your app.

```rust
// in your app:
use hotel::{Hotel, Table as HotelTable};

fn App(cx: Scope) -> Element {
    // get some hotels
    let hotels = vec![
        Hotel { id: 1, name: "Hotel 1".into(), city: "City 1".into() },
        Hotel { id: 2, name: "Hotel 2".into(), city: "City 2".into() },
    ];
    
    cx.render(rsx! {
        h1 { "Hotel Table" }
        HotelTable {
            class: "table",
            items: &hotels,
        }
    })
}
```

And that's it! Easy, right?

You can look at the examples in the `examples` directory to get a more complete overview of what dioxus-table can do.

## Event Handlers

The generated table component provides two events: Clicking on a row or a head cell.
Let's add some event handlers to our previous example.

```rust
cx.render(rsx! {
    h1 { "Hotel Table" }
    HotelTable {
        class: "table",
        items: &hotels,
        onrowclick: move |evt: TableRowEvent<_, _>| {
            web_sys::console::log_1(&format!("Row {}", evt.row_index).into());
        },
        onheadclick: move |evt: TableHeadEvent<_>| {
            web_sys::console::log_1(&format!("Head {} '{}'", evt.column_index, evt.field).into());
        },
    }
})
```

## Customization

### Classes

Per table / row / cell

### Custom renderers

```rust
```