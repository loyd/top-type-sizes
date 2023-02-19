use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Options {
    /// Shows only this number of top types.
    #[structopt(short, long, default_value = "100")]
    pub limit: usize,
    /// Prints top `limit` types in ascending order.
    #[structopt(short, long)]
    pub reverse: bool,
    /// Hides fields with a size less than this value.
    #[structopt(short, long, default_value = "0")]
    pub hide_less: usize,
    /// Sorts fields by size and removes paddings.
    #[structopt(long)]
    pub sort_fields: bool,
}
