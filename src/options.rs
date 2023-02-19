use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Options {
    #[structopt(short, long, default_value = "100")]
    pub limit: usize,
    #[structopt(short, long)]
    pub reverse: bool,
    #[structopt(short, long, default_value = "0")]
    pub hide_less: usize,
}
