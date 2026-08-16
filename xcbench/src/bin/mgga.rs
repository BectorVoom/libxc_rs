//! MGGA register-pressure benchmark: `mgga_x_tpss` kxc (3rd derivatives),
//! spin-polarized — a single straight-line kernel with ~5,700 live temporaries
//! and 39 array arguments. This is the worst case the CubeCL path has to carry.
//!
//! Run: cargo run --release --bin mgga [--features hip] -- [npoints] [reps]

use cubecl::prelude::*;
use xcbench::harness::{loadavg, report, run_interleaved, Leg};
use xcbench::mgga_glue::{self as glue, Bufs, Params};

// libxc mgga_x_tpss (id 202) ext_params defaults: b, c, e, kappa, mu, BLOC_a, BLOC_b
const PARAMS: Params = Params {
    param_BLOC_a: 2.0,
    param_BLOC_b: 0.0,
    param_b: 0.40,
    param_c: 1.59096,
    param_e: 1.537,
    param_kappa: 0.8040,
    param_mu: 0.21951,
    dens_threshold: 1.0e-32,
    zeta_threshold: 1.0e-15,
};

const WORKGROUP: u32 = 256;
const RAYON_CHUNK: usize = 2048;

/// Fill the spin-polarized inputs with a physically sensible grid.
fn fill_inputs(b: &mut Bufs) {
    let n = b.n;
    let mut s: u64 = 0x9E3779B97F4A7C15;
    let mut next = || {
        s ^= s >> 12;
        s ^= s << 25;
        s ^= s >> 27;
        (s.wrapping_mul(0x2545F4914F6CDD1D) >> 11) as f64 / (1u64 << 53) as f64
    };
    for ip in 0..n {
        // Two spin channels, each log-uniform over [1e-8, 1e2].
        let ra = 10f64.powf(-8.0 + 10.0 * next());
        let rb = ra * (0.2 + 1.6 * next());
        b.rho[ip * 2] = ra;
        b.rho[ip * 2 + 1] = rb;

        // sigma = (grad_a.grad_a, grad_a.grad_b, grad_b.grad_b), reduced grad in [0,3].
        let ga = {
            let s_red = 3.0 * next();
            let kf = (3.0 * std::f64::consts::PI * std::f64::consts::PI * ra).powf(1.0 / 3.0);
            s_red * 2.0 * kf * ra
        };
        let gb = {
            let s_red = 3.0 * next();
            let kf = (3.0 * std::f64::consts::PI * std::f64::consts::PI * rb).powf(1.0 / 3.0);
            s_red * 2.0 * kf * rb
        };
        let cos = 2.0 * next() - 1.0;
        b.sigma[ip * 3] = ga * ga;
        b.sigma[ip * 3 + 1] = ga * gb * cos;
        b.sigma[ip * 3 + 2] = gb * gb;

        // tau must satisfy tau >= tau_W = sigma/(8 rho); sample above that bound.
        let tw_a = ga * ga / (8.0 * ra);
        let tw_b = gb * gb / (8.0 * rb);
        b.tau[ip * 2] = tw_a * (1.0 + 4.0 * next());
        b.tau[ip * 2 + 1] = tw_b * (1.0 + 4.0 * next());

        // Laplacian: same magnitude scale as tau, either sign.
        b.lapl[ip * 2] = tw_a * (4.0 * next() - 2.0);
        b.lapl[ip * 2 + 1] = tw_b * (4.0 * next() - 2.0);
    }
}

fn zero_outputs(b: &mut Bufs) {
    // Inputs are the first four entries of STRIDES; everything after is output.
    for (name, _) in glue::STRIDES.iter().skip(4) {
        let v = b.output_mut(name);
        v.fill(0.0);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(100_000);
    let reps: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(200);

    let per_point: usize = glue::STRIDES.iter().map(|(_, k)| k).sum();
    println!("=== libxc_rs: mgga_x_tpss kxc POLARIZED (3rd deriv), f64 ===");
    println!("grid points  : {n}");
    println!("arrays       : {} ({} f64 per grid point, {:.2} KB/pt)",
             glue::STRIDES.len(), per_point, per_point as f64 * 8.0 / 1024.0);
    println!("buffer set   : {:.1} MB", (n * per_point * 8) as f64 / 1048576.0);
    println!("reps         : {reps} (interleaved, best-of accepted)");
    println!("rayon threads: {}", rayon::current_num_threads());
    println!("loadavg start: {:.2}", loadavg());

    // ---- reference: one clean serial pass -------------------------------
    let mut refb = Bufs::new(n);
    fill_inputs(&mut refb);
    glue::run_native_serial(&mut refb, &PARAMS);
    let ref_zk = refb.zk.clone();
    let ref_v3tau3 = refb.v3tau3.clone();
    let ref_v3rho3 = refb.v3rho3.clone();

    // ---- native buffers -------------------------------------------------
    let mut nb = Bufs::new(n);
    fill_inputs(&mut nb);
    let mut rb = Bufs::new(n);
    fill_inputs(&mut rb);
    let rptrs = rb.ptrs();

    // ---- cubecl clients + correctness snapshot --------------------------
    let mut hostb = Bufs::new(n);
    fill_inputs(&mut hostb);
    zero_outputs(&mut hostb);

    let cpu_client = cubecl::cpu::CpuRuntime::client(&cubecl::cpu::CpuDevice);
    println!("compiling cubecl-cpu kernel (JIT)...");
    let t_jit = std::time::Instant::now();
    let cpu_dev = glue::upload::<cubecl::cpu::CpuRuntime>(&cpu_client, &hostb);
    glue::launch::<cubecl::cpu::CpuRuntime>(&cpu_client, &cpu_dev, n, &PARAMS, WORKGROUP);
    cubecl::future::block_on(cpu_client.sync()).expect("cpu sync");
    println!("  cubecl-cpu first launch (incl. JIT): {:.2} s", t_jit.elapsed().as_secs_f64());
    let cpu_zk = glue::read::<cubecl::cpu::CpuRuntime>(&cpu_client, &cpu_dev.zk);
    let cpu_v3tau3 = glue::read::<cubecl::cpu::CpuRuntime>(&cpu_client, &cpu_dev.v3tau3);
    let cpu_v3rho3 = glue::read::<cubecl::cpu::CpuRuntime>(&cpu_client, &cpu_dev.v3rho3);

    #[cfg(feature = "hip")]
    let (hip_client, hip_dev, hip_zk, hip_v3tau3, hip_v3rho3) = {
        let c = cubecl_hip::HipRuntime::client(&cubecl_hip::AmdDevice::default());
        println!("compiling cubecl-hip kernel (hiprtc)...");
        let t = std::time::Instant::now();
        let d = glue::upload::<cubecl_hip::HipRuntime>(&c, &hostb);
        glue::launch::<cubecl_hip::HipRuntime>(&c, &d, n, &PARAMS, WORKGROUP);
        cubecl::future::block_on(c.sync()).expect("hip sync");
        println!("  cubecl-hip first launch (incl. hiprtc compile): {:.2} s", t.elapsed().as_secs_f64());
        let zk = glue::read::<cubecl_hip::HipRuntime>(&c, &d.zk);
        let a = glue::read::<cubecl_hip::HipRuntime>(&c, &d.v3tau3);
        let b = glue::read::<cubecl_hip::HipRuntime>(&c, &d.v3rho3);
        (c, d, zk, a, b)
    };

    // ---- legs -----------------------------------------------------------
    let cpu_c = &cpu_client;
    let cpu_d = &cpu_dev;
    let mut legs: Vec<Leg> = vec![
        Leg::new(
            "native serial",
            Box::new(move || glue::run_native_serial(&mut nb, &PARAMS)),
        ),
        Leg::new(
            "native rayon",
            Box::new(move || glue::run_native_rayon(rptrs, &PARAMS, n, RAYON_CHUNK)),
        ),
        Leg::new(
            "cubecl-cpu resident",
            Box::new(move || {
                glue::launch::<cubecl::cpu::CpuRuntime>(cpu_c, cpu_d, n, &PARAMS, WORKGROUP);
                cubecl::future::block_on(cpu_c.sync()).expect("cpu sync");
            }),
        ),
    ];

    #[cfg(feature = "hip")]
    {
        let hip_c = &hip_client;
        let hip_d = &hip_dev;
        legs.push(Leg::new(
            "cubecl-hip resident",
            Box::new(move || {
                glue::launch::<cubecl_hip::HipRuntime>(hip_c, hip_d, n, &PARAMS, WORKGROUP);
                cubecl::future::block_on(hip_c.sync()).expect("hip sync");
            }),
        ));
    }

    run_interleaved(&mut legs, 1, reps);
    println!("loadavg end  : {:.2}", loadavg());
    println!();
    report(&legs, n, "native rayon");

    // ---- numerical agreement -------------------------------------------
    let maxrel = |a: &[f64], b: &[f64]| -> f64 {
        a.iter()
            .zip(b)
            .map(|(x, y)| {
                if x == y {
                    0.0
                } else if *y == 0.0 {
                    (x - y).abs()
                } else {
                    ((x - y) / y).abs()
                }
            })
            .fold(0.0f64, f64::max)
    };
    println!();
    println!("=== numerical agreement vs native serial (max rel diff) ===");
    println!(
        "cubecl-cpu  zk {:.3e}  v3rho3 {:.3e}  v3tau3 {:.3e}   bitwise: {}",
        maxrel(&cpu_zk, &ref_zk),
        maxrel(&cpu_v3rho3, &ref_v3rho3),
        maxrel(&cpu_v3tau3, &ref_v3tau3),
        cpu_zk == ref_zk && cpu_v3rho3 == ref_v3rho3 && cpu_v3tau3 == ref_v3tau3
    );
    #[cfg(feature = "hip")]
    println!(
        "cubecl-hip  zk {:.3e}  v3rho3 {:.3e}  v3tau3 {:.3e}   bitwise: {}",
        maxrel(&hip_zk, &ref_zk),
        maxrel(&hip_v3rho3, &ref_v3rho3),
        maxrel(&hip_v3tau3, &ref_v3tau3),
        hip_zk == ref_zk && hip_v3rho3 == ref_v3rho3 && hip_v3tau3 == ref_v3tau3
    );
}
