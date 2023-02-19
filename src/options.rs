use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Options {
    /// Shows only this number of top types.
    #[structopt(short, long, default_value = "100")]
    pub limit: usize,
    /// Prints top `limit` types in ascending order.
    #[structopt(short, long)]
    pub reverse: bool,
    /// Removes wrappers like `MaybeUninit`.
    #[structopt(short = "w", long)]
    pub remove_wrappers: bool,
    /// Hides fields with size less than this value.
    #[structopt(short, long, default_value = "0")]
    pub hide_less: usize,
    /// Sorts fields by size and removes paddings.
    #[structopt(short, long)]
    pub sort_fields: bool,
}
