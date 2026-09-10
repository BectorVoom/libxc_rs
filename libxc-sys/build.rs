use std::env;
use std::path::PathBuf;

fn main() {
    // `LIBXC_RS_FP_CONTRACT=off` builds the oracle with `-ffp-contract=off`.
    //
    // GCC contracts `a*b + c` into a single FMA by default; rustc does not,
    // and leaves the multiply and the add as written. That is the whole of the
    // residual this tree carries against the oracle on its worst fields
    // (`hyb_gga_xc_wb97x_d3` v2rho2, `gga_x_beefvdw` v2rho2), and it is what
    // `gga_x_wpbeh`'s `vsigma` amplifies as the reduced gradient goes to zero,
    // where the formula is ill-conditioned enough to turn an ulp into a whole
    // number.
    //
    // This is **not** the default and must not become one: it compares against
    // a libxc nobody builds. It exists so the attribution can be *demonstrated*
    // rather than argued -- run the oracle harness with it and the residual
    // moves the way the FMA hypothesis says it should.
    //
    // Changing it changes the cmake build directory's flags, so the whole C
    // library rebuilds (~300 objects).
    println!("cargo:rerun-if-env-changed=LIBXC_RS_FP_CONTRACT");
    let mut cfg = cmake::Config::new("../libxc-master");
    if env::var("LIBXC_RS_FP_CONTRACT").as_deref() == Ok("off") {
        cfg.cflag("-ffp-contract=off");
    }

    // Build vendored libxc 7.0.0 via cmake
    let dst = cfg
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("ENABLE_FORTRAN", "OFF")
        .define("ENABLE_PYTHON", "OFF")
        .define("DISABLE_VXC", "OFF")
        .define("DISABLE_FXC", "OFF")
        .define("DISABLE_KXC", "OFF")
        .define("DISABLE_LXC", "OFF")
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .build();

    // Link against the built static library
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=xc");

    // Also check lib64 (some cmake builds put it there)
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());

    // Generate Rust FFI bindings from xc.h
    let header_path = dst.join("include").join("xc.h");
    let bindings = bindgen::Builder::default()
        .header(header_path.to_string_lossy())
        .allowlist_function("xc_.*")
        .allowlist_type("xc_.*")
        .allowlist_var("XC_.*")
        .derive_default(true)
        .generate()
        .expect("Failed to generate bindings for libxc");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("libxc_bindings.rs"))
        .expect("Failed to write bindings");
}
