//! Phase breakdown of the library's full-call buffer path.
//!
//! Replicates exactly what `libxc_eval::eval::gga_dispatch::dispatch_gga` does
//! per evaluation, timing each phase separately so the overhead can be
//! attributed instead of guessed at.

use cubecl::prelude::*;
use std::time::Instant;

use libxc_kernel_gga_x_pbe::vxc_unpol::gga_x_pbe_vxc_unpol;

const KAPPA: f64 = 0.8040;
const MU: f64 = 0.2195149727645171;
const DENS_THRESHOLD: f64 = 1.0e-32;
const ZETA_THRESHOLD: f64 = 1.0e-15;
const WORKGROUP: u32 = 256;

/// Mirrors the new `libxc_eval::kernel::launch::zero_fill_f64`.
#[cube(launch_unchecked)]
fn zero_fill_f64(out: &mut Array<f64>) {
    let i = ABSOLUTE_POS;
    if i < out.len() {
        out[i] = 0.0;
    }
}

/// New path: uninitialised device allocation zeroed by a device-side kernel,
/// with no host allocation and no host->device transfer.
fn zero_buffer_on_device(
    client: &cubecl::client::ComputeClient<cubecl::cpu::CpuRuntime>,
    n: usize,
) -> cubecl::server::Handle {
    let h = client.empty(n * core::mem::size_of::<f64>());
    let count = CubeCount::new_1d((n as u32).div_ceil(WORKGROUP));
    let dim = CubeDim::new_1d(WORKGROUP);
    unsafe {
        zero_fill_f64::launch_unchecked::<cubecl::cpu::CpuRuntime>(
            client,
            count,
            dim,
            ArrayArg::from_raw_parts(h.clone(), n),
        );
    }
    h
}

fn best(reps: usize, mut f: impl FnMut() -> f64) -> f64 {
    let mut b = f64::INFINITY;
    for _ in 0..reps {
        let t = f();
        if t < b {
            b = t;
        }
    }
    b
}

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1_000_000);
    let reps: usize = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);

    let rho: Vec<f64> = (0..n).map(|i| 1e-3 + (i % 1000) as f64 * 1e-3).collect();
    let sigma: Vec<f64> = rho.iter().map(|r| r * r * 0.7).collect();

    // Caller-provided output slices, as the public API hands them in.
    let mut out_zk = vec![0f64; n];
    let mut out_vrho = vec![0f64; n];
    let mut out_vsigma = vec![0f64; n];

    // Warm the JIT so kernel compilation is not attributed to a phase.
    {
        let c = cubecl::cpu::CpuRuntime::client(&cubecl::cpu::CpuDevice);
        let r = c.create_from_slice(bytemuck::cast_slice(&rho));
        let s = c.create_from_slice(bytemuck::cast_slice(&sigma));
        let z = c.create_from_slice(bytemuck::cast_slice(&vec![0f64; n]));
        let a = c.create_from_slice(bytemuck::cast_slice(&vec![0f64; n]));
        let b = c.create_from_slice(bytemuck::cast_slice(&vec![0f64; n]));
        let (count, dim) = (
            CubeCount::new_1d((n as u32).div_ceil(WORKGROUP)),
            CubeDim::new_1d(WORKGROUP),
        );
        unsafe {
            gga_x_pbe_vxc_unpol::launch_unchecked::<cubecl::cpu::CpuRuntime>(
                &c, count, dim,
                ArrayArg::from_raw_parts(r, n), ArrayArg::from_raw_parts(s, n),
                ArrayArg::from_raw_parts(z, n), ArrayArg::from_raw_parts(a, n),
                ArrayArg::from_raw_parts(b, n),
                KAPPA, MU, DENS_THRESHOLD, ZETA_THRESHOLD,
            );
        }
        cubecl::future::block_on(c.sync()).unwrap();
    }

    println!("=== full-call phase breakdown, gga_x_pbe vxc unpol, n = {n} ===");
    println!("(best of {reps}; 3 outputs: zk, vrho, vsigma)\n");

    // -- phase 1: client acquisition -------------------------------------
    let t_client = best(reps, || {
        let t = Instant::now();
        let c = cubecl::cpu::CpuRuntime::client(&cubecl::cpu::CpuDevice);
        let e = t.elapsed().as_secs_f64();
        std::hint::black_box(&c);
        e
    });

    let client = cubecl::cpu::CpuRuntime::client(&cubecl::cpu::CpuDevice);

    // -- phase 2: zeroing the caller's output slices ----------------------
    let t_fill = best(reps, || {
        let t = Instant::now();
        out_zk.fill(0.0);
        out_vrho.fill(0.0);
        out_vsigma.fill(0.0);
        t.elapsed().as_secs_f64()
    });

    // -- phase 3: uploading inputs ---------------------------------------
    let t_upload_in = best(reps, || {
        let t = Instant::now();
        let a = client.create_from_slice(bytemuck::cast_slice(&rho));
        let b = client.create_from_slice(bytemuck::cast_slice(&sigma));
        let e = t.elapsed().as_secs_f64();
        std::hint::black_box((a, b));
        e
    });

    // -- phase 4a: allocating the host zero vectors -----------------------
    let t_zero_alloc = best(reps, || {
        let t = Instant::now();
        let a = vec![0f64; n];
        let b = vec![0f64; n];
        let c = vec![0f64; n];
        let e = t.elapsed().as_secs_f64();
        std::hint::black_box((&a, &b, &c));
        e
    });

    // -- phase 4b: uploading those zeros to device ------------------------
    let zeros = vec![0f64; n];
    let t_zero_upload = best(reps, || {
        let t = Instant::now();
        let a = client.create_from_slice(bytemuck::cast_slice(&zeros));
        let b = client.create_from_slice(bytemuck::cast_slice(&zeros));
        let c = client.create_from_slice(bytemuck::cast_slice(&zeros));
        let e = t.elapsed().as_secs_f64();
        std::hint::black_box((a, b, c));
        e
    });

    // -- phase 4c: NEW -- zero the same 3 buffers on device ---------------
    // Warm the zero kernel's JIT first so compilation is not timed.
    {
        let h = zero_buffer_on_device(&client, n);
        cubecl::future::block_on(client.sync()).unwrap();
        std::hint::black_box(h);
    }
    let t_zero_device = best(reps, || {
        let t = Instant::now();
        let a = zero_buffer_on_device(&client, n);
        let b = zero_buffer_on_device(&client, n);
        let c = zero_buffer_on_device(&client, n);
        cubecl::future::block_on(client.sync()).unwrap();
        let e = t.elapsed().as_secs_f64();
        std::hint::black_box((a, b, c));
        e
    });

    // -- phase 6d: NEW -- read_one straight into the caller's slice -------
    // (i.e. what `read_output_buffer_into` does: no intermediate Vec.)

    // -- phase 5: launch + sync ------------------------------------------
    let rh = client.create_from_slice(bytemuck::cast_slice(&rho));
    let sh = client.create_from_slice(bytemuck::cast_slice(&sigma));
    let zh = client.create_from_slice(bytemuck::cast_slice(&zeros));
    let ah = client.create_from_slice(bytemuck::cast_slice(&zeros));
    let bh = client.create_from_slice(bytemuck::cast_slice(&zeros));
    let t_launch = best(reps, || {
        let t = Instant::now();
        let (count, dim) = (
            CubeCount::new_1d((n as u32).div_ceil(WORKGROUP)),
            CubeDim::new_1d(WORKGROUP),
        );
        unsafe {
            gga_x_pbe_vxc_unpol::launch_unchecked::<cubecl::cpu::CpuRuntime>(
                &client, count, dim,
                ArrayArg::from_raw_parts(rh.clone(), n),
                ArrayArg::from_raw_parts(sh.clone(), n),
                ArrayArg::from_raw_parts(zh.clone(), n),
                ArrayArg::from_raw_parts(ah.clone(), n),
                ArrayArg::from_raw_parts(bh.clone(), n),
                KAPPA, MU, DENS_THRESHOLD, ZETA_THRESHOLD,
            );
        }
        cubecl::future::block_on(client.sync()).unwrap();
        t.elapsed().as_secs_f64()
    });

    // -- phase 6a: read_one (device -> host Bytes) ------------------------
    let t_read = best(reps, || {
        let t = Instant::now();
        let a = client.read_one(zh.clone()).unwrap();
        let b = client.read_one(ah.clone()).unwrap();
        let c = client.read_one(bh.clone()).unwrap();
        let e = t.elapsed().as_secs_f64();
        std::hint::black_box((&a, &b, &c));
        e
    });

    // -- phase 6b: the .to_vec() the helper adds on top -------------------
    let bytes_a = client.read_one(zh.clone()).unwrap();
    let t_tovec = best(reps, || {
        let t = Instant::now();
        let v: Vec<f64> = bytemuck::cast_slice(&bytes_a).to_vec();
        let e = t.elapsed().as_secs_f64();
        std::hint::black_box(&v);
        e
    });
    let t_tovec3 = t_tovec * 3.0;

    // -- phase 6c: copy_from_slice into the caller's buffer ---------------
    let src: Vec<f64> = bytemuck::cast_slice(&bytes_a).to_vec();
    let t_copy = best(reps, || {
        let t = Instant::now();
        out_zk.copy_from_slice(&src);
        out_vrho.copy_from_slice(&src);
        out_vsigma.copy_from_slice(&src);
        t.elapsed().as_secs_f64()
    });

    let ms = |x: f64| x * 1e3;
    let total = t_client + t_fill + t_upload_in + t_zero_alloc + t_zero_upload
        + t_launch + t_read + t_tovec3 + t_copy;

    println!("{:<40} {:>9} {:>8}", "phase", "ms", "% total");
    println!("{}", "-".repeat(60));
    let row = |name: &str, v: f64| {
        println!("{:<40} {:>9.3} {:>7.1}%", name, ms(v), 100.0 * v / total)
    };
    row("1  client acquisition", t_client);
    row("2  fill(0.0) caller output slices", t_fill);
    row("3  upload rho + sigma", t_upload_in);
    row("4a alloc host zero vecs (x3)", t_zero_alloc);
    row("4b upload zeros to device (x3)", t_zero_upload);
    row("5  launch + sync  [THE ACTUAL WORK]", t_launch);
    row("6a read_one device->host (x3)", t_read);
    row("6b .to_vec() in read_output_buffer (x3)", t_tovec3);
    row("6c copy_from_slice to caller (x3)", t_copy);
    println!("{}", "-".repeat(60));
    println!("{:<40} {:>9.3}", "TOTAL", ms(total));
    println!("{:<40} {:>9.3}", "overhead (total - launch)", ms(total - t_launch));
    // ---- end-to-end A/B -------------------------------------------------
    // Phase timings above mis-charge the device-zeroing path, because timing it
    // in isolation forces a `sync()` that the real flow never pays: the zero
    // kernels are simply enqueued ahead of the main kernel on the same stream.
    // The only fair comparison is a complete call, measured end to end.
    let launch_main = |rh: &cubecl::server::Handle,
                       sh: &cubecl::server::Handle,
                       zh: &cubecl::server::Handle,
                       ah: &cubecl::server::Handle,
                       bh: &cubecl::server::Handle| {
        let count = CubeCount::new_1d((n as u32).div_ceil(WORKGROUP));
        let dim = CubeDim::new_1d(WORKGROUP);
        unsafe {
            gga_x_pbe_vxc_unpol::launch_unchecked::<cubecl::cpu::CpuRuntime>(
                &client, count, dim,
                ArrayArg::from_raw_parts(rh.clone(), n),
                ArrayArg::from_raw_parts(sh.clone(), n),
                ArrayArg::from_raw_parts(zh.clone(), n),
                ArrayArg::from_raw_parts(ah.clone(), n),
                ArrayArg::from_raw_parts(bh.clone(), n),
                KAPPA, MU, DENS_THRESHOLD, ZETA_THRESHOLD,
            );
        }
    };

    // OLD: host zero vecs uploaded, read back through an intermediate Vec.
    let t_old = best(reps, || {
        let t = Instant::now();
        out_zk.fill(0.0); out_vrho.fill(0.0); out_vsigma.fill(0.0);
        let rh = client.create_from_slice(bytemuck::cast_slice(&rho));
        let sh = client.create_from_slice(bytemuck::cast_slice(&sigma));
        let z0 = vec![0f64; n];
        let z1 = vec![0f64; n];
        let z2 = vec![0f64; n];
        let zh = client.create_from_slice(bytemuck::cast_slice(&z0));
        let ah = client.create_from_slice(bytemuck::cast_slice(&z1));
        let bh = client.create_from_slice(bytemuck::cast_slice(&z2));
        launch_main(&rh, &sh, &zh, &ah, &bh);
        for (h, dst) in [(&zh, &mut out_zk), (&ah, &mut out_vrho), (&bh, &mut out_vsigma)] {
            let bytes = client.read_one(h.clone()).unwrap();
            let v: Vec<f64> = bytemuck::cast_slice(&bytes).to_vec();
            dst.copy_from_slice(&v);
        }
        t.elapsed().as_secs_f64()
    });

    // NEW: device-side zeroing, read back straight into the caller's slice.
    let t_new = best(reps, || {
        let t = Instant::now();
        out_zk.fill(0.0); out_vrho.fill(0.0); out_vsigma.fill(0.0);
        let rh = client.create_from_slice(bytemuck::cast_slice(&rho));
        let sh = client.create_from_slice(bytemuck::cast_slice(&sigma));
        let zh = zero_buffer_on_device(&client, n);
        let ah = zero_buffer_on_device(&client, n);
        let bh = zero_buffer_on_device(&client, n);
        launch_main(&rh, &sh, &zh, &ah, &bh);
        for (h, dst) in [(&zh, &mut out_zk), (&ah, &mut out_vrho), (&bh, &mut out_vsigma)] {
            let bytes = client.read_one(h.clone()).unwrap();
            let src: &[f64] = bytemuck::cast_slice(&bytes);
            dst.copy_from_slice(src);
        }
        t.elapsed().as_secs_f64()
    });

    // NEW-B: keep host-zero upload, change only the read-back.
    let t_readonly = best(reps, || {
        let t = Instant::now();
        out_zk.fill(0.0); out_vrho.fill(0.0); out_vsigma.fill(0.0);
        let rh = client.create_from_slice(bytemuck::cast_slice(&rho));
        let sh = client.create_from_slice(bytemuck::cast_slice(&sigma));
        let z0 = vec![0f64; n];
        let z1 = vec![0f64; n];
        let z2 = vec![0f64; n];
        let zh = client.create_from_slice(bytemuck::cast_slice(&z0));
        let ah = client.create_from_slice(bytemuck::cast_slice(&z1));
        let bh = client.create_from_slice(bytemuck::cast_slice(&z2));
        launch_main(&rh, &sh, &zh, &ah, &bh);
        for (h, dst) in [(&zh, &mut out_zk), (&ah, &mut out_vrho), (&bh, &mut out_vsigma)] {
            let bytes = client.read_one(h.clone()).unwrap();
            let src: &[f64] = bytemuck::cast_slice(&bytes);
            dst.copy_from_slice(src);
        }
        t.elapsed().as_secs_f64()
    });

    // ---- CORRECTNESS: the new path must be bit-identical to the old -----
    // `client.empty()` hands back uninitialised memory, so if the zero-fill
    // kernel did not run (or ran wrong) the `+=` accumulating kernels would
    // silently produce garbage. Compare full outputs, not a spot check.
    let run_old = |zk: &mut [f64], vrho: &mut [f64], vsigma: &mut [f64]| {
        let rh = client.create_from_slice(bytemuck::cast_slice(&rho));
        let sh = client.create_from_slice(bytemuck::cast_slice(&sigma));
        let z = vec![0f64; n];
        let zh = client.create_from_slice(bytemuck::cast_slice(&z));
        let ah = client.create_from_slice(bytemuck::cast_slice(&z));
        let bh = client.create_from_slice(bytemuck::cast_slice(&z));
        launch_main(&rh, &sh, &zh, &ah, &bh);
        for (h, dst) in [(&zh, &mut *zk), (&ah, &mut *vrho), (&bh, &mut *vsigma)] {
            let bytes = client.read_one(h.clone()).unwrap();
            dst.copy_from_slice(bytemuck::cast_slice(&bytes));
        }
    };
    let run_new = |zk: &mut [f64], vrho: &mut [f64], vsigma: &mut [f64]| {
        let rh = client.create_from_slice(bytemuck::cast_slice(&rho));
        let sh = client.create_from_slice(bytemuck::cast_slice(&sigma));
        let zh = zero_buffer_on_device(&client, n);
        let ah = zero_buffer_on_device(&client, n);
        let bh = zero_buffer_on_device(&client, n);
        launch_main(&rh, &sh, &zh, &ah, &bh);
        for (h, dst) in [(&zh, &mut *zk), (&ah, &mut *vrho), (&bh, &mut *vsigma)] {
            let bytes = client.read_one(h.clone()).unwrap();
            dst.copy_from_slice(bytemuck::cast_slice(&bytes));
        }
    };
    let (mut oz, mut ov, mut os) = (vec![0f64; n], vec![0f64; n], vec![0f64; n]);
    let (mut nz, mut nv, mut ns) = (vec![0f64; n], vec![0f64; n], vec![0f64; n]);
    run_old(&mut oz, &mut ov, &mut os);
    // Run the new path several times: a pooled allocator will hand back dirty
    // memory on reuse, so a single clean run could pass by luck.
    for _ in 0..5 {
        run_new(&mut nz, &mut nv, &mut ns);
    }
    let ident = oz == nz && ov == nv && os == ns;
    println!();
    println!("=== CORRECTNESS: device-zeroed path vs host-zero-upload path ===");
    println!("  bit-identical over all {n} points x 3 fields: {ident}");
    if !ident {
        let bad = oz.iter().zip(&nz).filter(|(a, b)| a != b).count()
            + ov.iter().zip(&nv).filter(|(a, b)| a != b).count()
            + os.iter().zip(&ns).filter(|(a, b)| a != b).count();
        println!("  MISMATCH in {bad} values -- device zeroing is NOT sound here");
        std::process::exit(1);
    }

    println!();
    println!("=== END-TO-END A/B (complete call, best of {reps}) ===");
    println!("  A  old: host-zero upload    + read via Vec   {:>8.3} ms   1.00x", ms(t_old));
    println!("  B  new: device-side zeroing + read direct    {:>8.3} ms  {:>5.2}x", ms(t_new), t_old / t_new);
    println!("  C  old zeroing              + read direct    {:>8.3} ms  {:>5.2}x", ms(t_readonly), t_old / t_readonly);

    println!();
    println!("=== isolated phase view (mis-charges B, see note above) ===");
    println!(
        "output zeroing:  host alloc + upload {:.3} ms  ->  zero on device {:.3} ms   ({:.2}x)",
        ms(t_zero_alloc + t_zero_upload),
        ms(t_zero_device),
        (t_zero_alloc + t_zero_upload) / t_zero_device
    );
    println!(
        "read-back:       read_one + to_vec + copy {:.3} ms  ->  read_one + copy {:.3} ms   ({:.2}x)",
        ms(t_read + t_tovec3 + t_copy),
        ms(t_read + t_copy),
        (t_read + t_tovec3 + t_copy) / (t_read + t_copy)
    );

    let saved = (t_zero_alloc + t_zero_upload - t_zero_device) + t_tovec3;
    let new_overhead = (total - t_launch) - saved;
    println!();
    println!(
        "per-call overhead: {:.3} ms -> {:.3} ms   (saved {:.3} ms, {:.0}%)",
        ms(total - t_launch),
        ms(new_overhead),
        ms(saved),
        100.0 * saved / (total - t_launch)
    );
    println!(
        "full call total:   {:.3} ms -> {:.3} ms   ({:.2}x)",
        ms(total),
        ms(total - saved),
        total / (total - saved)
    );
    println!();
    println!("Not removed: the up-front fill(0.0) of caller slices ({:.3} ms).", ms(t_fill));
    println!("  GGA dispatch has ~105 `?` early-exit points between that fill and");
    println!("  read-back, so on a dispatch error the caller's buffers must still");
    println!("  read as zero. Kept deliberately.");
}
