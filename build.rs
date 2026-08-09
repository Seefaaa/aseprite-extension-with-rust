use std::{env, fs, path::PathBuf};

const EXCLUDED: [&str; 3] = ["lua.c", "luac.c", "onelua.c"];

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let lua_src = manifest_dir.join("lua-src");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let sources: Vec<PathBuf> = fs::read_dir(&lua_src)
        .expect("failed to read lua-src directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension().is_some_and(|ext| ext == "c")
                && !EXCLUDED.contains(&path.file_name().unwrap().to_str().unwrap())
        })
        .collect();

    let mut build = cc::Build::new();
    build
        .include(&lua_src)
        .opt_level(2)
        .warnings(false)
        .files(&sources);

    match target_os.as_str() {
        "linux" => {
            build.define("LUA_USE_LINUX", None);
        }
        "macos" => {
            build.define("LUA_USE_MACOSX", None);
        }
        // luaconf.h enables LUA_USE_WINDOWS on its own via `_WIN32`.
        _ => {}
    }

    build.compile("lua54");

    println!(
        "cargo:rerun-if-changed={}",
        lua_src.join("luaconf.h").display()
    );

    for source in &sources {
        println!("cargo:rerun-if-changed={}", source.display());
    }
}
