use std::cmp::Reverse;

use crate::{options::Options, schema::*};

// TODO: merge variants with same layout.
// TODO: hide zero fields.
// TODO: link large types (parent-child).

pub fn transform(mut types: Vec<Type>, options: &Options) -> Vec<Type> {
    types.sort_unstable_by_key(|t| Reverse(t.size));
    types.truncate(options.limit);
    types
}
