// fn main() {
//     cc::Build::new()
//         .cpp(true)
//         .flag("-std=c++14")
//         .flag("-fno-builtin-malloc")
//         .flag("-Bsymbolic")
//         .define("NDEBUG", None)
//         .define("_REENTRANT", Some("1"))
//         .opt_level(3)
//         .include("./Hoard/src")
//         .include("./Hoard/src/include")
//         .include("./Hoard/src/include/util")
//         .include("./Hoard/src/include/hoard")
//         .include("./Hoard/src/include/superblocks")
//         .include("./Heap-Layers")
//         .include("./")
//         .file("Hoard/src/source/libhoard.cpp")
//         .file("Hoard/src/source/unixtls.cpp")
//         .file("Heap-Layers/wrappers/gnuwrapper.cpp")
//         .compile("libhoard.a");
// }

use std::path::PathBuf;
use std::process::Command;

const CPPFLAGS: &'static str = "-std=c++14 -flto -O3 -DNDEBUG -ffast-math -fno-builtin-malloc -Wall -Wextra -Wshadow -Wconversion -Wuninitialized -Dalways_inline=";

fn main() {
    // target/{build}/hoard-sys-{hash}/out
    let build_dir = PathBuf::from(std::env::var("OUT_DIR").expect("No OUT_DIR."));
    // target/{build}/hoard-sys-{hash}/out/Hoard
    let hoard_build = build_dir.join("Hoard");
    // target/{build}/hoard-sys-{hash}/out/Hoard/src
    let hoard_src = build_dir.join("Hoard/src");
    // target/{build}/hoard-sys-{hash}/out/Hoard/src/Heap-Layers
    let heaplayers_build = build_dir.join("Hoard/src/Heap-Layers");

    // Project root and submodules
    let root_dir = project_root::get_project_root()
        .expect("Failed to get project root.")
        .join("hoard-allocator");
    let hoard_module = root_dir.join("Hoard");
    let heaplayers_module = root_dir.join("Heap-Layers");

    // Copy Hoard submodule to hoard_build
    if !hoard_build.exists() {
        let mut cmd = Command::new("cp");
        cmd.current_dir(&build_dir).args(&[
            "-r",
            &hoard_module.to_str().unwrap(),
            &hoard_build.to_str().unwrap(),
        ]);
        run_command(&mut cmd);
        assert!(
            hoard_build.exists(),
            "Hoard is not symlinked properly: {:?}",
            hoard_build
        );
    }

    // Copy Heap-Layers submodule to heaplayers_build
    if !heaplayers_build.exists() {
        let mut cmd = Command::new("cp");
        cmd.current_dir(&hoard_src).args(&[
            "-r",
            &heaplayers_module.to_str().unwrap(),
            &heaplayers_build.to_str().unwrap(),
        ]);
        run_command(&mut cmd);
        assert!(
            heaplayers_build.exists(),
            "Heap-Layers is not symlinked properly: {:?}",
            heaplayers_build
        );
    }

    // Run make in hoard_src
    let mut cmd = Command::new("make");
    cmd.current_dir(&hoard_src)
        .args(&[&get_hoard_build_string(), &format!("CPPFLAGS={}", CPPFLAGS)]);
    run_command(&mut cmd);

    println!("cargo:rustc-link-lib=static=hoard");
    println!(
        "cargo:rustc-link-search=native={}",
        &hoard_src.to_str().unwrap()
    );
}

fn run_command(cmd: &mut Command) {
    match cmd.status() {
        Ok(ret) => assert!(ret.success(), "{:?} returned non-zero: {:?}", cmd, ret),
        Err(e) => panic!("Failed to run command {:?}: {:?}", cmd, e),
    }
}

fn get_hoard_build_string() -> String {
    use uname::uname;
    match uname() {
        Ok(info) => format!("{}-gcc-{}-static", info.sysname, info.machine),
        Err(e) => panic!("Failed to get uname: {:?}", e),
    }
}
