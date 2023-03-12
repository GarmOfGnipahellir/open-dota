use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Widget, attributes(bundle))]
pub fn derive_widget(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, attrs, .. } = parse_macro_input!(item as DeriveInput);

    let get_widget_registration_impl = get_widget_registration(&ident);

    let bundle_widget_impl = attrs
        .iter()
        .find(|&attr| attr.path.is_ident(&Ident::new("bundle", Span::call_site())))
        .map(|_| bundle_widget(&ident));

    quote! {
        #get_widget_registration_impl

        #bundle_widget_impl
    }
    .into()
}

fn get_widget_registration(ident: &Ident) -> TokenStream {
    quote! {
        impl GetWidgetRegistration for #ident {
            fn get_widget() -> Box<dyn Widget> {
                Box::new(Self::default())
            }

            fn get_widget_registration() -> WidgetRegistration {
                WidgetRegistration::of::<Self>()
            }
        }
    }
}

fn bundle_widget(ident: &Ident) -> TokenStream {
    let bundle = Ident::new(&format!("{}Bundle", ident), Span::call_site());
    quote! {
        impl Widget for #ident {
            fn spawn<'w>(&self, parent: &'w mut WorldChildBuilder) -> EntityMut<'w> {
                parent.spawn(#bundle::default())
            }
        }
    }
}
