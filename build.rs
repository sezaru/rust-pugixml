extern crate gcc;

fn main()
{
    gcc::Config::new().cpp(true)
        .file("wrapper/wrapper.cpp")
        .file("pugixml/src/pugixml.cpp")
        .include("pugixml/src")
        .compile("libpugixml.a")
}
