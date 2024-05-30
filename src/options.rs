use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Options {
    /// Shows only this number of top types.
    #[structopt(short = "l", long)]
    pub limit: Option<usize>,
    /// Prints top `limit` types in ascending order.
    #[structopt(short = "r", long)]
    pub reverse: bool,
    /// Removes wrappers like `MaybeUninit`.
    #[structopt(short = "w", long)]
    pub remove_wrappers: bool,
    /// Hides types and fields with size less than this value.
    #[structopt(short = "h", long)]
    pub hide_less: Option<usize>,
    /// Sorts fields by size and removes paddings.
    #[structopt(short = "s", long)]
    pub sort_fields: bool,
    /// Shows only types that match these patterns.
    #[structopt(short = "f", long)]
    pub filter: Vec<Regex>,
    /// Excludes types that match these patterns.
    #[structopt(short = "e", long)]
    pub exclude: Vec<Regex>,
    /// Shows only types that match these patterns and their children,
    /// heuristically.
    #[structopt(short = "p", long)]
    pub expand: Vec<Regex>,
}

impl Default for Options {
    fn default() -> Self {
        Self::from_iter(vec!["top-type-sizes"])
    }
}
