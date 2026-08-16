//! CubeCL-vs-native benchmark for libxc_rs XC kernels.
//!
//! Legs measured (all f64, identical FP operation order):
//!   1. native serial       — plain Rust, 1 thread
//!   2. native rayon        — plain Rust, all cores
//!   3. cubecl-cpu resident — buffers already on device, launch + sync only
//!   4. cubecl-cpu full     — alloc + upload + launch + readback (library API today)
//!   5. hip resident        — GPU, buffers already device-resident
//!   6. hip full            — GPU, H2D + launch + D2H
//!
//! Timing is INTERLEAVED: every rep runs each leg once, in rotating order, so
//! thermal drift and background load hit all legs equally. Reported figure is
//! best-of-N per leg, which is the standard robust estimator under noise.
//!
//! Run: cargo run --release [--features hip] -- [npoints] [reps]

use xcbench::native;
use xcbench::harness::{Leg, loadavg, run_interleaved};

use cubecl::client::ComputeClient;
use cubecl::prelude::*;
use cubecl::server::Handle;


use libxc_kernel_gga_x_pbe::vxc_unpol::gga_x_pbe_vxc_unpol;

// libxc gga_x_pbe (id 101) ext_params defaults, and the libxc default thresholds.
const KAPPA: f64 = 0.8040;
const MU: f64 = 0.2195149727645171;
const DENS_THRESHOLD: f64 = 1.0e-32;
const ZETA_THRESHOLD: f64 = 1.0e-15;

const WORKGROUP: u32 = 256;

// ---------------------------------------------------------------------------
// Grid generation — deterministic, spans a realistic DFT density range
// ---------------------------------------------------------------------------

fn make_grid(n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut s: u64 = 0x243F6A8885A308D3;
    let mut next = || {
        s ^= s >> 12;
        s ^= s << 25;
        s ^= s >> 27;
        (s.wrapping_mul(0x2545F4914F6CDD1D) >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut rho = Vec::with_capacity(n);
    let mut sigma = Vec::with_capacity(n);
    for _ in 0..n {
        // rho log-uniform over [1e-8, 1e2]: molecular grids span ~10 decades.
        let r: f64 = 10f64.powf(-8.0 + 10.0 * next());
        // sigma = |grad rho|^2 with reduced gradient s in [0, 3] (physical range).
        let s_red = 3.0 * next();
        let kf = (3.0 * std::f64::consts::PI * std::f64::consts::PI * r).powf(1.0 / 3.0);
        let grad = s_red * 2.0 * kf * r;
        rho.push(r);
        sigma.push(grad * grad);
    }
    (rho, sigma)
}

fn launch_config(n: usize) -> (CubeCount, CubeDim) {
    (
        CubeCount::new_1d((n as u32).div_ceil(WORKGROUP)),
        CubeDim::new_1d(WORKGROUP),
    )
}

// ---------------------------------------------------------------------------
// CubeCL plumbing — generic over runtime so CPU and HIP share one code path
// ---------------------------------------------------------------------------

struct DeviceBufs {
    rho: Handle,
    sigma: Handle,
    zk: Handle,
    vrho: Handle,
    vsigma: Handle,
}

fn upload<R: Runtime>(client: &ComputeClient<R>, rho: &[f64], sigma: &[f64]) -> DeviceBufs {
    let n = rho.len();
    let zeros = vec![0f64; n];
    DeviceBufs {
        rho: client.create_from_slice(bytemuck::cast_slice(rho)),
        sigma: client.create_from_slice(bytemuck::cast_slice(sigma)),
        zk: client.create_from_slice(bytemuck::cast_slice(&zeros)),
        vrho: client.create_from_slice(bytemuck::cast_slice(&zeros)),
        vsigma: client.create_from_slice(bytemuck::cast_slice(&zeros)),
    }
}

fn launch<R: Runtime>(client: &ComputeClient<R>, b: &DeviceBufs, n: usize) {
    let (count, dim) = launch_config(n);
    unsafe {
        gga_x_pbe_vxc_unpol::launch_unchecked::<R>(
            client,
            count,
            dim,
            ArrayArg::from_raw_parts(b.rho.clone(), n),
            ArrayArg::from_raw_parts(b.sigma.clone(), n),
            ArrayArg::from_raw_parts(b.zk.clone(), n),
            ArrayArg::from_raw_parts(b.vrho.clone(), n),
            ArrayArg::from_raw_parts(b.vsigma.clone(), n),
            KAPPA,
            MU,
            DENS_THRESHOLD,
            ZETA_THRESHOLD,
        );
    }
}

fn sync<R: Runtime>(client: &ComputeClient<R>) {
    cubecl::future::block_on(client.sync()).expect("device sync failed");
}

fn readback<R: Runtime>(client: &ComputeClient<R>, h: &Handle) -> Vec<f64> {
    let bytes = client.read_one(h.clone()).expect("readback failed");
    bytemuck::cast_slice(&bytes).to_vec()
}

/// One clean launch, returning (zk, vrho, vsigma) for correctness checking.
fn snapshot<R: Runtime>(
    client: &ComputeClient<R>,
    rho: &[f64],
    sigma: &[f64],
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = rho.len();
    let b = upload::<R>(client, rho, sigma);
    launch::<R>(client, &b, n);
    sync::<R>(client);
    (
        readback::<R>(client, &b.zk),
        readback::<R>(client, &b.vrho),
        readback::<R>(client, &b.vsigma),
    )
}

fn max_rel_diff(a: &[f64], b: &[f64]) -> f64 {
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
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1_000_000);
    let reps: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(15);
    let chunk: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(2048);

    println!("=== libxc_rs: gga_x_pbe vxc unpolarized, f64 ===");
    println!("grid points  : {n}");
    println!("reps         : {reps} (interleaved, best-of reported)");
    println!("rayon threads: {}", rayon::current_num_threads());
    println!("rayon chunk  : {chunk}");
    println!("loadavg start: {:.2}", loadavg());

    let (rho, sigma) = make_grid(n);

    // Reference: single clean serial pass.
    let mut ref_zk = vec![0f64; n];
    let mut ref_vrho = vec![0f64; n];
    let mut ref_vsigma = vec![0f64; n];
    native::pbe_vxc_unpol_serial_libm(
        &rho, &sigma, &mut ref_zk, &mut ref_vrho, &mut ref_vsigma, KAPPA, MU, DENS_THRESHOLD,
        ZETA_THRESHOLD,
    );

    // Clients + warm JIT + correctness snapshots (outside the timed section).
    let cpu_client = cubecl::cpu::CpuRuntime::client(&cubecl::cpu::CpuDevice);
    let (cpu_zk, cpu_vrho, cpu_vsigma) =
        snapshot::<cubecl::cpu::CpuRuntime>(&cpu_client, &rho, &sigma);
    let cpu_bufs = upload::<cubecl::cpu::CpuRuntime>(&cpu_client, &rho, &sigma);

    #[cfg(feature = "hip")]
    let hip_client = cubecl_hip::HipRuntime::client(&cubecl_hip::AmdDevice::default());
    #[cfg(feature = "hip")]
    let (hip_zk, hip_vrho, hip_vsigma) =
        snapshot::<cubecl_hip::HipRuntime>(&hip_client, &rho, &sigma);
    #[cfg(feature = "hip")]
    let hip_bufs = upload::<cubecl_hip::HipRuntime>(&hip_client, &rho, &sigma);

    // Per-leg output buffers so each closure can hold its own &mut.
    let mut s_zk = vec![0f64; n];
    let mut s_vrho = vec![0f64; n];
    let mut s_vsigma = vec![0f64; n];
    let mut r_zk = vec![0f64; n];
    let mut r_vrho = vec![0f64; n];
    let mut r_vsigma = vec![0f64; n];

    let (rho_r, sigma_r) = (&rho, &sigma);
    let cpu_c = &cpu_client;
    let cpu_b = &cpu_bufs;

    let mut legs: Vec<Leg> = vec![
        Leg::new(
            "native serial",
            Box::new(move || {
                native::pbe_vxc_unpol_serial(
                    rho_r,
                    sigma_r,
                    &mut s_zk,
                    &mut s_vrho,
                    &mut s_vsigma,
                    KAPPA,
                    MU,
                    DENS_THRESHOLD,
                    ZETA_THRESHOLD,
                )
            }),
        ),
        Leg::new(
            "native rayon",
            Box::new(move || {
                native::pbe_vxc_unpol_rayon(
                    rho_r,
                    sigma_r,
                    &mut r_zk,
                    &mut r_vrho,
                    &mut r_vsigma,
                    KAPPA,
                    MU,
                    DENS_THRESHOLD,
                    ZETA_THRESHOLD,
                    chunk,
                )
            }),
        ),
        Leg::new(
            "cubecl-cpu resident",
            Box::new(move || {
                launch::<cubecl::cpu::CpuRuntime>(cpu_c, cpu_b, n);
                sync::<cubecl::cpu::CpuRuntime>(cpu_c);
            }),
        ),
        Leg::new(
            "cubecl-cpu full call",
            Box::new(move || {
                let b = upload::<cubecl::cpu::CpuRuntime>(cpu_c, rho_r, sigma_r);
                launch::<cubecl::cpu::CpuRuntime>(cpu_c, &b, n);
                std::hint::black_box(readback::<cubecl::cpu::CpuRuntime>(cpu_c, &b.zk));
                std::hint::black_box(readback::<cubecl::cpu::CpuRuntime>(cpu_c, &b.vrho));
                std::hint::black_box(readback::<cubecl::cpu::CpuRuntime>(cpu_c, &b.vsigma));
            }),
        ),
    ];

    #[cfg(feature = "hip")]
    {
        let hip_c = &hip_client;
        let hip_b = &hip_bufs;
        legs.push(Leg::new(
            "cubecl-hip resident",
            Box::new(move || {
                launch::<cubecl_hip::HipRuntime>(hip_c, hip_b, n);
                sync::<cubecl_hip::HipRuntime>(hip_c);
            }),
        ));
        legs.push(Leg::new(
            "cubecl-hip full call",
            Box::new(move || {
                let b = upload::<cubecl_hip::HipRuntime>(hip_c, rho_r, sigma_r);
                launch::<cubecl_hip::HipRuntime>(hip_c, &b, n);
                std::hint::black_box(readback::<cubecl_hip::HipRuntime>(hip_c, &b.zk));
                std::hint::black_box(readback::<cubecl_hip::HipRuntime>(hip_c, &b.vrho));
                std::hint::black_box(readback::<cubecl_hip::HipRuntime>(hip_c, &b.vsigma));
            }),
        ));
    }

    run_interleaved(&mut legs, 2, reps);

    println!("loadavg end  : {:.2}", loadavg());
    println!();

    let base = legs
        .iter()
        .find(|l| l.name == "native rayon")
        .map(|l| l.best)
        .unwrap();

    println!(
        "{:<24} {:>10} {:>11} {:>13} {:>10} {:>9} {:>10}",
        "leg", "best", "per point", "throughput", "mean", "vs rayon", "reps ok/rej"
    );
    println!("{}", "-".repeat(95));
    for l in &legs {
        println!(
            "{:<24} {:>7.2} ms {:>8.2} ns/pt {:>8.1} Mpts/s {:>7.2} ms {:>8.2}x {:>5}/{:<5}",
            l.name,
            l.best * 1e3,
            l.best * 1e9 / n as f64,
            n as f64 / l.best / 1e6,
            l.total / l.count.max(1) as f64 * 1e3,
            base / l.best,
            l.count,
            l.rejected,
        );
    }
    println!();
    println!("--- unfiltered fallback (best over ALL reps, incl. contended) ---");
    for l in &legs {
        println!(
            "{:<24} {:>7.2} ms {:>8.2} ns/pt   (min foreign-CPU seen: {:.2} cores)",
            l.name,
            l.best_any * 1e3,
            l.best_any * 1e9 / n as f64,
            l.min_contention,
        );
    }

    // The perf legs use the kernel's arithmetic cbrt; confirm it still agrees
    // with the libm reference after the floor-vs-truncate exponent rewrite.
    let mut f_zk = vec![0f64; n];
    let mut f_vrho = vec![0f64; n];
    let mut f_vsigma = vec![0f64; n];
    native::pbe_vxc_unpol_serial(
        &rho, &sigma, &mut f_zk, &mut f_vrho, &mut f_vsigma, KAPPA, MU, DENS_THRESHOLD,
        ZETA_THRESHOLD,
    );
    println!();
    println!("=== numerical agreement vs native serial (max rel diff) ===");
    println!(
        "native-fast zk {:.3e}  vrho {:.3e}  vsigma {:.3e}",
        max_rel_diff(&f_zk, &ref_zk),
        max_rel_diff(&f_vrho, &ref_vrho),
        max_rel_diff(&f_vsigma, &ref_vsigma)
    );
    println!(
        "cubecl-cpu  zk {:.3e}  vrho {:.3e}  vsigma {:.3e}   bitwise: {}",
        max_rel_diff(&cpu_zk, &ref_zk),
        max_rel_diff(&cpu_vrho, &ref_vrho),
        max_rel_diff(&cpu_vsigma, &ref_vsigma),
        cpu_zk == ref_zk && cpu_vrho == ref_vrho && cpu_vsigma == ref_vsigma
    );
    #[cfg(feature = "hip")]
    println!(
        "cubecl-hip  zk {:.3e}  vrho {:.3e}  vsigma {:.3e}   bitwise: {}",
        max_rel_diff(&hip_zk, &ref_zk),
        max_rel_diff(&hip_vrho, &ref_vrho),
        max_rel_diff(&hip_vsigma, &ref_vsigma),
        hip_zk == ref_zk && hip_vrho == ref_vrho && hip_vsigma == ref_vsigma
    );
}
