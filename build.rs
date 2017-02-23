extern crate bindgen;
extern crate regex;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use regex::Regex;

enum Machine {
    X86,
    X8664,
}

struct Fixup {
    pat: String,
    rep: String,
}

fn main() {
    let target = env::var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();

    let machine = match target.get(0) {
        Some(&"x86") => Machine::X86,
        Some(&"x86_64") => Machine::X8664,
        _ => {
            panic!("Unsupported machine {:?}", target);
        }
    };

    match target.get(2) {
        Some(&"windows") => build_win32(machine),
        _ => {
            panic!("Unsupported platform {:?}", target);
        }
    };
}

fn build_win32(machine: Machine) {
    let libs = ["kernel32", "user32", "gdi32", "opengl32"];
    for lib in libs.iter() {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }

    // Note: .clang_arg("--help") is your friend!
    let mut builder = bindgen::builder()
        .no_unstable_rust()
        //.use_core()
        .header("bindings.h")
        .clang_arg("-w")
        .clang_arg("-fms-compatibility")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .clang_arg("-D_UNICODE=1")
        .clang_arg("-DUNICODE=1");

    match machine {
        Machine::X86 => {
            builder = builder.clang_arg("--target=i686-pc-win32").clang_arg("-DWIN32=1");
        }
        Machine::X8664 => {
            builder = builder.clang_arg("--target=x86_64-pc-win32");
        }
    }

    // Note: requires "%VS140COMNTOOLS%..\..\VC\vcvarsall.bat" x86_amd64
    let include = env::var("INCLUDE").expect("Missing %INCLUDE%");
    let include: Vec<_> = include.split(';').collect();
    for inc in include {
        builder = builder.clang_arg("-I")
            .clang_arg(inc);
    }

    builder = builder.hide_type("^__vcrt_.+")
        .hide_type("^__security_.+")
        .hide_type("^__report_.+")
        .hide_type("^__va_start")
        .hide_type("^va_list")
        .hide_type("^.*printf.*")
        .hide_type("^FormatMessage.");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut bindings = builder.generate()
        .expect("Unable to generate bindings")
        .to_string();
    let fixups = vec![Fixup {
                          pat: r###"extern "C" \{"###.into(),
                          rep: r###"extern "system" {"###.into(),
                      },
                      Fixup {
                          pat: r###"#\[link_name = ".*"\](\r\n|\r|\n)"###.into(),
                          rep: String::new(),
                      }];
    for fixup in fixups.iter() {
        bindings = Regex::new(&fixup.pat)
            .unwrap()
            .replace_all(&bindings, fixup.rep.as_str())
            .into_owned()
            .into();
    }

    File::create(out_dir.join("bindings.rs"))
        .and_then(|mut file| file.write_all(&bindings.into_bytes()))
        .expect("Unable to write bindings");
}
