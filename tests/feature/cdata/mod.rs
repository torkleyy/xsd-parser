use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "tns:Document")])
}

fn config_mixed() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "tns:MixedContent")])
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/cdata/schema.xsd",
        "tests/feature/cdata/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Document;

    let obj = crate::utils::quick_xml_read_test::<Document, _>(
        "tests/feature/cdata/example/default.xml",
    );

    assert_eq!(obj.title, "Sample Document");
    assert_eq!(obj.content, "This is <text> with special chars & symbols");
    assert_eq!(obj.code, "fn main() {\n    println!(\"Hello, world!\");\n}");
}

#[test]
fn generate_quick_xml_mixed() {
    generate_test(
        "tests/feature/cdata/schema.xsd",
        "tests/feature/cdata/expected/quick_xml_mixed.rs",
        config_mixed().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_mixed() {
    use quick_xml_mixed::MixedContent;

    let obj = crate::utils::quick_xml_read_test::<MixedContent, _>(
        "tests/feature/cdata/example/mixed.xml",
    );

    assert_eq!(obj.description, "This has <escaped> text and <unescaped> & raw content mixed together with & more escapes");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_mixed {
    #![allow(unused_imports)]

    include!("expected/quick_xml_mixed.rs");
}
