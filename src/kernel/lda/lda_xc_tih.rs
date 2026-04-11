//! LDA_XC_TIH kernel functions translated from libxc maple2c.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_vxc/lda_xc_tih.c`.
//! Translation preserves exact maple2c variable names and operation order.

#[allow(unused_variables, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;

// ============================================================================
// UNPOLARIZED FUNCTIONS
// ============================================================================

/// LDA_XC_TIH vxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_xc_tih_vxc_unpol(
    rho: &Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let t3 = f64::tanh(1.0953 + 0.0334789 * rho[ip]);
        let t7 = f64::tanh(-0.414661 + 0.152399 * rho[ip]);
        let t11 = f64::tanh(-0.354691 + 0.0390837 * rho[ip]);
        let t15 = f64::tanh(0.0748531 + 0.136598 * rho[ip]);
        let t19 = f64::tanh(-1.41063 + 0.00496577 * rho[ip]);
        let t23 = f64::tanh(0.48315 + 4.02905 * rho[ip]);
        let t27 = f64::tanh(-0.420166 + 0.0104352 * rho[ip]);
        let t31 = f64::tanh(1.47409 + 0.442455 * rho[ip]);
        let tvrho0 = 0.625039 - 1.30351 * t3 - 1.37026 * t7 - 1.29598 * t11 + 1.04305 * t15 - 0.909651 * t19 - 0.991782 * t23 - 0.915745 * t27 - 1.95026 * t31;
        vrho[ip] += tvrho0;

    }
}

/// LDA_XC_TIH fxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_xc_tih_fxc_unpol(
    rho: &Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let t3 = f64::tanh(1.0953 + 0.0334789 * rho[ip]);
        let t7 = f64::tanh(-0.414661 + 0.152399 * rho[ip]);
        let t11 = f64::tanh(-0.354691 + 0.0390837 * rho[ip]);
        let t15 = f64::tanh(0.0748531 + 0.136598 * rho[ip]);
        let t19 = f64::tanh(-1.41063 + 0.00496577 * rho[ip]);
        let t23 = f64::tanh(0.48315 + 4.02905 * rho[ip]);
        let t27 = f64::tanh(-0.420166 + 0.0104352 * rho[ip]);
        let t31 = f64::tanh(1.47409 + 0.442455 * rho[ip]);
        let tvrho0 = 0.625039 - 1.30351 * t3 - 1.37026 * t7 - 1.29598 * t11 + 1.04305 * t15 - 0.909651 * t19 - 0.991782 * t23 - 0.915745 * t27 - 1.95026 * t31;
        vrho[ip] += tvrho0;

        let t33 = t3 * t3;
        let t35 = t7 * t7;
        let t37 = t11 * t11;
        let t39 = t15 * t15;
        let t41 = t19 * t19;
        let t43 = t23 * t23;
        let t45 = t27 * t27;
        let t47 = t31 * t31;
        let tv2rho20 = -5.03355413957527 + 0.043640080939 * t33 + 0.20882625374 * t35 + 0.050651693526 * t37 - 0.1424785439 * t39 + 0.00451711764627 * t41 + 3.9959392671 * t43 + 0.009555982224 * t45 + 0.8629022883 * t47;
        v2rho2[ip] += tv2rho20;

    }
}

/// LDA_XC_TIH kxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_xc_tih_kxc_unpol(
    rho: &Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let t3 = f64::tanh(1.0953 + 0.0334789 * rho[ip]);
        let t7 = f64::tanh(-0.414661 + 0.152399 * rho[ip]);
        let t11 = f64::tanh(-0.354691 + 0.0390837 * rho[ip]);
        let t15 = f64::tanh(0.0748531 + 0.136598 * rho[ip]);
        let t19 = f64::tanh(-1.41063 + 0.00496577 * rho[ip]);
        let t23 = f64::tanh(0.48315 + 4.02905 * rho[ip]);
        let t27 = f64::tanh(-0.420166 + 0.0104352 * rho[ip]);
        let t31 = f64::tanh(1.47409 + 0.442455 * rho[ip]);
        let tvrho0 = 0.625039 - 1.30351 * t3 - 1.37026 * t7 - 1.29598 * t11 + 1.04305 * t15 - 0.909651 * t19 - 0.991782 * t23 - 0.915745 * t27 - 1.95026 * t31;
        vrho[ip] += tvrho0;

        let t33 = t3 * t3;
        let t35 = t7 * t7;
        let t37 = t11 * t11;
        let t39 = t15 * t15;
        let t41 = t19 * t19;
        let t43 = t23 * t23;
        let t45 = t27 * t27;
        let t47 = t31 * t31;
        let tv2rho20 = -5.03355413957527 + 0.043640080939 * t33 + 0.20882625374 * t35 + 0.050651693526 * t37 - 0.1424785439 * t39 + 0.00451711764627 * t41 + 3.9959392671 * t43 + 0.009555982224 * t45 + 0.8629022883 * t47;
        v2rho2[ip] += tv2rho20;

        let t50 = 0.0334789 - 0.0334789 * t33;
        let t54 = 0.152399 - 0.152399 * t35;
        let t58 = 0.0390837 - 0.0390837 * t37;
        let t62 = 0.136598 - 0.136598 * t39;
        let t66 = 0.00496577 - 0.00496577 * t41;
        let t70 = 4.02905 - 4.02905 * t43;
        let t74 = 0.0104352 - 0.0104352 * t45;
        let t78 = 0.442455 - 0.442455 * t47;
        let tv3rho30 = 0.087280161878 * t3 * t50 + 0.41765250748 * t7 * t54 + 0.101303387052 * t11 * t58 - 0.2849570878 * t15 * t62 + 0.00903423529254 * t19 * t66 + 7.9918785342 * t23 * t70 + 0.019111964448 * t27 * t74 + 1.7258045766 * t31 * t78;
        v3rho3[ip] += tv3rho30;

    }
}

/// LDA_XC_TIH lxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_xc_tih_lxc_unpol(
    rho: &Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let t3 = f64::tanh(1.0953 + 0.0334789 * rho[ip]);
        let t7 = f64::tanh(-0.414661 + 0.152399 * rho[ip]);
        let t11 = f64::tanh(-0.354691 + 0.0390837 * rho[ip]);
        let t15 = f64::tanh(0.0748531 + 0.136598 * rho[ip]);
        let t19 = f64::tanh(-1.41063 + 0.00496577 * rho[ip]);
        let t23 = f64::tanh(0.48315 + 4.02905 * rho[ip]);
        let t27 = f64::tanh(-0.420166 + 0.0104352 * rho[ip]);
        let t31 = f64::tanh(1.47409 + 0.442455 * rho[ip]);
        let tvrho0 = 0.625039 - 1.30351 * t3 - 1.37026 * t7 - 1.29598 * t11 + 1.04305 * t15 - 0.909651 * t19 - 0.991782 * t23 - 0.915745 * t27 - 1.95026 * t31;
        vrho[ip] += tvrho0;

        let t33 = t3 * t3;
        let t35 = t7 * t7;
        let t37 = t11 * t11;
        let t39 = t15 * t15;
        let t41 = t19 * t19;
        let t43 = t23 * t23;
        let t45 = t27 * t27;
        let t47 = t31 * t31;
        let tv2rho20 = -5.03355413957527 + 0.043640080939 * t33 + 0.20882625374 * t35 + 0.050651693526 * t37 - 0.1424785439 * t39 + 0.00451711764627 * t41 + 3.9959392671 * t43 + 0.009555982224 * t45 + 0.8629022883 * t47;
        v2rho2[ip] += tv2rho20;

        let t50 = 0.0334789 - 0.0334789 * t33;
        let t54 = 0.152399 - 0.152399 * t35;
        let t58 = 0.0390837 - 0.0390837 * t37;
        let t62 = 0.136598 - 0.136598 * t39;
        let t66 = 0.00496577 - 0.00496577 * t41;
        let t70 = 4.02905 - 4.02905 * t43;
        let t74 = 0.0104352 - 0.0104352 * t45;
        let t78 = 0.442455 - 0.442455 * t47;
        let tv3rho30 = 0.087280161878 * t3 * t50 + 0.41765250748 * t7 * t54 + 0.101303387052 * t11 * t58 - 0.2849570878 * t15 * t62 + 0.00903423529254 * t19 * t66 + 7.9918785342 * t23 * t70 + 0.019111964448 * t27 * t74 + 1.7258045766 * t31 * t78;
        v3rho3[ip] += tv3rho30;

        let t81 = t50 * t50;
        let t85 = t54 * t54;
        let t89 = t58 * t58;
        let t93 = t62 * t62;
        let t97 = t66 * t66;
        let t101 = t70 * t70;
        let t105 = t74 * t74;
        let t109 = t78 * t78;
        let tv4rho40 = 0.087280161878 * t81 - 0.005844087622994748 * t33 * t50 + 0.41765250748 * t85 - 0.12729964897488905 * t35 * t54 + 0.101303387052 * t89 - 0.007918622377048505 * t37 * t58 - 0.2849570878 * t93 + 0.0778491365586088 * t39 * t62 + 0.00903423529254 * t97 - 8.972386917727271e-05 * t41 * t66 + 7.9918785342 * t101 - 64.39935641643702 * t43 * t70 + 0.019111964448 * t105 - 0.0003988743428155392 * t45 * t74 + 1.7258045766 * t109 - 1.527181727879106 * t47 * t78;
        v4rho4[ip] += tv4rho40;

    }
}

// ============================================================================
// POLARIZED FUNCTIONS
// ============================================================================

/// LDA_XC_TIH vxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_xc_tih_vxc_pol(
    rho: &Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t4 = f64::tanh(1.0953 + 0.0334789 * rho[ip * 2] + 0.0334789 * rho[ip * 2 + 1]);
        let t9 = f64::tanh(-0.414661 + 0.152399 * rho[ip * 2] + 0.152399 * rho[ip * 2 + 1]);
        let t14 = f64::tanh(-0.354691 + 0.0390837 * rho[ip * 2] + 0.0390837 * rho[ip * 2 + 1]);
        let t19 = f64::tanh(0.0748531 + 0.136598 * rho[ip * 2] + 0.136598 * rho[ip * 2 + 1]);
        let t24 = f64::tanh(-1.41063 + 0.00496577 * rho[ip * 2] + 0.00496577 * rho[ip * 2 + 1]);
        let t29 = f64::tanh(0.48315 + 4.02905 * rho[ip * 2] + 4.02905 * rho[ip * 2 + 1]);
        let t34 = f64::tanh(-0.420166 + 0.0104352 * rho[ip * 2] + 0.0104352 * rho[ip * 2 + 1]);
        let t39 = f64::tanh(1.47409 + 0.442455 * rho[ip * 2] + 0.442455 * rho[ip * 2 + 1]);
        let tvrho0 = 0.625039 - 1.30351 * t4 - 1.37026 * t9 - 1.29598 * t14 + 1.04305 * t19 - 0.909651 * t24 - 0.991782 * t29 - 0.915745 * t34 - 1.95026 * t39;
        vrho[ip * 2] += tvrho0;

        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;

    }
}

/// LDA_XC_TIH fxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_xc_tih_fxc_pol(
    rho: &Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t4 = f64::tanh(1.0953 + 0.0334789 * rho[ip * 2] + 0.0334789 * rho[ip * 2 + 1]);
        let t9 = f64::tanh(-0.414661 + 0.152399 * rho[ip * 2] + 0.152399 * rho[ip * 2 + 1]);
        let t14 = f64::tanh(-0.354691 + 0.0390837 * rho[ip * 2] + 0.0390837 * rho[ip * 2 + 1]);
        let t19 = f64::tanh(0.0748531 + 0.136598 * rho[ip * 2] + 0.136598 * rho[ip * 2 + 1]);
        let t24 = f64::tanh(-1.41063 + 0.00496577 * rho[ip * 2] + 0.00496577 * rho[ip * 2 + 1]);
        let t29 = f64::tanh(0.48315 + 4.02905 * rho[ip * 2] + 4.02905 * rho[ip * 2 + 1]);
        let t34 = f64::tanh(-0.420166 + 0.0104352 * rho[ip * 2] + 0.0104352 * rho[ip * 2 + 1]);
        let t39 = f64::tanh(1.47409 + 0.442455 * rho[ip * 2] + 0.442455 * rho[ip * 2 + 1]);
        let tvrho0 = 0.625039 - 1.30351 * t4 - 1.37026 * t9 - 1.29598 * t14 + 1.04305 * t19 - 0.909651 * t24 - 0.991782 * t29 - 0.915745 * t34 - 1.95026 * t39;
        vrho[ip * 2] += tvrho0;

        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;

        let t41 = t4 * t4;
        let t43 = t9 * t9;
        let t45 = t14 * t14;
        let t47 = t19 * t19;
        let t49 = t24 * t24;
        let t51 = t29 * t29;
        let t53 = t34 * t34;
        let t55 = t39 * t39;
        let tv2rho20 = -5.03355413957527 + 0.043640080939 * t41 + 0.20882625374 * t43 + 0.050651693526 * t45 - 0.1424785439 * t47 + 0.00451711764627 * t49 + 3.9959392671 * t51 + 0.009555982224 * t53 + 0.8629022883 * t55;
        v2rho2[ip * 3] += tv2rho20;

        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;

        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;

    }
}

/// LDA_XC_TIH kxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_xc_tih_kxc_pol(
    rho: &Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t4 = f64::tanh(1.0953 + 0.0334789 * rho[ip * 2] + 0.0334789 * rho[ip * 2 + 1]);
        let t9 = f64::tanh(-0.414661 + 0.152399 * rho[ip * 2] + 0.152399 * rho[ip * 2 + 1]);
        let t14 = f64::tanh(-0.354691 + 0.0390837 * rho[ip * 2] + 0.0390837 * rho[ip * 2 + 1]);
        let t19 = f64::tanh(0.0748531 + 0.136598 * rho[ip * 2] + 0.136598 * rho[ip * 2 + 1]);
        let t24 = f64::tanh(-1.41063 + 0.00496577 * rho[ip * 2] + 0.00496577 * rho[ip * 2 + 1]);
        let t29 = f64::tanh(0.48315 + 4.02905 * rho[ip * 2] + 4.02905 * rho[ip * 2 + 1]);
        let t34 = f64::tanh(-0.420166 + 0.0104352 * rho[ip * 2] + 0.0104352 * rho[ip * 2 + 1]);
        let t39 = f64::tanh(1.47409 + 0.442455 * rho[ip * 2] + 0.442455 * rho[ip * 2 + 1]);
        let tvrho0 = 0.625039 - 1.30351 * t4 - 1.37026 * t9 - 1.29598 * t14 + 1.04305 * t19 - 0.909651 * t24 - 0.991782 * t29 - 0.915745 * t34 - 1.95026 * t39;
        vrho[ip * 2] += tvrho0;

        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;

        let t41 = t4 * t4;
        let t43 = t9 * t9;
        let t45 = t14 * t14;
        let t47 = t19 * t19;
        let t49 = t24 * t24;
        let t51 = t29 * t29;
        let t53 = t34 * t34;
        let t55 = t39 * t39;
        let tv2rho20 = -5.03355413957527 + 0.043640080939 * t41 + 0.20882625374 * t43 + 0.050651693526 * t45 - 0.1424785439 * t47 + 0.00451711764627 * t49 + 3.9959392671 * t51 + 0.009555982224 * t53 + 0.8629022883 * t55;
        v2rho2[ip * 3] += tv2rho20;

        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;

        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;

        let t58 = 0.0334789 - 0.0334789 * t41;
        let t62 = 0.152399 - 0.152399 * t43;
        let t66 = 0.0390837 - 0.0390837 * t45;
        let t70 = 0.136598 - 0.136598 * t47;
        let t74 = 0.00496577 - 0.00496577 * t49;
        let t78 = 4.02905 - 4.02905 * t51;
        let t82 = 0.0104352 - 0.0104352 * t53;
        let t86 = 0.442455 - 0.442455 * t55;
        let tv3rho30 = 0.087280161878 * t4 * t58 + 0.41765250748 * t9 * t62 + 0.101303387052 * t14 * t66 - 0.2849570878 * t19 * t70 + 0.00903423529254 * t24 * t74 + 7.9918785342 * t29 * t78 + 0.019111964448 * t34 * t82 + 1.7258045766 * t39 * t86;
        v3rho3[ip * 4] += tv3rho30;

        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;

        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;

        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;

    }
}

/// LDA_XC_TIH lxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_xc_tih_lxc_pol(
    rho: &Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t4 = f64::tanh(1.0953 + 0.0334789 * rho[ip * 2] + 0.0334789 * rho[ip * 2 + 1]);
        let t9 = f64::tanh(-0.414661 + 0.152399 * rho[ip * 2] + 0.152399 * rho[ip * 2 + 1]);
        let t14 = f64::tanh(-0.354691 + 0.0390837 * rho[ip * 2] + 0.0390837 * rho[ip * 2 + 1]);
        let t19 = f64::tanh(0.0748531 + 0.136598 * rho[ip * 2] + 0.136598 * rho[ip * 2 + 1]);
        let t24 = f64::tanh(-1.41063 + 0.00496577 * rho[ip * 2] + 0.00496577 * rho[ip * 2 + 1]);
        let t29 = f64::tanh(0.48315 + 4.02905 * rho[ip * 2] + 4.02905 * rho[ip * 2 + 1]);
        let t34 = f64::tanh(-0.420166 + 0.0104352 * rho[ip * 2] + 0.0104352 * rho[ip * 2 + 1]);
        let t39 = f64::tanh(1.47409 + 0.442455 * rho[ip * 2] + 0.442455 * rho[ip * 2 + 1]);
        let tvrho0 = 0.625039 - 1.30351 * t4 - 1.37026 * t9 - 1.29598 * t14 + 1.04305 * t19 - 0.909651 * t24 - 0.991782 * t29 - 0.915745 * t34 - 1.95026 * t39;
        vrho[ip * 2] += tvrho0;

        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;

        let t41 = t4 * t4;
        let t43 = t9 * t9;
        let t45 = t14 * t14;
        let t47 = t19 * t19;
        let t49 = t24 * t24;
        let t51 = t29 * t29;
        let t53 = t34 * t34;
        let t55 = t39 * t39;
        let tv2rho20 = -5.03355413957527 + 0.043640080939 * t41 + 0.20882625374 * t43 + 0.050651693526 * t45 - 0.1424785439 * t47 + 0.00451711764627 * t49 + 3.9959392671 * t51 + 0.009555982224 * t53 + 0.8629022883 * t55;
        v2rho2[ip * 3] += tv2rho20;

        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;

        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;

        let t58 = 0.0334789 - 0.0334789 * t41;
        let t62 = 0.152399 - 0.152399 * t43;
        let t66 = 0.0390837 - 0.0390837 * t45;
        let t70 = 0.136598 - 0.136598 * t47;
        let t74 = 0.00496577 - 0.00496577 * t49;
        let t78 = 4.02905 - 4.02905 * t51;
        let t82 = 0.0104352 - 0.0104352 * t53;
        let t86 = 0.442455 - 0.442455 * t55;
        let tv3rho30 = 0.087280161878 * t4 * t58 + 0.41765250748 * t9 * t62 + 0.101303387052 * t14 * t66 - 0.2849570878 * t19 * t70 + 0.00903423529254 * t24 * t74 + 7.9918785342 * t29 * t78 + 0.019111964448 * t34 * t82 + 1.7258045766 * t39 * t86;
        v3rho3[ip * 4] += tv3rho30;

        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;

        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;

        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;

        let t89 = t58 * t58;
        let t93 = t62 * t62;
        let t97 = t66 * t66;
        let t101 = t70 * t70;
        let t105 = t74 * t74;
        let t109 = t78 * t78;
        let t113 = t82 * t82;
        let t117 = t86 * t86;
        let tv4rho40 = 0.087280161878 * t89 - 0.005844087622994748 * t41 * t58 + 0.41765250748 * t93 - 0.12729964897488905 * t43 * t62 + 0.101303387052 * t97 - 0.007918622377048505 * t45 * t66 - 0.2849570878 * t101 + 0.0778491365586088 * t47 * t70 + 0.00903423529254 * t105 - 8.972386917727271e-05 * t49 * t74 + 7.9918785342 * t109 - 64.39935641643702 * t51 * t78 + 0.019111964448 * t113 - 0.0003988743428155392 * t53 * t82 + 1.7258045766 * t117 - 1.527181727879106 * t55 * t86;
        v4rho4[ip * 5] += tv4rho40;

        let tv4rho41 = tv4rho40;
        v4rho4[ip * 5 + 1] += tv4rho41;

        let tv4rho42 = tv4rho41;
        v4rho4[ip * 5 + 2] += tv4rho42;

        let tv4rho43 = tv4rho42;
        v4rho4[ip * 5 + 3] += tv4rho43;

        let tv4rho44 = tv4rho43;
        v4rho4[ip * 5 + 4] += tv4rho44;

    }
}
