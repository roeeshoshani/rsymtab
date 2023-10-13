use quote::{format_ident, quote, quote_spanned, spanned::Spanned, ToTokens};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn export(
    attribute_tokens: proc_macro::TokenStream,
    item_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if !attribute_tokens.is_empty() {
        return quote! {
            compile_error!("this attribute macro does not accept any arguments");
        }
        .into();
    }
    let item = parse_macro_input!(item_tokens as syn::Item);
    let (item_ident, item_ptr) = match &item {
        syn::Item::Fn(fn_item) => (&fn_item.sig.ident, fn_item.sig.ident.to_token_stream()),
        syn::Item::Static(static_item) => {
            let ident = &static_item.ident;
            (
                ident,
                quote! {
                    &#ident as *const _
                },
            )
        }
        _ => {
            return quote! {
                compile_error!("this type of item can not be exported")
            }
            .into()
        }
    };
    let item_name = item_ident.to_string();
    let sym_item_name = format_ident!("__RSYMTAB_ITEM_{}", item_name);
    let section_name = format!("rsymtab.{}", item_name);
    quote! {
        #item

        #[link_section = #section_name]
        static #sym_item_name: ::rsymtab::RsymtabSymbol = ::rsymtab::RsymtabSymbol {
            name: #item_name,
            address: ::rsymtab::SymbolAddress::_from_reference(unsafe { &*(#item_ptr as *const ()) })
        };
    }
    .into()
}
