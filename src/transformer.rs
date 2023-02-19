use crate::{options::Options, schema::*};

// TODO: merge variants with same layout.
// TODO: hide zero fields.
// TODO: link large types (parent-child).

pub fn transform(mut types: Vec<Type>, options: &Options) -> Vec<Type> {
    types.sort_unstable_by(|a, b| (b.size, &b.name).cmp(&(a.size, &a.name)));
    types.dedup();
    types.truncate(options.limit);
    types
}
