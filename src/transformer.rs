use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use regex::Regex;

use crate::{options::Options, schema::*};

// TODO: link large types (parent-child).
// TODO: merge types with common prefix and similar layouts.
// TODO: support whitelist and blacklist.

/// Filters all types using whitelisted and blacklisted patterns.
fn filter_types(types: &mut Vec<Type>, filters: &[Regex], exclude: &[Regex]) {
    if filters.is_empty() && exclude.is_empty() {
        return;
    }

    types.retain(|type_| {
        exclude.iter().all(|pattern| !pattern.is_match(&type_.name))
            && (filters.is_empty() || filters.iter().any(|pattern| pattern.is_match(&type_.name)))
    });
}

/// Retains only types that match patterns and their children. Based on
/// hueristics. Types must be sorted in descending order.
fn expand(types: &mut Vec<Type>, patterns: &[Regex]) {
    if patterns.is_empty() {
        return;
    }

    let field_size = |f: &FieldOrPadding| match f {
        FieldOrPadding::Field(f) => Some(f.size),
        FieldOrPadding::Padding(_) => None,
    };

    let mut sizes = HashSet::new();

    types.retain(|type_| {
        if !sizes.contains(&type_.size) && !patterns.iter().any(|p| p.is_match(&type_.name)) {
            return false;
        }

        match &type_.kind {
            TypeKind::Struct(s) => {
                sizes.extend(s.items.iter().filter_map(field_size));
            }
            TypeKind::Enum(e) => sizes.extend(
                e.variants
                    .iter()
                    .flat_map(|variant| variant.items.iter())
                    .filter_map(field_size),
            ),
        }

        true
    });
}

/// Sorts all variants and merges ones with similar layouts.
fn sort_and_merge_variants(type_: &mut Type) {
    let TypeKind::Enum(e) = &mut type_.kind else {
        return;
    };

    if e.variants.len() <= 1 {
        return;
    }

    let old = std::mem::take(&mut e.variants);
    let mut variants = HashMap::<usize, Vec<EnumVariant>>::with_capacity(old.len());

    for next_variant in old {
        let entry = variants.entry(next_variant.size).or_default();

        // Try to find a variant with the same layout.
        if let Some(v) = entry.iter_mut().find(|v| next_variant.items == v.items) {
            v.name.push_str(", ");
            v.name.push_str(&next_variant.name);
        } else {
            entry.push(next_variant);
        }
    }

    let mut new = variants
        .into_iter()
        .flat_map(|(_, vec)| vec)
        .collect::<Vec<_>>();

    new.sort_by_key(|v| Reverse(v.size));
    e.variants = new;
}

/// Sorts fields and removes paddings.
fn sort_fields_and_remove_paddings(type_: &mut Type) {
    type_.end_padding = None;

    let do_it = |fields: &mut Vec<_>| {
        fields.retain(|f| matches!(f, FieldOrPadding::Field(_)));
        fields.sort_by_key(|f| Reverse(f.size()))
    };

    match &mut type_.kind {
        TypeKind::Struct(s) => do_it(&mut s.items),
        TypeKind::Enum(e) => {
            for variant in &mut e.variants {
                do_it(&mut variant.items);
            }
        }
    };
}

/// Removes fields and paddings smaller than `threshold`.
fn remove_small_fields(type_: &mut Type, threshold: usize) {
    if type_.end_padding.map_or(false, |p| p < threshold) {
        type_.end_padding = None;
    }

    let do_it = |fields: &mut Vec<FieldOrPadding>| {
        fields.retain(|f| f.size() >= threshold);
    };

    match &mut type_.kind {
        TypeKind::Struct(s) => do_it(&mut s.items),
        TypeKind::Enum(e) => {
            if e.discriminant_size.map_or(false, |p| p < threshold) {
                e.discriminant_size = None;
            }

            for variant in &mut e.variants {
                do_it(&mut variant.items);
            }
        }
    };
}

/// Removes wrappers like `MaybeUninit` and custom ones.
fn remove_wrappers(types: &mut Vec<Type>) {
    let is_wrapper = |type_size, items: &[FieldOrPadding]| {
        items.iter().all(|f| f.size() == type_size || f.size() == 0)
    };

    types.retain(|type_| {
        if type_.end_padding.is_some() {
            return true;
        }

        match &type_.kind {
            TypeKind::Struct(s) => !is_wrapper(type_.size, &s.items),
            TypeKind::Enum(e) => {
                e.discriminant_size.is_some()
                    || !e.variants.iter().all(|v| is_wrapper(type_.size, &v.items))
            }
        }
    });
}

pub fn transform(mut types: Vec<Type>, options: &Options) -> Vec<Type> {
    filter_types(&mut types, &options.filter, &options.exclude);

    // Use stable sort to preserve partial ordering.
    // Also sort by name to do proper deduplication.
    types.sort_by(|a, b| (b.size, &b.name).cmp(&(a.size, &a.name)));
    types.dedup();

    if options.remove_wrappers {
        remove_wrappers(&mut types);
    }

    expand(&mut types, &options.expand);

    types.truncate(options.limit);

    for type_ in &mut types {
        if options.hide_less > 0 {
            remove_small_fields(type_, options.hide_less);
        }

        if options.sort_fields {
            sort_fields_and_remove_paddings(type_);
        }

        sort_and_merge_variants(type_);
    }

    types
}
