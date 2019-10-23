use failure::Error;
use pest::iterators::Pair;

use crate::document_tree::{
	ExtraAttributes,
	elements as e,
	element_categories as c,
	extra_attributes as a,
	attribute_types as at
};

use crate::parser::{
	pest_rst::Rule,
//    pair_ext_parse::PairExt,
};

use super::whitespace_normalize_name;


pub fn convert_inline(pair: Pair<Rule>) -> Result<c::TextOrInlineElement, Error> {
	Ok(match pair.as_rule() {
		Rule::str       		=> pair.as_str().into(),
		Rule::reference 		=> convert_reference(pair)?.into(),
		Rule::substitution_ref 	=> convert_substitution(pair)?.into(),
		rule => unimplemented!("unknown rule {:?}", rule),
	})
}

fn convert_reference(pair: Pair<Rule>) -> Result<e::Reference, Error> {
	let name;
	let refuri = None;
	let refid;
	let refname = vec![];
	let concrete = pair.into_inner().next().unwrap();
	match concrete.as_rule() {
		Rule::reference_target => {
			let rt_inner = concrete.into_inner().next().unwrap(); // reference_target_uq or target_name_qu
			refid = Some(rt_inner.as_str().into());
			name  = Some(rt_inner.as_str().into());
		},
		Rule::reference_explicit => unimplemented!("explicit reference"),
		Rule::reference_auto => unimplemented!("auto reference"),
		_ => unreachable!(),
	};
	Ok(e::Reference::with_extra(
		a::Reference { name, refuri, refid, refname }
	))
}

fn convert_substitution(pair: Pair<Rule>) -> Result<e::SubstitutionReference, Error> {
	let concrete = pair.into_inner().next().unwrap();
	match concrete.as_rule() {
		Rule::substitution_name => {
			let name = whitespace_normalize_name(concrete.as_str());
			Ok(a::ExtraAttributes::with_extra(
				a::SubstitutionReference {
					refname: vec![at::NameToken(name)]
				}
			))
		}
		_ => unreachable!()
	}
}
