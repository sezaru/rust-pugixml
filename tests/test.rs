extern crate pugixml;

use pugixml::tree::{DocumentBuilder, Encoding, FormatOption, NodeBase};
use pugixml::tree::document::to_file;
use pugixml::tree::first_child;
use pugixml::version;

#[test]
fn test() {
    match DocumentBuilder::new().from_file("tests/resources/cfe.xml").finish() {
        Ok(document) => {
            println!("Loaded!");
            let hue = to_file(&document,
                              "cfe2.xml",
                              "\thue",
                              vec![FormatOption::Default],
                              Encoding::Auto);
            println!("hue: {}", hue)
        }
        Err(why) => println!("Failed: {:?}", why),
    }

    match DocumentBuilder::new().from_string("<xml>huehue</xml>").finish() {
        Ok(document) => {
            println!("Loaded!");
            println!("document {:?}", document.kind());
            match first_child(&document).and_then(|a| first_child(&a)) {
                Ok(node) => println!("first_child kind {:?}", node.value()),
                Err(why) => println!("{:?}", why),
            }
        }
        Err(why) => println!("Failed: {:?}", why),
    }

    println!("pugixml version {}", version())
}
