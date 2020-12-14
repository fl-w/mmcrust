use std::{path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.h");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir);
    let source_dir = format!("{}/src", std::env::var("CARGO_MANIFEST_DIR").unwrap());

    // build bison/yacc files, probably only works on linux
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "flex -o {}/lex.yy.c src/C.flex && bison -d -t -v src/C.y -b {}/C >/dev/null",
            out_dir, out_dir
        ))
        .output()
        .expect("failed to build bison/yacc files.");

    cc::Build::new()
        .include("src")
        .file(out_path.join("lex.yy.c"))
        .file(out_path.join("C.tab.c"))
        .file("src/symbol_table.c")
        .file("src/nodes.c")
        .file("src/main.c")
        .warnings(false)
        .compile("parser");

    let bindings = bindgen::Builder::default()
        .clang_args(format!("-I{} -I{}", source_dir, out_dir).split(' '))
        .header("src/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
