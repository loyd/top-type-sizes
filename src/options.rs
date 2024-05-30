use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Options {
    /// Shows only this number of top types.
    ///
    /// This limit is applied after all other filters.
    /// {n}{n}{n}
    #[structopt(short = "l", long)]
    pub limit: Option<usize>,
    /// Prints types in descending order.
    ///
    /// This option is applied after the -l/--limit option.
    /// {n}{n}{n}
    #[structopt(short = "r", long)]
    pub reverse: bool,
    /// Hides wrappers like `MaybeUninit` and `ManuallyDrop`.
    ///
    /// This option removes types having the same layout as an inner type.
    /// {n}{n}{n}
    #[structopt(short = "w", long)]
    pub remove_wrappers: bool,
    /// Hides types and fields with size less than this value.
    #[structopt(short = "h", long)]
    pub hide_less: Option<usize>,
    /// Sorts fields by size and hides paddings.
    ///
    /// Note: enum variants are sorted and merged anyway.
    /// {n}{n}{n}
    #[structopt(short = "s", long)]
    pub sort_fields: bool,
    /// Shows only types that match these patterns.
    ///
    /// Patterns are regex (in the regex crate's syntax).
    /// Can be provided multiple times.
    /// {n}{n}{n}
    #[structopt(short = "f", long)]
    pub filter: Vec<Regex>,
    /// Excludes types that match these patterns.
    ///
    /// Patterns are regex (in the regex crate's syntax).
    /// Can be provided multiple times.
    /// {n}{n}{n}
    #[structopt(short = "e", long)]
    pub exclude: Vec<Regex>,
    /// Shows only types that match these patterns and their children.
    ///
    /// It uses two mechanisms to expand types: {n}
    /// - by field's type name (requires at least nightly 24-03-22) {n}
    /// - by field's size if the `--expand_by_size` option is enabled
    ///
    /// Note: currently field's type names are provided only for `await`.
    ///
    /// Patterns are regex (in the regex crate's syntax).
    /// Can be provided multiple times.
    /// {n}{n}{n}
    #[structopt(short = "p", long)]
    pub expand: Vec<Regex>,
    /// Modify the -p/--expand option to expand also by field's size.
    #[structopt(long)]
    pub expand_by_size: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self::from_iter(vec!["top-type-sizes"])
    }
}
