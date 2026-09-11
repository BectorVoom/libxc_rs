use std::env;
use std::path::PathBuf;

fn main() {
    // `LIBXC_RS_FP_CONTRACT=off` builds the oracle with `-ffp-contract=off`.
    //
    // GCC contracts `a*b + c` into a single FMA when the target has one;
    // rustc does not. That used to be the residual this tree carried against
    // the oracle (`hyb_gga_xc_wb97x_d3` v2rho2, `gga_x_beefvdw`,
    // `gga_x_wpbeh` vsigma), because the oracle was built `-march=native`.
    // It is now built like PySCF's wheel (`ENABLE_XHOST=OFF`, generic
    // x86-64, no FMA -- see below), so there is nothing left to contract and
    // this switch is a no-op kept for experiments on an XHOST build.
    //
    // Changing it changes the cmake build directory's flags, so the whole C
    // library rebuilds (~300 objects).
    println!("cargo:rerun-if-env-changed=LIBXC_RS_FP_CONTRACT");
    let mut cfg = cmake::Config::new("../libxc-master");
    if env::var("LIBXC_RS_FP_CONTRACT").as_deref() == Ok("off") {
        cfg.cflag("-ffp-contract=off");
    }

    // Build the vendored libxc via cmake.
    //
    // `ENABLE_XHOST=OFF`: libxc's CMake defaults it ON, which adds
    // `-march=native`, and on an FMA-capable host GCC then contracts
    // `a*b + c` into FMAs. The libxc PySCF ships is a generic x86-64 build
    // with no FMA, so an XHOST oracle measures against a library no PySCF
    // user runs. `routed_kernels_*` flagged gga_x_am05, mgga_c_mn12_sx,
    // gga_c_w94, mgga_c_revm06 and gga_k_meyer against it while all five
    // are bit-identical to PySCF's libxc (2026-09-11).
    //
    // `profile("Release")`: cmake-rs otherwise follows the cargo profile, and
    // the test profile configures `CMAKE_BUILD_TYPE=Debug` -- `-g`, no `-O`.
    // An -O0 libxc does not constant-fold libm calls, so every `cbrt(M_PI)`
    // it evaluates at runtime through glibc comes out different from the -O3
    // build PySCF ships (GCC folds those through MPFR, correctly rounded).
    // Release is `-O3 -DNDEBUG`, the wheel's own flags.
    //
    // `BUILD_SHARED_LIBS=ON`, linked as a dylib: a static libxc linked into a
    // Rust binary resolves its runtime `cbrt` calls to compiler_builtins'
    // local copy (`nm` shows `t cbrt` in the test binaries), not glibc's.
    // glibc's cbrt is not correctly rounded and the other one differs from
    // it, so the oracle was a libxc no PySCF user runs. As a shared object,
    // libxc binds `cbrt` through the dynamic linker to libm like the wheel
    // does. The five kernel_oracle outliers plus hyb_gga_xc_hflyp/hse03 in
    // composite_oracle were exactly this: replayed on the same 300-point
    // grid through shared libraries they were bit-identical (2026-09-11).
    let dst = cfg
        .profile("Release")
        .define("ENABLE_XHOST", "OFF")
        .define("BUILD_SHARED_LIBS", "ON")
        .define("BUILD_TESTING", "OFF")
        .define("ENABLE_FORTRAN", "OFF")
        .define("ENABLE_PYTHON", "OFF")
        .define("DISABLE_VXC", "OFF")
        .define("DISABLE_FXC", "OFF")
        .define("DISABLE_KXC", "OFF")
        .define("DISABLE_LXC", "OFF")
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .build();

    // Link against the built shared library (lib64 too: some cmake builds
    // install there). No rpath: `rustc-link-arg` would reach only this
    // package's own targets, and it has none. `cargo test` / `cargo run` put
    // native search paths inside the target dir on the loader path, which is
    // how the verify binaries find it.
    for sub in ["lib", "lib64"] {
        println!("cargo:rustc-link-search=native={}/{sub}", dst.display());
    }
    println!("cargo:rustc-link-lib=dylib=xc");

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
