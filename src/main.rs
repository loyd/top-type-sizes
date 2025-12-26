use structopt::StructOpt;

use top_type_sizes::*;

fn main() -> eyre::Result<()> {
    let options = options::Options::from_args();
    let stdin = std::io::stdin().lock();
    let data = reader::read(stdin)?;
    let types = parser::parse(&data)?;
    let types = transformer::transform(types, &options);
    let output = formatter::format(types, &options);

    println!("{output}");
    Ok(())
}
