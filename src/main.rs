use type_size_top::*;

fn main() {
    let options = options::Options {
        limit: 50,
        reverse: true,
        hide_less: 0,
    };

    let stdin = std::io::stdin().lock();
    let data = reader::read(stdin).unwrap();
    let types = parser::parse(&data).unwrap();
    let types = transformer::transform(types, &options);
    let output = formatter::format(types, &options);

    println!("{output}");
}
