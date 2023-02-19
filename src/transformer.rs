use std::cmp::Reverse;

use crate::{options::Options, schema::*};

// TODO: link large types (parent-child).
// TODO: merge types with common prefix and similar layouts.
// TODO: support whitelist and blacklist.

fn merge_variants(variants: Vec<EnumVariant>) -> Vec<EnumVariant> {
    if variants.len() <= 1 {
        return variants;
    }

    let mut new = Vec::with_capacity(variants.len());

    for var in variants {
        if let Some(EnumVariant { name, size, items }) = new.last_mut() {
            if var.size == *size && &var.items == items {
                name.push_str(", ");
                name.push_str(&var.name);
                continue;
            }
        }

        new.push(var);
    }

    new
}

/// Sorts all variants and merges ones with similar layouts.
fn sort_and_merge_variants(types: &mut [Type]) {
    types
        .iter_mut()
        .filter_map(|type_| match &mut type_.kind {
            TypeKind::Enum(e) => Some(&mut e.variants),
            TypeKind::Struct(_) => None,
        })
        .for_each(|variants| {
            variants.sort_by_key(|v| Reverse(v.size));
            *variants = merge_variants(std::mem::take(variants));
        })
}

pub fn transform(mut types: Vec<Type>, options: &Options) -> Vec<Type> {
    // Use stable sort to preserve partial ordering.
    types.sort_by(|a, b| (b.size, &b.name).cmp(&(a.size, &a.name)));
    types.dedup();
    types.truncate(options.limit);

    sort_and_merge_variants(&mut types);

    types
}
