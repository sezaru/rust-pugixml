extern crate gcc;

fn main()
{
    gcc::Config::new().cpp(true)
        .file("wrapper/wrapper.cpp")
        .file("pugixml/src/pugixml.cpp")
        .include("pugixml/src")
        .cpp_set_stdlib(Some("c++"))
        .flag("--sysroot=/home/ebarreto/x-tools/i686-unknown-linux-gnu/i686-unknown-linux-gnu/sysroot")
        .compile("libpugixml.a");
}
