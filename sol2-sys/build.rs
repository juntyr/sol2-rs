use std::{env, path::PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=sol2");

    let lua_root = env::var("DEP_LUA_ROOT")
        .map(PathBuf::from)
        .expect("missing lua dependency");

    // ---------------------------------------------------------
    // Configure sol2
    // ---------------------------------------------------------
    let mut sol2_config = cmake::Config::new("sol2");
    configure_cmake_tools(&mut sol2_config);
    sol2_config.define("SOL2_ENABLE_INSTALL", "ON");
    sol2_config.define("SOL2_BUILD_LUA", "OFF");
    sol2_config.define("SOL2_LUA_VERSION", "5.4");
    sol2_config.define("CMAKE_PREFIX_PATH", lua_root);
    let sol2_out = sol2_config.build();

    println!("cargo::metadata=root={}", sol2_out.display());
    println!(
        "cargo::metadata=include={}",
        sol2_out.join("include").display()
    );
}

fn configure_cmake_tools(config: &mut cmake::Config) {
    if let Ok(ar) = env::var("AR") {
        config.define("CMAKE_AR", ar);
    }
    if let Ok(ld) = env::var("LD") {
        config.define("CMAKE_LINKER", ld);
    }
    if let Ok(nm) = env::var("NM") {
        config.define("CMAKE_NM", nm);
    }
    if let Ok(objdump) = env::var("OBJDUMP") {
        config.define("CMAKE_OBJDUMP", objdump);
    }
    if let Ok(ranlib) = env::var("RANLIB") {
        config.define("CMAKE_RANLIB", ranlib);
    }
    if let Ok(strip) = env::var("STRIP") {
        config.define("CMAKE_STRIP", strip);
    }
}
