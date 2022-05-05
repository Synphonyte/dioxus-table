# Dioxus Table

<!-- Crates version -->
<a href="https://crates.io/crates/dioxus-table">
    <img src="https://img.shields.io/crates/v/dioxus-table.svg?style=flat" alt="Crates.io version" />
</a>

<!-- Sponsor -->
<a href="https://github.com/sponsors/Synphonyte">
    <img src="https://img.shields.io/github/sponsors/Synphonyte?logo=github-sponsors&style=flat" alt="GitHub Sponsors" />
</a>

Make data-driven table rendering easy with Dioxus ([Live Example]())

## Installation

Add the following to your Cargo.toml

```toml
[dependencies]
dioxus-table = "0.1.1"
```

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
    
    #[table(skip)] // don't show this column
    pub internal: u32,
}
```

This generates a Dioxus component called `Table` in the module `hotel` ready to be used in your app.

```rust
// in your app:
use hotel::{Hotel, Table as HotelTable};

fn App(cx: Scope) -> Element {
    // get some hotels
    let hotels = vec![
        Hotel { id: 1, name: "Hotel 1".to_owned(), city: "City 1".to_owned(), internal: 42 },
        Hotel { id: 2, name: "Hotel 2".to_owned(), city: "City 2".to_owned(), internal: 42 },
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

When you click on a head cell or on a data row you'll see some information logged to the console.

## Customization

You can customize most aspects of the table rendering. Here is an overview of all available options.

### Per-column/field options

| Option     | Description                                                                              |
|:-----------|:-----------------------------------------------------------------------------------------|
| class      | HTML class(es) added to the `<th>` and `<td>` tags                                       |
| cell_class | HTML class(es) added only to the `<td>` cell tag                                         |
| head_class | HTML class(es) added only to the `<th>` head tag                                         |
| title      | Custom title that is put into the `<th>`. By default the capitalized field name is used. |
| precision  | For decimal types this sets the number of digits after the decimal point                 |
| renderer   | Custom cell render component                                                             |
| skip       | Don't render this column                                                                 |

### Per-table options

| Option             | Description                                                                       |
|:-------------------|:----------------------------------------------------------------------------------|
| row_class          | HTML class(es) added to the `<tr>` row tags except the first one (the header row) |
| head_row_class     | Added only to the first `<tr>` (the header row)                                   |
| tag                | The HTML tag name used for the root of the table. Defaults to `"table"`.          |
| row_renderer       | Custom row renderer component                                                     |
| head_cell_renderer | Custom head cell renderer component                                               |
| dyn_row_classes    | Enables reactive row classes through the method `row_classes()`                   |

### Dynamic row classes

A simple way to give feedback to interactions is to add a class to a row element. This makes is easy to
highlight a selected row for example. Above the `struct` enable the `dyn_row_classes` option.

```rust
#[derive(PartialEq, TableData)]
#[table(dyn_row_classes)]
struct Hotel {
    ...
}
```

What classes are added to a row is determined by calling the `row_classes()` method. So let's implement it.

```rust
impl Hotel {
    fn row_classes(&self, index: usize, cx: Scope<T>) -> Vec<String> {
        if /* this hotel is selected */ {
            vec!["selected".to_string()]
        } else {
            vec![]
        }
    }
}
```

The method `row_classes()` is called for each row. It receives the `index` of the row and the current context.
Please look at the hotel example in the `examples` directory for a full example.

### Custom renderers

Custom renderers are a powerful way to customize almost all aspects of the table rendering.
Yet they are very easy to use.

#### Custom cell renderer

Probably the most common use for a custom renderer is to customize the representation of a value in a table cell.

Let's say we have a table of books with a `title` and a `rating` field. 
The `rating` field is an integer number from 1 to 5 that represents the number of stars a book has received.

```rust
#[derive(PartialEq, TableData)]
pub struct Book {
    pub title: i32,
    pub rating: u8,
}
```

With the default renderer we only see a number in the "Rating" column. If we want to display this
number as stars we can write a custom renderer.

```rust
#[derive(PartialEq, TableData)]
pub struct Book {
    pub title: i32,
    
    #[table(cell_renderer = "StarRenderer")] // specify the custom renderer
    pub rating: u8,
}

// The actual renderer component. It has to accept the DefaultTableCellProps.
pub fn StarRenderer(cx: Scope<DefaultTableCellProps<i32>>) -> Element {
    // the value of the rating field is provided as cx.props.value here
    let count = cx.props.value as usize; 

    // create a string with #count filled stars.
    let mut stars = "".to_owned();
    for _ in 0..count {
        stars += "★";
    }
    // then fill up the rest of the 5 stars with emtpy stars
    for _ in count..5 {
        stars += "☆";
    }

    // display the string
    cx.render(rsx! {
        td {
            class: "{cx.props.class}",
            "{stars}"
        }
    })
}
```

Now the rating is properly displayed as stars. To see this in action run the hotels example in the `examples/` folder.

#### Custom row and head cell renderers

They work basically the same as the cell renderers but are specified above the struct definition.

```rust
#[derive(PartialEq, TableData)]
#[table(row_renderer = "MyRowRenderer", head_cell_renderer = "MyHeadCellRenderer")]
pub struct Book {
    pub title: i32,
    pub rating: u8,
}
```

To see how to implement them please refer to the default renderers in src/cell_renderers.rs and src/row_enderers.rs. The easiest way to get going is to copy and paste the respective default renderer and customize from there.
