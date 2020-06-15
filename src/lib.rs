#![feature(box_syntax)] // :)

extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;

pub(crate) mod types {
	use syn::{
		Expr, Result,
		buffer::Cursor,
		parse::{Parse, ParseStream},
		token::CustomToken,
	};

	use proc_macro2::TokenStream;

	syn::custom_punctuation!(ImplicationSign, ==>);

	#[derive(Debug)]
	pub(crate) struct Implication {
		pub lhs: Expr,
		pub rhs: Expr,
	}

	impl Parse for Implication {
		fn parse(input: ParseStream) -> Result<Self> {
			let lhs = input.step(|cursor| {
				let mut rest = *cursor;
				let mut accumulator: TokenStream = TokenStream::new();
				let mut last_valid: Option<(Expr, Cursor)> = None;

				while let Some((tt, next)) = rest.token_tree() {
					accumulator.extend(vec![tt].into_iter());

					if <ImplicationSign as CustomToken>::peek(rest.clone()) {
						break;
					}

					if let Ok(exp) = syn::parse::<Expr>(accumulator.clone().into()) {
						last_valid = Some((exp, next));
					}

					rest = next;
				}

				last_valid.ok_or(cursor.error("tohle neni implikace lol"))
			})?;

			let _ = input.call(ImplicationSign::parse)?;

			let rhs = input.call(Expr::parse)?;

			Ok(Self { lhs, rhs })
		}
	}
}

#[proc_macro]
pub fn implication(item: TokenStream) -> TokenStream {
	let imp: types::Implication = syn::parse(item).unwrap();

	dbg!(imp);

	todo!("vakaras suck my dick");
}
