use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::parse::Parse;
use syn::{parse_macro_input, Ident, Token};

struct InputConsumerMacro {
	name: Ident,
	types: Vec<Ident>,
}

impl Parse for InputConsumerMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let mut types: Vec<Ident> = Vec::new();
		let name: Ident = input.parse()?;
		input.parse::<Token![;]>()?;
		while !input.is_empty() {
			let ty: Ident = input.parse()?;
			types.push(ty);
			if !input.is_empty() {input.parse::<Token![,]>()?;}
		}
		Ok(InputConsumerMacro {
			name,
			types,
		})
    }
}

#[proc_macro]
pub fn derive_input_consumer(input: TokenStream) -> TokenStream {
	let mut subtypes_str: HashSet<&str> = HashSet::from(["AnyButtonConsumer", "ButtonConsumer", "HotkeyConsumer", "CursorPositionConsumer", "CursorMotionConsumer", "ScrollConsumer", "ResizeConsumer", "FocusConsumer", "CursorInWindowConsumer", "CloseConsumer"]);
	let InputConsumerMacro {
		name,
		types
	} = parse_macro_input!(input as InputConsumerMacro);

	for ty in types {
		subtypes_str.remove(ty.to_string().as_str());
	}

	let mut subtypes: Vec<Ident> = Vec::new();
	for t in subtypes_str {
		subtypes.push(format_ident!("{}", t));
	}

	let iter = subtypes.iter();

	let expanded = quote! {
		impl InputConsumer for RenderableTest {}
		use input::*;
		#(impl #iter for #name {fn accepts(&self) -> bool {false}})*
	};
	TokenStream::from(expanded)
}

// #[proc_macro]
// pub fn better_derive_input_consumer(input: TokenStream) -> TokenStream {
// 	let InputConsumerMacro {
// 		name,
// 		types
// 	} = parse_macro_input!(input as InputConsumerMacro);
// 	if types.contains(&format_ident!("AnyButtonConsumer")) {

// 	}
// 	todo!()
// }

#[test]
fn test() {
	assert_eq!(syn::parse_str::<Ident>("Hello").unwrap().to_string(), "Hello");
}