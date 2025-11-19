use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, RendererFlags},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    let mut config = Config::test_default().with_generate([(IdentType::Element, "tns:SDL")]);

    // Enable all parser flags. For details please refer to the flags documentation.
    config.parser.flags = ParserFlags::all();

    // Enable all interpreter flags. For details please refer to the flags documentation.
    config.interpreter.flags = InterpreterFlags::BUILDIN_TYPES | InterpreterFlags::DEFAULT_TYPEDEFS;

    // Enable all optimizer flags, except `REMOVE_DUPLICATES` because it can cause
    // some problems in some schemas, so it is disabled by default. For details
    // please refer to the flags documentation.
    config.optimizer.flags = OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATES;

    // Enable all generator flags. For details please refer to the flags documentation.
    config.generator.flags = GeneratorFlags::all();

    config.renderer.flags = RendererFlags::all();

    config
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/mixed_choice_any/schema.xsd",
        "tests/feature/mixed_choice_any/expected/default.rs",
        config(),
    );
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/mixed_choice_any/schema.xsd",
        "tests/feature/mixed_choice_any/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
fn generate_quick_xml_configured() {
    generate_test(
        "tests/feature/mixed_choice_any/schema.xsd",
        "tests/feature/mixed_choice_any/expected/quick_xml_configured.rs",
        config()
            .with_any_support(
                "xsd_parser::xml::AnyElement",
                "xsd_parser::xml::AnyAttributes",
            )
            .with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::tns::Sdl;

    // This read currently enters an infinite loop due to the interaction of
    // mixed="true" + xs:any inside an unbounded xs:choice.
    let _ = crate::utils::quick_xml_read_test::<Sdl, _>(
        "tests/feature/mixed_choice_any/example/default.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn mixed_choice_any_configured() {
    use quick_xml_configured::tns::Sdl;

    // With xs:any properly configured, this read must complete without hanging.
    let _ = crate::utils::quick_xml_read_test::<Sdl, _>(
        "tests/feature/mixed_choice_any/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_configured {
    #![allow(unused_imports)]

    include!("expected/quick_xml_configured.rs");
}
