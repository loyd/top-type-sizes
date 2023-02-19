use std::cmp::Reverse;

use crate::{options::Options, schema::*};

// TODO: link large types (parent-child).
// TODO: merge types with common prefix and similar layouts.
// TODO: support whitelist and blacklist.

/// Sorts all variants and merges ones with similar layouts.
fn sort_and_merge_variants(type_: &mut Type) {
    let TypeKind::Enum(e) = &mut type_.kind else {
        return;
    };

    if e.variants.len() <= 1 {
        return;
    }

    let mut old = std::mem::take(&mut e.variants);
    old.sort_by_key(|v| Reverse(v.size)); // TODO: sort also by hash of items.

    let mut new = Vec::with_capacity(old.len());

    for var in old {
        if let Some(EnumVariant { name, size, items }) = new.last_mut() {
            if var.size == *size && &var.items == items {
                name.push_str(", ");
                name.push_str(&var.name);
                continue;
            }
        }

        new.push(var);
    }

    e.variants = new;
}

pub fn transform(mut types: Vec<Type>, options: &Options) -> Vec<Type> {
    // Use stable sort to preserve partial ordering.
    // Also sort by name to do proper deduplication.
    types.sort_by(|a, b| (b.size, &b.name).cmp(&(a.size, &a.name)));
    types.dedup();
    types.truncate(options.limit);

    for type_ in &mut types {
        sort_and_merge_variants(type_);
    }

    types
}
