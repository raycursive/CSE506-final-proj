
fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .flag("-fno-builtin-malloc")
        .flag("-Bsymbolic")
        .define("NDEBUG", None)
        .define("_REENTRANT", Some("1"))
        .opt_level(3)
        .include("./Hoard/src")
        .include("./Hoard/src/include")
        .include("./Hoard/src/include/util")
        .include("./Hoard/src/include/hoard")
        .include("./Hoard/src/include/superblocks")
        .include("./Heap-Layers")
        .include("./")
        .file("Hoard/src/source/libhoard.cpp")
        .file("Hoard/src/source/unixtls.cpp")
        .file("Heap-Layers/wrappers/gnuwrapper.cpp")
        .compile("libhoard.a");
}
