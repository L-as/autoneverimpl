#[macro_use]
extern crate quote;
extern crate proc_macro;

use syn::TraitItem;
use proc_macro2::Span;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn autoneverimpl(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	assert!(args.into_iter().next().is_none(), "autoneverimpl doesn't accept any arguments!");

	let input: syn::ItemTrait = syn::parse(input).unwrap();

	let ident = input.ident;

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let items = input.items;

	let impl_items: proc_macro2::TokenStream = items.into_iter().flat_map(|item| {
		match item {
			TraitItem::Type(mut ty) => {
				if ty.default.is_none() {
					ty.default = Some((syn::token::Eq::new(Span::call_site()), syn::Type::Never(syn::TypeNever {bang_token: syn::token::Bang::new(Span::call_site())})));
				}

				TraitItem::Type(ty).into_token_stream().into_iter()
			},
			item => item.into_token_stream().into_iter()
		}
	}).collect();

	let expanded = quote! {
		impl #impl_generics #ident #ty_generics for ! #where_clause {
			#impl_items
		}
	};

	expanded.into()
}
