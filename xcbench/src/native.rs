//! Plain-Rust transliteration of `gga_x_pbe_vxc_unpol`.
//!
//! Same maple2c variable names and the same floating-point operation order as
//! `crates/kernels/gga/gga_x_pbe/src/vxc_unpol.rs`, with `select()` replaced by
//! an eager branchless `sel()` so both arms are still evaluated.
//!
//! Two cube roots live here on purpose:
//!
//! * [`cbrt_fast`] is a port of the kernel's `cbrt_f64` — pure arithmetic, no
//!   libm call, so a loop containing it can be vectorised. The performance legs
//!   use this, which makes them algorithmically identical to the CubeCL kernel.
//! * [`cbrt_libm`] calls the system `cbrt`, i.e. exactly what libxc's
//!   `POW_1_3` resolves to. The oracle-faithful reference leg uses this.
//!
//! The loops reslice every buffer to a common length up front so LLVM can prove
//! the indices in bounds and drop the checks; one surviving bounds check blocks
//! vectorisation of the whole loop.

pub const M_PI: f64 = std::f64::consts::PI;
pub const M_CBRTPI: f64 = 1.4645918875615232;
pub const M_CBRT2: f64 = 1.2599210498948732;
pub const M_CBRT3: f64 = 1.4422495703074084;
pub const M_CBRT6: f64 = 1.8171205928321397;

const CBRT_F_P1: f64 = 1.259921049894873164767; // 2^( 1/3)
const CBRT_F_P2: f64 = 1.587401051968199474752; // 2^( 2/3)

#[inline(always)]
pub fn sel(c: bool, a: f64, b: f64) -> f64 {
    if c { a } else { b }
}

/// Port of `libxc_kernel_math::powers::cbrt_f64` — branch-free, no libm call.
///
/// All integer work stays in 64-bit lanes so the vectoriser never has to pack
/// between 32- and 64-bit widths, and `xe / 3` / `xe % 3` are done with a magic
/// multiply rather than a real integer division (x86 has no vectorised 64-bit
/// divide, and its multiply-high has no AVX-512 form either).
///
/// The split uses *floor* division: `xe == 3*n + r` with `r` in {0,1,2}, so the
/// rescale table shrinks to {1, 2^(1/3), 2^(2/3)}. This is bit-identical to the
/// truncating split, because moving one power of three between `r` and `n`
/// multiplies the factor by an exact power of two.
#[inline(always)]
pub fn cbrt_fast(x: f64) -> f64 {
    let a = x.abs();
    let bits = a.to_bits();

    let raw = (bits >> 52) & 0x7ff;
    let is_sub = raw == 0;
    let scaled = a * 18014398509481984.0; // 2^54
    let bits_u = if is_sub { scaled.to_bits() } else { bits };
    let raw_u = ((bits_u >> 52) & 0x7ff) as i64 + if is_sub { -54i64 } else { 0 };
    let xm = f64::from_bits((bits_u & 0x800f_ffff_ffff_ffff) | (1022u64 << 52));
    let xe = raw_u - 1022;

    let u = 0.354895765043919860
        + (1.50819193781584896
            + (-2.11499494167371287
                + (2.44693122563534430
                    + (-1.83469277483613086
                        + (0.784932344976639262 - 0.145263899385486377 * xm) * xm)
                        * xm)
                    * xm)
                * xm)
            * xm;

    let t2 = u * u * u;

    // Bias by a multiple of 3 so the quotient is non-negative, then
    // floor(e2/3) == (e2 * 21846) >> 16 exactly for e2 well under 32768.
    let e2 = xe + 1536; // xe in [-1074, 1024] -> e2 in [462, 2560]
    let nf = (e2 * 21846) >> 16;
    let r = e2 - 3 * nf; // 0, 1 or 2
    let n = nf - 512; // undo the 1536/3 bias

    let fac = sel(r == 0, 1.0, sel(r == 1, CBRT_F_P1, CBRT_F_P2));
    let ym = u * (t2 + 2.0 * xm) / (2.0 * t2 + xm) * fac;

    let pow2 = f64::from_bits(((n + 1023) as u64) << 52);
    let y0 = ym * pow2;

    let t = y0 * y0;
    let err = f64::mul_add(t, y0, -a);
    let y = y0 - err / (3.0 * t);

    let signed = sel(x < 0.0, -y, y);
    let degenerate = (x == 0.0) || x.is_nan() || x.is_infinite();
    sel(degenerate, x + x, signed)
}

/// libxc's `POW_1_3`: the system libm `cbrt`.
#[inline(always)]
pub fn cbrt_libm(x: f64) -> f64 {
    x.cbrt()
}

#[inline(always)]
pub fn piecewise3(cond: bool, val_true: f64, val_false: f64) -> f64 {
    sel(cond, val_true, val_false)
}

#[inline(always)]
pub fn piecewise5(c1: bool, v1: f64, c2: bool, v2: f64, v_else: f64) -> f64 {
    sel(c1, v1, sel(c2, v2, v_else))
}

/// One grid point of `gga_x_pbe` vxc, unpolarized. The cube root is a parameter
/// so the perf and reference legs share a single body.
#[inline(always)]
pub fn pbe_vxc_unpol_point_with<F: Fn(f64) -> f64>(
    rho_ip: f64,
    sigma_ip: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
    pow_1_3: F,
) -> (f64, f64, f64) {
    let t2 = rho_ip / 2.0 <= dens_threshold;
    let t3 = M_CBRT3;
    let t4 = M_CBRTPI;
    let t6 = t3 / t4;
    let t7 = 1.0 <= zeta_threshold;
    let t8 = zeta_threshold - 1.0;
    let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
    let t11 = 1.0 + t10;
    let t13 = pow_1_3(zeta_threshold);
    let t15 = pow_1_3(t11);
    let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
    let t18 = pow_1_3(rho_ip);
    let t20 = M_CBRT6;
    let t22 = M_PI * M_PI;
    let t23 = pow_1_3(t22);
    let t24 = t23 * t23;
    let t25 = 1.0 / t24;
    let t27 = M_CBRT2;
    let t28 = t27 * t27;
    let t30 = rho_ip * rho_ip;
    let t31 = t18 * t18;
    let t33 = 1.0 / t31 / t30;
    let t37 = param_kappa + param_mu * t20 * t25 * sigma_ip * t28 * t33 / 24.0;
    let t42 = 1.0 + param_kappa * (1.0 - param_kappa / t37);
    let t46 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t42);
    let tzk0 = 2.0 * t46;

    let t52 = t30 * rho_ip;
    let t56 = param_kappa * param_kappa;
    let t58 = t6 * t17 / t18 / t52 * t56;
    let t59 = t37 * t37;
    let t61 = 1.0 / t59 * param_mu;
    let t64 = t25 * sigma_ip * t28;
    let t65 = t61 * t20 * t64;
    let t69 = piecewise3(t2, 0.0, -t6 * t17 / t31 * t42 / 8.0 + t58 * t65 / 24.0);
    let tvrho0 = 2.0 * rho_ip * t69 + 2.0 * t46;

    let t78 = t20 * t25 * t28;
    let t79 = t61 * t78;
    let t82 = piecewise3(t2, 0.0, -t6 * t17 / t18 / t30 * t56 * t79 / 64.0);
    let tvsigma0 = 2.0 * rho_ip * t82;

    (tzk0, tvrho0, tvsigma0)
}

/// The kernel-equivalent point function (arithmetic cbrt).
#[inline(always)]
pub fn pbe_vxc_unpol_point(
    rho_ip: f64,
    sigma_ip: f64,
    kappa: f64,
    mu: f64,
    dt: f64,
    zt: f64,
) -> (f64, f64, f64) {
    pbe_vxc_unpol_point_with(rho_ip, sigma_ip, kappa, mu, dt, zt, cbrt_fast)
}

#[inline(always)]
fn sweep<F: Fn(f64) -> f64 + Copy>(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    kappa: f64,
    mu: f64,
    dt: f64,
    zt: f64,
    cbrt: F,
) {
    let n = zk.len();
    // Reslicing to a common length lets LLVM discharge every bounds check.
    let rho = &rho[..n];
    let sigma = &sigma[..n];
    let vrho = &mut vrho[..n];
    let vsigma = &mut vsigma[..n];
    for i in 0..n {
        let (a, b, c) =
            pbe_vxc_unpol_point_with(rho[i], sigma[i], kappa, mu, dt, zt, cbrt);
        zk[i] = a;
        vrho[i] = b;
        vsigma[i] = c;
    }
}

/// Single-threaded sweep, kernel-equivalent arithmetic cbrt.
pub fn pbe_vxc_unpol_serial(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    kappa: f64,
    mu: f64,
    dt: f64,
    zt: f64,
) {
    sweep(rho, sigma, zk, vrho, vsigma, kappa, mu, dt, zt, cbrt_fast);
}

/// Single-threaded sweep using libm `cbrt` — the oracle-faithful reference.
pub fn pbe_vxc_unpol_serial_libm(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    kappa: f64,
    mu: f64,
    dt: f64,
    zt: f64,
) {
    sweep(rho, sigma, zk, vrho, vsigma, kappa, mu, dt, zt, cbrt_libm);
}

/// Rayon-parallel sweep. Chunked so each worker runs a long contiguous
/// vectorisable loop rather than paying task overhead per element.
#[allow(clippy::too_many_arguments)]
pub fn pbe_vxc_unpol_rayon(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    kappa: f64,
    mu: f64,
    dt: f64,
    zt: f64,
    chunk: usize,
) {
    use rayon::prelude::*;
    let n = zk.len();
    let rho = &rho[..n];
    let sigma = &sigma[..n];
    let vrho = &mut vrho[..n];
    let vsigma = &mut vsigma[..n];

    zk.par_chunks_mut(chunk)
        .zip(vrho.par_chunks_mut(chunk))
        .zip(vsigma.par_chunks_mut(chunk))
        .zip(rho.par_chunks(chunk))
        .zip(sigma.par_chunks(chunk))
        .for_each(|((((zk_c, vrho_c), vsigma_c), rho_c), sigma_c)| {
            sweep(
                rho_c, sigma_c, zk_c, vrho_c, vsigma_c, kappa, mu, dt, zt, cbrt_fast,
            );
        });
}

/// Block-structured sweep: process `W` points at a time through explicit
/// fixed-size arrays.
///
/// NEGATIVE RESULT, kept as a reproducible probe. The intent was to coax LLVM
/// into widening the body. It does not: at W=4 the emitted code is simply the
/// scalar body unrolled 2x (1041 asm lines, 34 `divsd`, every operand `xmm`)
/// versus 527 lines / 17 `divsd` unblocked. Not one `divpd` is produced.
/// With ~17 divisions and a large live set, the vectoriser rejects this loop on
/// register pressure regardless of how the source is shaped. Real SIMD here
/// needs hand-written intrinsics, not a source-level hint.
#[inline(always)]
pub fn sweep_blocked<const W: usize>(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    kappa: f64,
    mu: f64,
    dt: f64,
    zt: f64,
) {
    let n = zk.len();
    let rho = &rho[..n];
    let sigma = &sigma[..n];
    let vrho = &mut vrho[..n];
    let vsigma = &mut vsigma[..n];

    let nb = n / W;
    for b in 0..nb {
        let o = b * W;
        let mut a = [0.0f64; W];
        let mut bb = [0.0f64; W];
        let mut c = [0.0f64; W];
        for k in 0..W {
            let (x, y, z) = pbe_vxc_unpol_point(rho[o + k], sigma[o + k], kappa, mu, dt, zt);
            a[k] = x;
            bb[k] = y;
            c[k] = z;
        }
        zk[o..o + W].copy_from_slice(&a);
        vrho[o..o + W].copy_from_slice(&bb);
        vsigma[o..o + W].copy_from_slice(&c);
    }
    for i in nb * W..n {
        let (x, y, z) = pbe_vxc_unpol_point(rho[i], sigma[i], kappa, mu, dt, zt);
        zk[i] = x;
        vrho[i] = y;
        vsigma[i] = z;
    }
}

/// Concrete instantiation so the symbol survives into the .s for inspection.
pub fn pbe_blocked4(
    rho: &[f64], sigma: &[f64], zk: &mut [f64], vrho: &mut [f64], vsigma: &mut [f64],
    kappa: f64, mu: f64, dt: f64, zt: f64,
) {
    sweep_blocked::<4>(rho, sigma, zk, vrho, vsigma, kappa, mu, dt, zt);
}
