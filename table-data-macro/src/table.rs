use darling::util::IdentString;
use darling::{ast, util, FromDeriveInput, FromField};
use heck::ToTitleCase;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;

// TODO : type checked tailwind

#[derive(Debug, FromField)]
#[darling(attributes(table))]
struct TableDataField {
    ident: Option<syn::Ident>,
    // ty: syn::Type,
    // #[darling(default)]
    // pk: bool,
    #[darling(default)]
    renderer: Option<IdentString>,

    #[darling(default)]
    precision: Option<i64>,

    #[darling(default)]
    class: Option<String>,

    #[darling(default)]
    cell_class: Option<String>,

    #[darling(default)]
    head_class: Option<String>,

    #[darling(default)]
    title: Option<String>,
}

impl TableDataField {
    pub fn cell_class(&self) -> String {
        let mut class = "".to_owned();

        if let Some(ref c) = self.class {
            class.push_str(c);
        }
        if let Some(ref c) = self.cell_class {
            class.push(' ');
            class.push_str(c);
        }

        class
    }

    pub fn head_class(&self) -> String {
        let mut class = "".to_owned();

        if let Some(ref c) = self.class {
            class.push_str(c);
        }
        if let Some(ref c) = self.head_class {
            class.push(' ');
            class.push_str(c);
        }

        class
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(table),
    supports(struct_named),
    forward_attrs(allow, doc, cfg)
)]
pub struct TableDataDeriveInput {
    ident: syn::Ident,
    data: ast::Data<util::Ignored, TableDataField>,

    #[darling(default)]
    tag: Option<IdentString>,

    #[darling(default)]
    row_renderer: Option<IdentString>,

    #[darling(default)]
    row_class: Option<String>,

    #[darling(default)]
    head_row_class: Option<String>,

    #[darling(default)]
    head_cell_renderer: Option<IdentString>,
}

fn get_renderer_for_field(name: &syn::Ident, field: &TableDataField) -> TokenStream2 {
    let props = get_props_for_field(name, &field);

    if let Some(renderer) = &field.renderer {
        let ident = renderer.as_ident();
        quote! {
            rsx! {
                #ident {
                    #props
                }
            }
        }
    } else {
        quote! {
            rsx! {
                DefaultTableCellRenderer {
                    #props
                }
            }
        }
    }
}

fn get_head_renderer_for_field(
    name: &syn::Ident,
    field: &TableDataField,
    head_cell_renderer: &Option<IdentString>,
) -> TokenStream2 {
    let props = get_props_for_field(name, &field);

    if let Some(renderer) = &head_cell_renderer {
        let ident = renderer.as_ident();
        quote! {#ident}
    } else {
        quote! {DefaultTableHeaderRenderer}
    }
}

fn get_props_for_field(name: &syn::Ident, field: &TableDataField) -> TokenStream2 {
    let class = field.cell_class();

    let precision = if let Some(p) = &field.precision {
        quote!(precision: #p,)
    } else {
        quote!()
    };

    quote! {
        value: item.#name.clone(),
        class: #class,
        #precision
    }
}

impl ToTokens for TableDataDeriveInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let TableDataDeriveInput {
            ref ident,
            ref data,
            ref row_renderer,
            ref tag,
            ref row_class,
            ref head_row_class,
            ref head_cell_renderer,
        } = *self;

        let row_renderer = row_renderer
            .as_ref()
            .map(|r| r.as_ident().clone())
            .unwrap_or_else(|| syn::Ident::new("DefaultTableRowRenderer", row_renderer.span()));

        let tag = tag
            .as_ref()
            .map(|s| {
                let t = s.as_ident();
                quote!(#t)
            })
            .unwrap_or(quote!(table));

        let row_class = row_class
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or("".to_owned());

        let head_row_class = head_row_class
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or("".to_owned());

        let fields = data.as_ref().take_struct().expect("Is not enum").fields;

        let mut titles = vec![];
        let mut cells = vec![];

        for (i, f) in fields.into_iter().enumerate() {
            let name = f.ident.as_ref().expect("named field");
            let name_str = name.to_string();

            let title = if let Some(ref t) = f.title {
                t.clone()
            } else {
                name.to_string().to_title_case()
            };

            let head_class = f.head_class();

            let head_renderer = get_head_renderer_for_field(name, f, head_cell_renderer);
            titles.push(quote! { rsx! {
                #head_renderer {
                    class: #head_class,
                    column_index: #i,
                    field: #name_str,
                    onclick: move |evt| cx.props.onheadclick.call(evt),
                    #title
                }
            }});

            let cell_renderer = get_renderer_for_field(name, f);
            cells.push(quote! { rsx! {
                #cell_renderer
            }});
        }

        tokens.extend(quote! {
            use dioxus::events::MouseEvent;

            // TODO : pagination
            #[derive(Props)]
            pub struct TableProps<'a> {
                #[props(default)]
                class: &'a str,

                items: &'a Vec<#ident>,

                #[props(default)]
                pub onrowclick: EventHandler<'a, TableRowEvent<'a, #ident, MouseEvent>>,

                #[props(default)]
                pub onheadclick: EventHandler<'a, TableHeadEvent<MouseEvent>>,
            }

            #[allow(non_snake_case)]
            pub fn Table<'a>(cx: Scope<'a, TableProps<'a>>) -> Element {

                cx.render(rsx!{
                    #tag {
                        class: "{cx.props.class}",
                        tr {
                            class: #head_row_class,
                            #(#titles)*
                        }
                        cx.props.items.iter().enumerate().map(|(i, item)| rsx!(
                            #row_renderer {
                                index: i,
                                item: item,
                                class: #row_class,
                                onclick: move |evt| cx.props.onrowclick.call(evt),
                                #(#cells)*
                            }
                        ))
                    }
                })
            }
        });
    }
}
