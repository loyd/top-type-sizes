use top_type_sizes::*;

macro_rules! test_sample {
    ($name:ident) => {
        #[test]
        fn $name() {
            static SAMPLE: &'static str =
                include_str!(concat!("samples/", stringify!($name), ".txt"));

            let refined_sample = reader::read(SAMPLE.as_bytes()).unwrap();
            let types = parser::parse(&refined_sample).unwrap();
            insta::assert_debug_snapshot!(format!("{}_types", stringify!($name)), types);

            // TODO: check the transformer.

            let output = formatter::format(types, &Default::default());
            println!("{output}");
            // TODO: make snapshots of output.
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
