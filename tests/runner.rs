use structopt::StructOpt;

use top_type_sizes::*;

fn test(content: &'static str) {
    let refined_content = reader::read(content.as_bytes()).unwrap();

    let types = parser::parse(&refined_content).unwrap();
    insta::with_settings!({ description => "internal representation" }, {
        insta::assert_yaml_snapshot!("types", types);
    });

    snap_output(&types, &[]);
    snap_output(&types, &["-ws"]);
    snap_output(&types, &["-h8"]);
    snap_output(&types, &["-ws", "-h16"]);
}

fn snap_output(types: &[schema::Type], cmd: &[&str]) {
    let full_cmd = std::iter::once(&"top-type-sizes")
        .chain(cmd)
        .copied()
        .collect::<Vec<_>>();

    let options = options::Options::from_iter(&full_cmd);
    let snap_name = {
        let mut parts = cmd.to_vec();
        parts.insert(0, "output");
        parts.join("")
    };

    let types = transformer::transform(types.to_vec(), &options);
    let output = formatter::format(types, &options);

    insta::with_settings!({ description => full_cmd.join(" ") }, {
        insta::assert_snapshot!(snap_name, output);
    })
}

macro_rules! test_sample {
    ($name:ident) => {
        #[test]
        fn $name() {
            insta::with_settings!(
            {
                snapshot_path => concat!("snapshots/", stringify!($name)),
                prepend_module_to_snapshot => false,
                omit_expression => true,
            },
            {
                test(include_str!(concat!("samples/", stringify!($name), ".txt")))
            })
        }
    };
}

test_sample!(command);
test_sample!(control_flow);
test_sample!(alignment_enum);
test_sample!(tokio_udp);
test_sample!(several_types);
test_sample!(timex);
test_sample!(async_fn);
test_sample!(compiler_messages);

// TODO: add samples from rustc tests.
