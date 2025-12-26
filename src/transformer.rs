use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use regex::Regex;

use crate::{options::Options, schema::*};

/// Filters all types by size, regex filters and wrappers.
fn filter_types(types: &mut Vec<Type>, options: &Options) {
    // Skip filtering if no options are provided.
    if !options.remove_wrappers
        && options.hide_less.is_none()
        && options.filter.is_empty()
        && options.exclude.is_empty()
    {
        return;
    }

    types.retain(|type_| {
        // Remove by size.
        if type_.size < options.hide_less.unwrap_or(0) {
            return false;
        }

        // Remove wrappers (`MaybeUninit` etc).
        if options.remove_wrappers && is_wrapper(type_) {
            return false;
        }

        // Remove by explicit patterns.
        if options
            .exclude
            .iter()
            .any(|pattern| pattern.is_match(&type_.name))
        {
            return false;
        }

        options.filter.is_empty()
            || options
                .filter
                .iter()
                .any(|pattern| pattern.is_match(&type_.name))
    });
}

/// Detects wrappers like `MaybeUninit` and custom ones.
fn is_wrapper(type_: &Type) -> bool {
    if type_.end_padding.is_some() {
        return false;
    }

    let pred = |type_size, items: &[FieldOrPadding]| {
        items.iter().all(|f| f.size() == type_size || f.size() == 0)
    };

    match &type_.kind {
        TypeKind::Struct(s) => pred(type_.size, &s.items),
        TypeKind::Enum(e) => {
            e.discriminant_size.is_none() && e.variants.iter().all(|v| pred(type_.size, &v.items))
        }
    }
}

/// Retains only types that match patterns and their children using fields'
/// types when provided. If `use_size` is true, it also uses sizes in fields.
///
/// Types must be sorted in descending order by size.
fn expand(types: &mut Vec<Type>, patterns: &[Regex], use_size: bool) {
    if patterns.is_empty() {
        return;
    }

    let mut names = HashSet::new();
    let mut sizes = HashSet::new();

    types.retain(|type_| {
        if !names.contains(&type_.name)
            && !sizes.contains(&type_.size)
            && !patterns.iter().any(|p| p.is_match(&type_.name))
        {
            return false;
        }

        let append = |f: &FieldOrPadding| match f {
            FieldOrPadding::Field(f) => {
                if let Some(name) = &f.local_type {
                    names.insert(name.clone());
                } else if use_size {
                    sizes.insert(f.size);
                }
            }
            FieldOrPadding::Padding(_) => {}
        };

        match &type_.kind {
            TypeKind::Struct(s) => s.items.iter().for_each(append),
            TypeKind::Enum(e) => e
                .variants
                .iter()
                .flat_map(|variant| variant.items.iter())
                .for_each(append),
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
    if type_.end_padding.is_some_and(|p| p < threshold) {
        type_.end_padding = None;
    }

    let do_it = |fields: &mut Vec<FieldOrPadding>| {
        fields.retain(|f| f.size() >= threshold);
    };

    match &mut type_.kind {
        TypeKind::Struct(s) => do_it(&mut s.items),
        TypeKind::Enum(e) => {
            if e.discriminant_size.is_some_and(|p| p < threshold) {
                e.discriminant_size = None;
            }

            for variant in &mut e.variants {
                do_it(&mut variant.items);
            }
        }
    };
}

pub fn transform(mut types: Vec<Type>, options: &Options) -> Vec<Type> {
    filter_types(&mut types, options);

    // Use stable sort to preserve partial ordering.
    // Also sort by name to do proper deduplication.
    types.sort_by(|a, b| (b.size, &b.name).cmp(&(a.size, &a.name)));
    types.dedup();

    expand(&mut types, &options.expand, options.expand_by_size);

    if let Some(limit) = options.limit {
        types.truncate(limit);
    }

    for type_ in &mut types {
        if let Some(threshold) = options.hide_less {
            remove_small_fields(type_, threshold);
        }

        if options.sort_fields {
            sort_fields_and_remove_paddings(type_);
        }

        sort_and_merge_variants(type_);
    }

    types
}
