//! LDA_K_ZLP kernel functions translated from libxc maple2c.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_zlp.c`.
//! Translation preserves exact maple2c variable names and operation order.

#[allow(unused_variables, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_PI, M_CBRT3, M_CBRT4};
use crate::math::powers::{pow_1_3};
use crate::math::piecewise::{piecewise3};

// ============================================================================
// UNPOLARIZED FUNCTIONS
// ============================================================================

/// LDA_K_ZLP exc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_k_zlp_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = 1.0 / t4;
        let t7 = M_CBRT4;
        let t8 = t2 * t5 * t7;
        let t10 = pow_1_3(zeta_threshold);
        let t11 = t10 * t10;
        let t13 = piecewise3(1.0 <= zeta_threshold, t11 * zeta_threshold, 1.0);
        let t14 = pow_1_3(rho[ip]);
        let t15 = t14 * t14;
        let t16 = t13 * t15;
        let t17 = 1.0 / t14;
        let t19 = 1.0 + 510.2040816326531 * t17;
        let t20 = f64::ln(t19);
        let t23 = 1.0 - 0.00196 * t14 * t20;
        let t25 = t8 * t16 * t23;
        let tzk0 = 1.0790666666666666 * t25;
        zk[ip] += tzk0;

    }
}

/// LDA_K_ZLP vxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_k_zlp_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = 1.0 / t4;
        let t7 = M_CBRT4;
        let t8 = t2 * t5 * t7;
        let t10 = pow_1_3(zeta_threshold);
        let t11 = t10 * t10;
        let t13 = piecewise3(1.0 <= zeta_threshold, t11 * zeta_threshold, 1.0);
        let t14 = pow_1_3(rho[ip]);
        let t15 = t14 * t14;
        let t16 = t13 * t15;
        let t17 = 1.0 / t14;
        let t19 = 1.0 + 510.2040816326531 * t17;
        let t20 = f64::ln(t19);
        let t23 = 1.0 - 0.00196 * t14 * t20;
        let t25 = t8 * t16 * t23;
        let tzk0 = 1.0790666666666666 * t25;
        zk[ip] += tzk0;

        let t27 = t15 * rho[ip];
        let t29 = t27 * t2 * t5;
        let t30 = t7 * t13;
        let t35 = 1.0 / t19;
        let t38 = -0.0006533333333333333 / t15 * t20 + 0.3333333333333333 / rho[ip] * t35;
        let tvrho0 = 1.7984444444444445 * t25 + 1.0790666666666666 * t29 * t30 * t38;
        vrho[ip] += tvrho0;

    }
}

/// LDA_K_ZLP fxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_k_zlp_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = 1.0 / t4;
        let t7 = M_CBRT4;
        let t8 = t2 * t5 * t7;
        let t10 = pow_1_3(zeta_threshold);
        let t11 = t10 * t10;
        let t13 = piecewise3(1.0 <= zeta_threshold, t11 * zeta_threshold, 1.0);
        let t14 = pow_1_3(rho[ip]);
        let t15 = t14 * t14;
        let t16 = t13 * t15;
        let t17 = 1.0 / t14;
        let t19 = 1.0 + 510.2040816326531 * t17;
        let t20 = f64::ln(t19);
        let t23 = 1.0 - 0.00196 * t14 * t20;
        let t25 = t8 * t16 * t23;
        let tzk0 = 1.0790666666666666 * t25;
        zk[ip] += tzk0;

        let t27 = t15 * rho[ip];
        let t29 = t27 * t2 * t5;
        let t30 = t7 * t13;
        let t35 = 1.0 / t19;
        let t38 = -0.0006533333333333333 / t15 * t20 + 0.3333333333333333 / rho[ip] * t35;
        let tvrho0 = 1.7984444444444445 * t25 + 1.0790666666666666 * t29 * t30 * t38;
        vrho[ip] += tvrho0;

        let t42 = t13 * t17;
        let t52 = rho[ip] * rho[ip];
        let t57 = 1.0 / t14 / t52;
        let t58 = t19 * t19;
        let t59 = 1.0 / t58;
        let t62 = 0.00043555555555555557 / t27 * t20 - 0.2222222222222222 / t52 * t35 + 56.68934240362812 * t57 * t59;
        let tv2rho20 = 1.198962962962963 * t8 * t42 * t23 + 3.596888888888889 * t8 * t16 * t38 + 1.0790666666666666 * t29 * t30 * t62;
        v2rho2[ip] += tv2rho20;

    }
}

/// LDA_K_ZLP kxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_k_zlp_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = 1.0 / t4;
        let t7 = M_CBRT4;
        let t8 = t2 * t5 * t7;
        let t10 = pow_1_3(zeta_threshold);
        let t11 = t10 * t10;
        let t13 = piecewise3(1.0 <= zeta_threshold, t11 * zeta_threshold, 1.0);
        let t14 = pow_1_3(rho[ip]);
        let t15 = t14 * t14;
        let t16 = t13 * t15;
        let t17 = 1.0 / t14;
        let t19 = 1.0 + 510.2040816326531 * t17;
        let t20 = f64::ln(t19);
        let t23 = 1.0 - 0.00196 * t14 * t20;
        let t25 = t8 * t16 * t23;
        let tzk0 = 1.0790666666666666 * t25;
        zk[ip] += tzk0;

        let t27 = t15 * rho[ip];
        let t29 = t27 * t2 * t5;
        let t30 = t7 * t13;
        let t35 = 1.0 / t19;
        let t38 = -0.0006533333333333333 / t15 * t20 + 0.3333333333333333 / rho[ip] * t35;
        let tvrho0 = 1.7984444444444445 * t25 + 1.0790666666666666 * t29 * t30 * t38;
        vrho[ip] += tvrho0;

        let t42 = t13 * t17;
        let t52 = rho[ip] * rho[ip];
        let t57 = 1.0 / t14 / t52;
        let t58 = t19 * t19;
        let t59 = 1.0 / t58;
        let t62 = 0.00043555555555555557 / t27 * t20 - 0.2222222222222222 / t52 * t35 + 56.68934240362812 * t57 * t59;
        let tv2rho20 = 1.198962962962963 * t8 * t42 * t23 + 3.596888888888889 * t8 * t16 * t38 + 1.0790666666666666 * t29 * t30 * t62;
        v2rho2[ip] += tv2rho20;

        let t68 = t13 / t14 / rho[ip];
        let t82 = t52 * rho[ip];
        let t91 = 1.0 / t15 / t82;
        let t93 = 1.0 / t58 / t19;
        let t96 = -0.000725925925925926 / t15 / t52 * t20 + 0.37037037037037035 / t82 * t35 - 170.06802721088437 / t14 / t82 * t59 + 19282.089252934733 * t91 * t93;
        let tv3rho30 = -0.3996543209876543 * t8 * t68 * t23 + 3.596888888888889 * t8 * t42 * t38 + 5.395333333333333 * t8 * t16 * t62 + 1.0790666666666666 * t29 * t30 * t96;
        v3rho3[ip] += tv3rho30;

    }
}

/// LDA_K_ZLP lxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_k_zlp_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = 1.0 / t4;
        let t7 = M_CBRT4;
        let t8 = t2 * t5 * t7;
        let t10 = pow_1_3(zeta_threshold);
        let t11 = t10 * t10;
        let t13 = piecewise3(1.0 <= zeta_threshold, t11 * zeta_threshold, 1.0);
        let t14 = pow_1_3(rho[ip]);
        let t15 = t14 * t14;
        let t16 = t13 * t15;
        let t17 = 1.0 / t14;
        let t19 = 1.0 + 510.2040816326531 * t17;
        let t20 = f64::ln(t19);
        let t23 = 1.0 - 0.00196 * t14 * t20;
        let t25 = t8 * t16 * t23;
        let tzk0 = 1.0790666666666666 * t25;
        zk[ip] += tzk0;

        let t27 = t15 * rho[ip];
        let t29 = t27 * t2 * t5;
        let t30 = t7 * t13;
        let t35 = 1.0 / t19;
        let t38 = -0.0006533333333333333 / t15 * t20 + 0.3333333333333333 / rho[ip] * t35;
        let tvrho0 = 1.7984444444444445 * t25 + 1.0790666666666666 * t29 * t30 * t38;
        vrho[ip] += tvrho0;

        let t42 = t13 * t17;
        let t52 = rho[ip] * rho[ip];
        let t57 = 1.0 / t14 / t52;
        let t58 = t19 * t19;
        let t59 = 1.0 / t58;
        let t62 = 0.00043555555555555557 / t27 * t20 - 0.2222222222222222 / t52 * t35 + 56.68934240362812 * t57 * t59;
        let tv2rho20 = 1.198962962962963 * t8 * t42 * t23 + 3.596888888888889 * t8 * t16 * t38 + 1.0790666666666666 * t29 * t30 * t62;
        v2rho2[ip] += tv2rho20;

        let t68 = t13 / t14 / rho[ip];
        let t82 = t52 * rho[ip];
        let t91 = 1.0 / t15 / t82;
        let t93 = 1.0 / t58 / t19;
        let t96 = -0.000725925925925926 / t15 / t52 * t20 + 0.37037037037037035 / t82 * t35 - 170.06802721088437 / t14 / t82 * t59 + 19282.089252934733 * t91 * t93;
        let tv3rho30 = -0.3996543209876543 * t8 * t68 * t23 + 3.596888888888889 * t8 * t42 * t38 + 5.395333333333333 * t8 * t16 * t62 + 1.0790666666666666 * t29 * t30 * t96;
        v3rho3[ip] += tv3rho30;

        let t115 = t52 * t52;
        let t129 = t58 * t58;
        let tv4rho40 = 0.5328724279835391 * t8 * t13 * t57 * t23 - 1.5986172839506172 * t8 * t68 * t38 + 7.193777777777778 * t8 * t42 * t62 + 7.193777777777778 * t8 * t16 * t96 + 1.0790666666666666 * t29 * t30 * (0.0019358024691358024 * t91 * t20 - 0.9876543209876543 / t115 * t35 + 629.8815822625346 / t14 / t115 * t59 - 128547.26168623156 / t15 / t115 * t93 + 9837800.639252415 / t115 / rho[ip] / t129);
        v4rho4[ip] += tv4rho40;

    }
}

// ============================================================================
// POLARIZED FUNCTIONS
// ============================================================================

/// LDA_K_ZLP exc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_k_zlp_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = 1.0 / t4;
        let t7 = M_CBRT4;
        let t8 = t2 * t5 * t7;
        let t9 = rho[ip * 2] - rho[ip * 2 + 1];
        let t10 = rho[ip * 2] + rho[ip * 2 + 1];
        let t11 = 1.0 / t10;
        let t12 = t9 * t11;
        let t13 = 1.0 + t12;
        let t14 = t13 <= zeta_threshold;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t17 = t16 * zeta_threshold;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t14, t17, t19 * t13);
        let t22 = 1.0 - t12;
        let t23 = t22 <= zeta_threshold;
        let t24 = pow_1_3(t22);
        let t25 = t24 * t24;
        let t27 = piecewise3(t23, t17, t25 * t22);
        let t29 = t21 / 2.0 + t27 / 2.0;
        let t30 = pow_1_3(t10);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = 1.0 / t30;
        let t35 = 1.0 + 510.2040816326531 * t33;
        let t36 = f64::ln(t35);
        let t39 = 1.0 - 0.00196 * t30 * t36;
        let t41 = t8 * t32 * t39;
        let tzk0 = 1.0790666666666666 * t41;
        zk[ip] += tzk0;

    }
}

/// LDA_K_ZLP vxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_k_zlp_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = 1.0 / t4;
        let t7 = M_CBRT4;
        let t8 = t2 * t5 * t7;
        let t9 = rho[ip * 2] - rho[ip * 2 + 1];
        let t10 = rho[ip * 2] + rho[ip * 2 + 1];
        let t11 = 1.0 / t10;
        let t12 = t9 * t11;
        let t13 = 1.0 + t12;
        let t14 = t13 <= zeta_threshold;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t17 = t16 * zeta_threshold;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t14, t17, t19 * t13);
        let t22 = 1.0 - t12;
        let t23 = t22 <= zeta_threshold;
        let t24 = pow_1_3(t22);
        let t25 = t24 * t24;
        let t27 = piecewise3(t23, t17, t25 * t22);
        let t29 = t21 / 2.0 + t27 / 2.0;
        let t30 = pow_1_3(t10);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = 1.0 / t30;
        let t35 = 1.0 + 510.2040816326531 * t33;
        let t36 = f64::ln(t35);
        let t39 = 1.0 - 0.00196 * t30 * t36;
        let t41 = t8 * t32 * t39;
        let tzk0 = 1.0790666666666666 * t41;
        zk[ip] += tzk0;

        let t42 = 1.7984444444444445 * t41;
        let t43 = t31 * t10;
        let t45 = t43 * t2 * t5;
        let t46 = t10 * t10;
        let t47 = 1.0 / t46;
        let t48 = t9 * t47;
        let t49 = t11 - t48;
        let t52 = piecewise3(t14, 0.0, 5.0 / 3.0 * t19 * t49);
        let t53 = -t49;
        let t56 = piecewise3(t23, 0.0, 5.0 / 3.0 * t25 * t53);
        let t58 = t52 / 2.0 + t56 / 2.0;
        let t59 = t7 * t58;
        let t63 = t7 * t29;
        let t67 = 1.0 / t35;
        let t70 = -0.0006533333333333333 / t31 * t36 + 0.3333333333333333 * t11 * t67;
        let t73 = 1.0790666666666666 * t45 * t63 * t70;
        let tvrho0 = t42 + 1.0790666666666666 * t45 * t59 * t39 + t73;
        vrho[ip * 2] += tvrho0;

        let t74 = -t11 - t48;
        let t77 = piecewise3(t14, 0.0, 5.0 / 3.0 * t19 * t74);
        let t78 = -t74;
        let t81 = piecewise3(t23, 0.0, 5.0 / 3.0 * t25 * t78);
        let t84 = t7 * (t77 / 2.0 + t81 / 2.0);
        let t85 = t84 * t39;
        let tvrho1 = t42 + 1.0790666666666666 * t45 * t85 + t73;
        vrho[ip * 2 + 1] += tvrho1;

    }
}

/// LDA_K_ZLP fxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_k_zlp_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = 1.0 / t4;
        let t7 = M_CBRT4;
        let t8 = t2 * t5 * t7;
        let t9 = rho[ip * 2] - rho[ip * 2 + 1];
        let t10 = rho[ip * 2] + rho[ip * 2 + 1];
        let t11 = 1.0 / t10;
        let t12 = t9 * t11;
        let t13 = 1.0 + t12;
        let t14 = t13 <= zeta_threshold;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t17 = t16 * zeta_threshold;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t14, t17, t19 * t13);
        let t22 = 1.0 - t12;
        let t23 = t22 <= zeta_threshold;
        let t24 = pow_1_3(t22);
        let t25 = t24 * t24;
        let t27 = piecewise3(t23, t17, t25 * t22);
        let t29 = t21 / 2.0 + t27 / 2.0;
        let t30 = pow_1_3(t10);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = 1.0 / t30;
        let t35 = 1.0 + 510.2040816326531 * t33;
        let t36 = f64::ln(t35);
        let t39 = 1.0 - 0.00196 * t30 * t36;
        let t41 = t8 * t32 * t39;
        let tzk0 = 1.0790666666666666 * t41;
        zk[ip] += tzk0;

        let t42 = 1.7984444444444445 * t41;
        let t43 = t31 * t10;
        let t45 = t43 * t2 * t5;
        let t46 = t10 * t10;
        let t47 = 1.0 / t46;
        let t48 = t9 * t47;
        let t49 = t11 - t48;
        let t52 = piecewise3(t14, 0.0, 5.0 / 3.0 * t19 * t49);
        let t53 = -t49;
        let t56 = piecewise3(t23, 0.0, 5.0 / 3.0 * t25 * t53);
        let t58 = t52 / 2.0 + t56 / 2.0;
        let t59 = t7 * t58;
        let t63 = t7 * t29;
        let t67 = 1.0 / t35;
        let t70 = -0.0006533333333333333 / t31 * t36 + 0.3333333333333333 * t11 * t67;
        let t73 = 1.0790666666666666 * t45 * t63 * t70;
        let tvrho0 = t42 + 1.0790666666666666 * t45 * t59 * t39 + t73;
        vrho[ip * 2] += tvrho0;

        let t74 = -t11 - t48;
        let t77 = piecewise3(t14, 0.0, 5.0 / 3.0 * t19 * t74);
        let t78 = -t74;
        let t81 = piecewise3(t23, 0.0, 5.0 / 3.0 * t25 * t78);
        let t84 = t7 * (t77 / 2.0 + t81 / 2.0);
        let t85 = t84 * t39;
        let tvrho1 = t42 + 1.0790666666666666 * t45 * t85 + t73;
        vrho[ip * 2 + 1] += tvrho1;

        let t88 = t58 * t31;
        let t90 = t8 * t88 * t39;
        let t92 = t29 * t33;
        let t95 = 1.198962962962963 * t8 * t92 * t39;
        let t98 = 3.596888888888889 * t8 * t32 * t70;
        let t99 = 1.0 / t18;
        let t100 = t49 * t49;
        let t103 = t46 * t10;
        let t104 = 1.0 / t103;
        let t105 = t9 * t104;
        let t107 = -2.0 * t47 + 2.0 * t105;
        let t111 = piecewise3(t14, 0.0, 10.0 / 9.0 * t99 * t100 + 5.0 / 3.0 * t19 * t107);
        let t112 = 1.0 / t24;
        let t113 = t53 * t53;
        let t116 = -t107;
        let t120 = piecewise3(t23, 0.0, 10.0 / 9.0 * t112 * t113 + 5.0 / 3.0 * t25 * t116);
        let t122 = t111 / 2.0 + t120 / 2.0;
        let t123 = t7 * t122;
        let t128 = t45 * t59 * t70;
        let t136 = 1.0 / t30 / t46;
        let t137 = t35 * t35;
        let t138 = 1.0 / t137;
        let t141 = 0.00043555555555555557 / t43 * t36 - 0.2222222222222222 * t47 * t67 + 56.68934240362812 * t136 * t138;
        let t144 = 1.0790666666666666 * t45 * t63 * t141;
        let tv2rho20 = 3.596888888888889 * t90 + t95 + t98 + 1.0790666666666666 * t45 * t123 * t39 + 2.1581333333333332 * t128 + t144;
        v2rho2[ip * 3] += tv2rho20;

        let t147 = t31 * t2 * t5;
        let t148 = t147 * t85;
        let t150 = t99 * t74;
        let t153 = t19 * t9;
        let t157 = piecewise3(t14, 0.0, 10.0 / 9.0 * t150 * t49 + 10.0 / 3.0 * t153 * t104);
        let t158 = t112 * t78;
        let t161 = t25 * t9;
        let t165 = piecewise3(t23, 0.0, 10.0 / 9.0 * t158 * t53 - 10.0 / 3.0 * t161 * t104);
        let t168 = t7 * (t157 / 2.0 + t165 / 2.0);
        let t169 = t168 * t39;
        let t172 = t84 * t70;
        let t173 = t45 * t172;
        let tv2rho21 = 1.7984444444444445 * t90 + t95 + t98 + 1.7984444444444445 * t148 + 1.0790666666666666 * t45 * t169 + 1.0790666666666666 * t173 + 1.0790666666666666 * t128 + t144;
        v2rho2[ip * 3 + 1] += tv2rho21;

        let t177 = t74 * t74;
        let t181 = 2.0 * t47 + 2.0 * t105;
        let t185 = piecewise3(t14, 0.0, 10.0 / 9.0 * t99 * t177 + 5.0 / 3.0 * t19 * t181);
        let t186 = t78 * t78;
        let t189 = -t181;
        let t193 = piecewise3(t23, 0.0, 10.0 / 9.0 * t112 * t186 + 5.0 / 3.0 * t25 * t189);
        let t196 = t7 * (t185 / 2.0 + t193 / 2.0);
        let t197 = t196 * t39;
        let tv2rho22 = 3.596888888888889 * t148 + t95 + t98 + 1.0790666666666666 * t45 * t197 + 2.1581333333333332 * t173 + t144;
        v2rho2[ip * 3 + 2] += tv2rho22;

    }
}

/// LDA_K_ZLP kxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_k_zlp_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = 1.0 / t4;
        let t7 = M_CBRT4;
        let t8 = t2 * t5 * t7;
        let t9 = rho[ip * 2] - rho[ip * 2 + 1];
        let t10 = rho[ip * 2] + rho[ip * 2 + 1];
        let t11 = 1.0 / t10;
        let t12 = t9 * t11;
        let t13 = 1.0 + t12;
        let t14 = t13 <= zeta_threshold;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t17 = t16 * zeta_threshold;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t14, t17, t19 * t13);
        let t22 = 1.0 - t12;
        let t23 = t22 <= zeta_threshold;
        let t24 = pow_1_3(t22);
        let t25 = t24 * t24;
        let t27 = piecewise3(t23, t17, t25 * t22);
        let t29 = t21 / 2.0 + t27 / 2.0;
        let t30 = pow_1_3(t10);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = 1.0 / t30;
        let t35 = 1.0 + 510.2040816326531 * t33;
        let t36 = f64::ln(t35);
        let t39 = 1.0 - 0.00196 * t30 * t36;
        let t41 = t8 * t32 * t39;
        let tzk0 = 1.0790666666666666 * t41;
        zk[ip] += tzk0;

        let t42 = 1.7984444444444445 * t41;
        let t43 = t31 * t10;
        let t45 = t43 * t2 * t5;
        let t46 = t10 * t10;
        let t47 = 1.0 / t46;
        let t48 = t9 * t47;
        let t49 = t11 - t48;
        let t52 = piecewise3(t14, 0.0, 5.0 / 3.0 * t19 * t49);
        let t53 = -t49;
        let t56 = piecewise3(t23, 0.0, 5.0 / 3.0 * t25 * t53);
        let t58 = t52 / 2.0 + t56 / 2.0;
        let t59 = t7 * t58;
        let t63 = t7 * t29;
        let t67 = 1.0 / t35;
        let t70 = -0.0006533333333333333 / t31 * t36 + 0.3333333333333333 * t11 * t67;
        let t73 = 1.0790666666666666 * t45 * t63 * t70;
        let tvrho0 = t42 + 1.0790666666666666 * t45 * t59 * t39 + t73;
        vrho[ip * 2] += tvrho0;

        let t74 = -t11 - t48;
        let t77 = piecewise3(t14, 0.0, 5.0 / 3.0 * t19 * t74);
        let t78 = -t74;
        let t81 = piecewise3(t23, 0.0, 5.0 / 3.0 * t25 * t78);
        let t84 = t7 * (t77 / 2.0 + t81 / 2.0);
        let t85 = t84 * t39;
        let tvrho1 = t42 + 1.0790666666666666 * t45 * t85 + t73;
        vrho[ip * 2 + 1] += tvrho1;

        let t88 = t58 * t31;
        let t90 = t8 * t88 * t39;
        let t92 = t29 * t33;
        let t95 = 1.198962962962963 * t8 * t92 * t39;
        let t98 = 3.596888888888889 * t8 * t32 * t70;
        let t99 = 1.0 / t18;
        let t100 = t49 * t49;
        let t103 = t46 * t10;
        let t104 = 1.0 / t103;
        let t105 = t9 * t104;
        let t107 = -2.0 * t47 + 2.0 * t105;
        let t111 = piecewise3(t14, 0.0, 10.0 / 9.0 * t99 * t100 + 5.0 / 3.0 * t19 * t107);
        let t112 = 1.0 / t24;
        let t113 = t53 * t53;
        let t116 = -t107;
        let t120 = piecewise3(t23, 0.0, 10.0 / 9.0 * t112 * t113 + 5.0 / 3.0 * t25 * t116);
        let t122 = t111 / 2.0 + t120 / 2.0;
        let t123 = t7 * t122;
        let t128 = t45 * t59 * t70;
        let t136 = 1.0 / t30 / t46;
        let t137 = t35 * t35;
        let t138 = 1.0 / t137;
        let t141 = 0.00043555555555555557 / t43 * t36 - 0.2222222222222222 * t47 * t67 + 56.68934240362812 * t136 * t138;
        let t144 = 1.0790666666666666 * t45 * t63 * t141;
        let tv2rho20 = 3.596888888888889 * t90 + t95 + t98 + 1.0790666666666666 * t45 * t123 * t39 + 2.1581333333333332 * t128 + t144;
        v2rho2[ip * 3] += tv2rho20;

        let t147 = t31 * t2 * t5;
        let t148 = t147 * t85;
        let t150 = t99 * t74;
        let t153 = t19 * t9;
        let t157 = piecewise3(t14, 0.0, 10.0 / 9.0 * t150 * t49 + 10.0 / 3.0 * t153 * t104);
        let t158 = t112 * t78;
        let t161 = t25 * t9;
        let t165 = piecewise3(t23, 0.0, 10.0 / 9.0 * t158 * t53 - 10.0 / 3.0 * t161 * t104);
        let t168 = t7 * (t157 / 2.0 + t165 / 2.0);
        let t169 = t168 * t39;
        let t172 = t84 * t70;
        let t173 = t45 * t172;
        let tv2rho21 = 1.7984444444444445 * t90 + t95 + t98 + 1.7984444444444445 * t148 + 1.0790666666666666 * t45 * t169 + 1.0790666666666666 * t173 + 1.0790666666666666 * t128 + t144;
        v2rho2[ip * 3 + 1] += tv2rho21;

        let t177 = t74 * t74;
        let t181 = 2.0 * t47 + 2.0 * t105;
        let t185 = piecewise3(t14, 0.0, 10.0 / 9.0 * t99 * t177 + 5.0 / 3.0 * t19 * t181);
        let t186 = t78 * t78;
        let t189 = -t181;
        let t193 = piecewise3(t23, 0.0, 10.0 / 9.0 * t112 * t186 + 5.0 / 3.0 * t25 * t189);
        let t196 = t7 * (t185 / 2.0 + t193 / 2.0);
        let t197 = t196 * t39;
        let tv2rho22 = 3.596888888888889 * t148 + t95 + t98 + 1.0790666666666666 * t45 * t197 + 2.1581333333333332 * t173 + t144;
        v2rho2[ip * 3 + 2] += tv2rho22;

        let t201 = t122 * t31;
        let t203 = t8 * t201 * t39;
        let t205 = t58 * t33;
        let t207 = t8 * t205 * t39;
        let t210 = t8 * t88 * t70;
        let t213 = 1.0 / t30 / t10;
        let t214 = t29 * t213;
        let t217 = 0.3996543209876543 * t8 * t214 * t39;
        let t220 = 3.596888888888889 * t8 * t92 * t70;
        let t223 = 5.395333333333333 * t8 * t32 * t141;
        let t225 = 1.0 / t18 / t13;
        let t226 = t100 * t49;
        let t229 = t99 * t49;
        let t232 = t46 * t46;
        let t233 = 1.0 / t232;
        let t234 = t9 * t233;
        let t236 = 6.0 * t104 - 6.0 * t234;
        let t240 = piecewise3(t14, 0.0, -10.0 / 27.0 * t225 * t226 + 10.0 / 3.0 * t229 * t107 + 5.0 / 3.0 * t19 * t236);
        let t242 = 1.0 / t24 / t22;
        let t243 = t113 * t53;
        let t246 = t112 * t53;
        let t249 = -t236;
        let t253 = piecewise3(t23, 0.0, -10.0 / 27.0 * t242 * t243 + 10.0 / 3.0 * t246 * t116 + 5.0 / 3.0 * t25 * t249);
        let t255 = t240 / 2.0 + t253 / 2.0;
        let t256 = t7 * t255;
        let t261 = t45 * t123 * t70;
        let t264 = t45 * t59 * t141;
        let t277 = 1.0 / t31 / t103;
        let t279 = 1.0 / t137 / t35;
        let t282 = -0.000725925925925926 / t31 / t46 * t36 + 0.37037037037037035 * t104 * t67 - 170.06802721088437 / t30 / t103 * t138 + 19282.089252934733 * t277 * t279;
        let t285 = 1.0790666666666666 * t45 * t63 * t282;
        let tv3rho30 = 5.395333333333333 * t203 + 3.596888888888889 * t207 + 10.790666666666667 * t210 - t217 + t220 + t223 + 1.0790666666666666 * t45 * t256 * t39 + 3.2372 * t261 + 3.2372 * t264 + t285;
        v3rho3[ip * 4] += tv3rho30;

        let t290 = t33 * t2 * t5;
        let t291 = t290 * t85;
        let t294 = 3.596888888888889 * t147 * t169;
        let t295 = t147 * t172;
        let t297 = t225 * t74;
        let t300 = t99 * t9;
        let t311 = piecewise3(t14, 0.0, -10.0 / 27.0 * t297 * t100 + 40.0 / 9.0 * t300 * t104 * t49 + 10.0 / 9.0 * t150 * t107 + 10.0 / 3.0 * t19 * t104 - 10.0 * t153 * t233);
        let t312 = t242 * t78;
        let t315 = t112 * t9;
        let t326 = piecewise3(t23, 0.0, -10.0 / 27.0 * t312 * t113 - 40.0 / 9.0 * t315 * t104 * t53 + 10.0 / 9.0 * t158 * t116 - 10.0 / 3.0 * t25 * t104 + 10.0 * t161 * t233);
        let t329 = t7 * (t311 / 2.0 + t326 / 2.0);
        let t330 = t329 * t39;
        let t333 = t168 * t70;
        let t335 = 2.1581333333333332 * t45 * t333;
        let t336 = t84 * t141;
        let t337 = t45 * t336;
        let tv3rho31 = 1.7984444444444445 * t203 + 2.397925925925926 * t207 + 7.193777777777778 * t210 - t217 + t220 + t223 + 1.198962962962963 * t291 + t294 + 3.596888888888889 * t295 + 1.0790666666666666 * t45 * t330 + t335 + 1.0790666666666666 * t337 + 1.0790666666666666 * t261 + 2.1581333333333332 * t264 + t285;
        v3rho3[ip * 4 + 1] += tv3rho31;

        let t345 = t147 * t197;
        let t347 = t225 * t177;
        let t352 = t99 * t181;
        let t357 = -2.0 * t104 - 6.0 * t234;
        let t361 = piecewise3(t14, 0.0, -10.0 / 27.0 * t347 * t49 + 40.0 / 9.0 * t150 * t105 + 10.0 / 9.0 * t352 * t49 + 5.0 / 3.0 * t19 * t357);
        let t362 = t242 * t186;
        let t367 = t112 * t189;
        let t370 = -t357;
        let t374 = piecewise3(t23, 0.0, -10.0 / 27.0 * t362 * t53 - 40.0 / 9.0 * t158 * t105 + 10.0 / 9.0 * t367 * t53 + 5.0 / 3.0 * t25 * t370);
        let t377 = t7 * (t361 / 2.0 + t374 / 2.0);
        let t378 = t377 * t39;
        let t381 = t196 * t70;
        let t382 = t45 * t381;
        let tv3rho32 = 2.397925925925926 * t291 + t294 + 7.193777777777778 * t295 + 1.198962962962963 * t207 - t217 + t220 + 3.596888888888889 * t210 + t223 + 1.7984444444444445 * t345 + 1.0790666666666666 * t45 * t378 + 1.0790666666666666 * t382 + t335 + 2.1581333333333332 * t337 + 1.0790666666666666 * t264 + t285;
        v3rho3[ip * 4 + 2] += tv3rho32;

        let t389 = t177 * t74;
        let t395 = -6.0 * t104 - 6.0 * t234;
        let t399 = piecewise3(t14, 0.0, -10.0 / 27.0 * t225 * t389 + 10.0 / 3.0 * t150 * t181 + 5.0 / 3.0 * t19 * t395);
        let t400 = t186 * t78;
        let t405 = -t395;
        let t409 = piecewise3(t23, 0.0, -10.0 / 27.0 * t242 * t400 + 10.0 / 3.0 * t158 * t189 + 5.0 / 3.0 * t25 * t405);
        let t412 = t7 * (t399 / 2.0 + t409 / 2.0);
        let t413 = t412 * t39;
        let tv3rho33 = 3.596888888888889 * t291 + 5.395333333333333 * t345 + 10.790666666666667 * t295 - t217 + t220 + t223 + 1.0790666666666666 * t45 * t413 + 3.2372 * t382 + 3.2372 * t337 + t285;
        v3rho3[ip * 4 + 3] += tv3rho33;

    }
}

/// LDA_K_ZLP lxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_k_zlp_lxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = 1.0 / t4;
        let t7 = M_CBRT4;
        let t8 = t2 * t5 * t7;
        let t9 = rho[ip * 2] - rho[ip * 2 + 1];
        let t10 = rho[ip * 2] + rho[ip * 2 + 1];
        let t11 = 1.0 / t10;
        let t12 = t9 * t11;
        let t13 = 1.0 + t12;
        let t14 = t13 <= zeta_threshold;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t17 = t16 * zeta_threshold;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t14, t17, t19 * t13);
        let t22 = 1.0 - t12;
        let t23 = t22 <= zeta_threshold;
        let t24 = pow_1_3(t22);
        let t25 = t24 * t24;
        let t27 = piecewise3(t23, t17, t25 * t22);
        let t29 = t21 / 2.0 + t27 / 2.0;
        let t30 = pow_1_3(t10);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = 1.0 / t30;
        let t35 = 1.0 + 510.2040816326531 * t33;
        let t36 = f64::ln(t35);
        let t39 = 1.0 - 0.00196 * t30 * t36;
        let t41 = t8 * t32 * t39;
        let tzk0 = 1.0790666666666666 * t41;
        zk[ip] += tzk0;

        let t42 = 1.7984444444444445 * t41;
        let t43 = t31 * t10;
        let t45 = t43 * t2 * t5;
        let t46 = t10 * t10;
        let t47 = 1.0 / t46;
        let t48 = t9 * t47;
        let t49 = t11 - t48;
        let t52 = piecewise3(t14, 0.0, 5.0 / 3.0 * t19 * t49);
        let t53 = -t49;
        let t56 = piecewise3(t23, 0.0, 5.0 / 3.0 * t25 * t53);
        let t58 = t52 / 2.0 + t56 / 2.0;
        let t59 = t7 * t58;
        let t63 = t7 * t29;
        let t67 = 1.0 / t35;
        let t70 = -0.0006533333333333333 / t31 * t36 + 0.3333333333333333 * t11 * t67;
        let t73 = 1.0790666666666666 * t45 * t63 * t70;
        let tvrho0 = t42 + 1.0790666666666666 * t45 * t59 * t39 + t73;
        vrho[ip * 2] += tvrho0;

        let t74 = -t11 - t48;
        let t77 = piecewise3(t14, 0.0, 5.0 / 3.0 * t19 * t74);
        let t78 = -t74;
        let t81 = piecewise3(t23, 0.0, 5.0 / 3.0 * t25 * t78);
        let t84 = t7 * (t77 / 2.0 + t81 / 2.0);
        let t85 = t84 * t39;
        let tvrho1 = t42 + 1.0790666666666666 * t45 * t85 + t73;
        vrho[ip * 2 + 1] += tvrho1;

        let t88 = t58 * t31;
        let t90 = t8 * t88 * t39;
        let t92 = t29 * t33;
        let t95 = 1.198962962962963 * t8 * t92 * t39;
        let t98 = 3.596888888888889 * t8 * t32 * t70;
        let t99 = 1.0 / t18;
        let t100 = t49 * t49;
        let t103 = t46 * t10;
        let t104 = 1.0 / t103;
        let t105 = t9 * t104;
        let t107 = -2.0 * t47 + 2.0 * t105;
        let t111 = piecewise3(t14, 0.0, 10.0 / 9.0 * t99 * t100 + 5.0 / 3.0 * t19 * t107);
        let t112 = 1.0 / t24;
        let t113 = t53 * t53;
        let t116 = -t107;
        let t120 = piecewise3(t23, 0.0, 10.0 / 9.0 * t112 * t113 + 5.0 / 3.0 * t25 * t116);
        let t122 = t111 / 2.0 + t120 / 2.0;
        let t123 = t7 * t122;
        let t128 = t45 * t59 * t70;
        let t136 = 1.0 / t30 / t46;
        let t137 = t35 * t35;
        let t138 = 1.0 / t137;
        let t141 = 0.00043555555555555557 / t43 * t36 - 0.2222222222222222 * t47 * t67 + 56.68934240362812 * t136 * t138;
        let t144 = 1.0790666666666666 * t45 * t63 * t141;
        let tv2rho20 = 3.596888888888889 * t90 + t95 + t98 + 1.0790666666666666 * t45 * t123 * t39 + 2.1581333333333332 * t128 + t144;
        v2rho2[ip * 3] += tv2rho20;

        let t147 = t31 * t2 * t5;
        let t148 = t147 * t85;
        let t150 = t99 * t74;
        let t153 = t19 * t9;
        let t157 = piecewise3(t14, 0.0, 10.0 / 9.0 * t150 * t49 + 10.0 / 3.0 * t153 * t104);
        let t158 = t112 * t78;
        let t161 = t25 * t9;
        let t165 = piecewise3(t23, 0.0, 10.0 / 9.0 * t158 * t53 - 10.0 / 3.0 * t161 * t104);
        let t168 = t7 * (t157 / 2.0 + t165 / 2.0);
        let t169 = t168 * t39;
        let t172 = t84 * t70;
        let t173 = t45 * t172;
        let tv2rho21 = 1.7984444444444445 * t90 + t95 + t98 + 1.7984444444444445 * t148 + 1.0790666666666666 * t45 * t169 + 1.0790666666666666 * t173 + 1.0790666666666666 * t128 + t144;
        v2rho2[ip * 3 + 1] += tv2rho21;

        let t177 = t74 * t74;
        let t181 = 2.0 * t47 + 2.0 * t105;
        let t185 = piecewise3(t14, 0.0, 10.0 / 9.0 * t99 * t177 + 5.0 / 3.0 * t19 * t181);
        let t186 = t78 * t78;
        let t189 = -t181;
        let t193 = piecewise3(t23, 0.0, 10.0 / 9.0 * t112 * t186 + 5.0 / 3.0 * t25 * t189);
        let t196 = t7 * (t185 / 2.0 + t193 / 2.0);
        let t197 = t196 * t39;
        let tv2rho22 = 3.596888888888889 * t148 + t95 + t98 + 1.0790666666666666 * t45 * t197 + 2.1581333333333332 * t173 + t144;
        v2rho2[ip * 3 + 2] += tv2rho22;

        let t201 = t122 * t31;
        let t203 = t8 * t201 * t39;
        let t205 = t58 * t33;
        let t207 = t8 * t205 * t39;
        let t210 = t8 * t88 * t70;
        let t213 = 1.0 / t30 / t10;
        let t214 = t29 * t213;
        let t217 = 0.3996543209876543 * t8 * t214 * t39;
        let t220 = 3.596888888888889 * t8 * t92 * t70;
        let t223 = 5.395333333333333 * t8 * t32 * t141;
        let t225 = 1.0 / t18 / t13;
        let t226 = t100 * t49;
        let t229 = t99 * t49;
        let t232 = t46 * t46;
        let t233 = 1.0 / t232;
        let t234 = t9 * t233;
        let t236 = 6.0 * t104 - 6.0 * t234;
        let t240 = piecewise3(t14, 0.0, -10.0 / 27.0 * t225 * t226 + 10.0 / 3.0 * t229 * t107 + 5.0 / 3.0 * t19 * t236);
        let t242 = 1.0 / t24 / t22;
        let t243 = t113 * t53;
        let t246 = t112 * t53;
        let t249 = -t236;
        let t253 = piecewise3(t23, 0.0, -10.0 / 27.0 * t242 * t243 + 10.0 / 3.0 * t246 * t116 + 5.0 / 3.0 * t25 * t249);
        let t255 = t240 / 2.0 + t253 / 2.0;
        let t256 = t7 * t255;
        let t261 = t45 * t123 * t70;
        let t264 = t45 * t59 * t141;
        let t277 = 1.0 / t31 / t103;
        let t279 = 1.0 / t137 / t35;
        let t282 = -0.000725925925925926 / t31 / t46 * t36 + 0.37037037037037035 * t104 * t67 - 170.06802721088437 / t30 / t103 * t138 + 19282.089252934733 * t277 * t279;
        let t285 = 1.0790666666666666 * t45 * t63 * t282;
        let tv3rho30 = 5.395333333333333 * t203 + 3.596888888888889 * t207 + 10.790666666666667 * t210 - t217 + t220 + t223 + 1.0790666666666666 * t45 * t256 * t39 + 3.2372 * t261 + 3.2372 * t264 + t285;
        v3rho3[ip * 4] += tv3rho30;

        let t290 = t33 * t2 * t5;
        let t291 = t290 * t85;
        let t294 = 3.596888888888889 * t147 * t169;
        let t295 = t147 * t172;
        let t297 = t225 * t74;
        let t300 = t99 * t9;
        let t311 = piecewise3(t14, 0.0, -10.0 / 27.0 * t297 * t100 + 40.0 / 9.0 * t300 * t104 * t49 + 10.0 / 9.0 * t150 * t107 + 10.0 / 3.0 * t19 * t104 - 10.0 * t153 * t233);
        let t312 = t242 * t78;
        let t315 = t112 * t9;
        let t326 = piecewise3(t23, 0.0, -10.0 / 27.0 * t312 * t113 - 40.0 / 9.0 * t315 * t104 * t53 + 10.0 / 9.0 * t158 * t116 - 10.0 / 3.0 * t25 * t104 + 10.0 * t161 * t233);
        let t329 = t7 * (t311 / 2.0 + t326 / 2.0);
        let t330 = t329 * t39;
        let t333 = t168 * t70;
        let t335 = 2.1581333333333332 * t45 * t333;
        let t336 = t84 * t141;
        let t337 = t45 * t336;
        let tv3rho31 = 1.7984444444444445 * t203 + 2.397925925925926 * t207 + 7.193777777777778 * t210 - t217 + t220 + t223 + 1.198962962962963 * t291 + t294 + 3.596888888888889 * t295 + 1.0790666666666666 * t45 * t330 + t335 + 1.0790666666666666 * t337 + 1.0790666666666666 * t261 + 2.1581333333333332 * t264 + t285;
        v3rho3[ip * 4 + 1] += tv3rho31;

        let t345 = t147 * t197;
        let t347 = t225 * t177;
        let t352 = t99 * t181;
        let t357 = -2.0 * t104 - 6.0 * t234;
        let t361 = piecewise3(t14, 0.0, -10.0 / 27.0 * t347 * t49 + 40.0 / 9.0 * t150 * t105 + 10.0 / 9.0 * t352 * t49 + 5.0 / 3.0 * t19 * t357);
        let t362 = t242 * t186;
        let t367 = t112 * t189;
        let t370 = -t357;
        let t374 = piecewise3(t23, 0.0, -10.0 / 27.0 * t362 * t53 - 40.0 / 9.0 * t158 * t105 + 10.0 / 9.0 * t367 * t53 + 5.0 / 3.0 * t25 * t370);
        let t377 = t7 * (t361 / 2.0 + t374 / 2.0);
        let t378 = t377 * t39;
        let t381 = t196 * t70;
        let t382 = t45 * t381;
        let tv3rho32 = 2.397925925925926 * t291 + t294 + 7.193777777777778 * t295 + 1.198962962962963 * t207 - t217 + t220 + 3.596888888888889 * t210 + t223 + 1.7984444444444445 * t345 + 1.0790666666666666 * t45 * t378 + 1.0790666666666666 * t382 + t335 + 2.1581333333333332 * t337 + 1.0790666666666666 * t264 + t285;
        v3rho3[ip * 4 + 2] += tv3rho32;

        let t389 = t177 * t74;
        let t395 = -6.0 * t104 - 6.0 * t234;
        let t399 = piecewise3(t14, 0.0, -10.0 / 27.0 * t225 * t389 + 10.0 / 3.0 * t150 * t181 + 5.0 / 3.0 * t19 * t395);
        let t400 = t186 * t78;
        let t405 = -t395;
        let t409 = piecewise3(t23, 0.0, -10.0 / 27.0 * t242 * t400 + 10.0 / 3.0 * t158 * t189 + 5.0 / 3.0 * t25 * t405);
        let t412 = t7 * (t399 / 2.0 + t409 / 2.0);
        let t413 = t412 * t39;
        let tv3rho33 = 3.596888888888889 * t291 + 5.395333333333333 * t345 + 10.790666666666667 * t295 - t217 + t220 + t223 + 1.0790666666666666 * t45 * t413 + 3.2372 * t382 + 3.2372 * t337 + t285;
        v3rho3[ip * 4 + 3] += tv3rho33;

        let t420 = t8 * t122 * t33 * t39;
        let t424 = t8 * t58 * t213 * t39;
        let t429 = 0.5328724279835391 * t8 * t29 * t136 * t39;
        let t432 = t8 * t255 * t31 * t39;
        let t435 = t8 * t201 * t70;
        let t438 = t8 * t205 * t70;
        let t441 = t8 * t88 * t141;
        let t445 = 1.5986172839506172 * t8 * t214 * t70;
        let t448 = 7.193777777777778 * t8 * t92 * t141;
        let t451 = 7.193777777777778 * t8 * t32 * t282;
        let t452 = t13 * t13;
        let t454 = 1.0 / t18 / t452;
        let t455 = t100 * t100;
        let t461 = t107 * t107;
        let t467 = 1.0 / t232 / t10;
        let t468 = t9 * t467;
        let t470 = -24.0 * t233 + 24.0 * t468;
        let t474 = piecewise3(t14, 0.0, 40.0 / 81.0 * t454 * t455 - 20.0 / 9.0 * t225 * t100 * t107 + 10.0 / 3.0 * t99 * t461 + 40.0 / 9.0 * t229 * t236 + 5.0 / 3.0 * t19 * t470);
        let t475 = t22 * t22;
        let t477 = 1.0 / t24 / t475;
        let t478 = t113 * t113;
        let t484 = t116 * t116;
        let t493 = piecewise3(t23, 0.0, 40.0 / 81.0 * t477 * t478 - 20.0 / 9.0 * t242 * t113 * t116 + 10.0 / 3.0 * t112 * t484 + 40.0 / 9.0 * t246 * t249 - 5.0 / 3.0 * t25 * t470);
        let t501 = t45 * t256 * t70;
        let t504 = t45 * t123 * t141;
        let t507 = t45 * t59 * t282;
        let t521 = t137 * t137;
        let t528 = 1.0790666666666666 * t45 * t63 * (0.0019358024691358024 * t277 * t36 - 0.9876543209876543 * t233 * t67 + 629.8815822625346 / t30 / t232 * t138 - 128547.26168623156 / t31 / t232 * t279 + 9837800.639252415 * t467 / t521);
        let tv4rho40 = 7.193777777777778 * t420 - 1.5986172839506172 * t424 + t429 + 7.193777777777778 * t432 + 21.581333333333333 * t435 + 14.387555555555556 * t438 + 21.581333333333333 * t441 - t445 + t448 + t451 + 1.0790666666666666 * t45 * t7 * (t474 / 2.0 + t493 / 2.0) * t39 + 4.3162666666666665 * t501 + 6.4744 * t504 + 4.3162666666666665 * t507 + t528;
        v4rho4[ip * 5] += tv4rho40;

        let t533 = t213 * t2 * t5 * t85;
        let t540 = 3.596888888888889 * t420 - 1.198962962962963 * t424 + t429 - 0.3996543209876543 * t533 + 1.7984444444444445 * t432 + 10.790666666666667 * t435 + 10.790666666666667 * t438 + 16.186 * t441 - t445 + t448 + t451 + 1.0790666666666666 * t501;
        let t543 = t290 * t169;
        let t544 = 3.596888888888889 * t543;
        let t545 = t290 * t172;
        let t547 = t147 * t330;
        let t549 = t147 * t333;
        let t550 = 10.790666666666667 * t549;
        let t551 = t147 * t336;
        let t577 = 40.0 * t153 * t467;
        let t579 = piecewise3(t14, 0.0, 40.0 / 81.0 * t454 * t74 * t226 - 20.0 / 9.0 * t225 * t9 * t104 * t100 - 10.0 / 9.0 * t297 * t49 * t107 + 20.0 / 3.0 * t99 * t104 * t49 - 20.0 * t300 * t233 * t49 + 20.0 / 3.0 * t300 * t104 * t107 + 10.0 / 9.0 * t150 * t236 - 20.0 * t19 * t233 + t577);
        let t604 = 40.0 * t161 * t467;
        let t606 = piecewise3(t23, 0.0, 40.0 / 81.0 * t477 * t78 * t243 + 20.0 / 9.0 * t242 * t9 * t104 * t113 - 10.0 / 9.0 * t312 * t53 * t116 - 20.0 / 3.0 * t112 * t104 * t53 + 20.0 * t315 * t233 * t53 - 20.0 / 3.0 * t315 * t104 * t116 + 10.0 / 9.0 * t158 * t249 + 20.0 * t25 * t233 - t604);
        let t614 = t45 * t329 * t70;
        let t617 = t45 * t168 * t141;
        let t618 = 3.2372 * t617;
        let t620 = t45 * t84 * t282;
        let t622 = 3.2372 * t504 + 3.2372 * t507 + t528 + t544 + 3.596888888888889 * t545 + 5.395333333333333 * t547 + t550 + 5.395333333333333 * t551 + 1.0790666666666666 * t45 * t7 * (t579 / 2.0 + t606 / 2.0) * t39 + 3.2372 * t614 + t618 + 1.0790666666666666 * t620;
        let tv4rho41 = t540 + t622;
        v4rho4[ip * 5 + 1] += tv4rho41;

        let t631 = 1.198962962962963 * t420 - 0.7993086419753086 * t424 + t429 + 3.596888888888889 * t435 + 7.193777777777778 * t438 + 10.790666666666667 * t441 - t445 + t448 + t451 + 1.0790666666666666 * t504 + 2.1581333333333332 * t507 + t528 - 0.7993086419753086 * t533;
        let t640 = t290 * t197;
        let t642 = t147 * t378;
        let t644 = t147 * t381;
        let t655 = t9 * t9;
        let t658 = 1.0 / t232 / t46;
        let t674 = piecewise3(t14, 0.0, 40.0 / 81.0 * t454 * t177 * t100 - 80.0 / 27.0 * t297 * t49 * t9 * t104 - 10.0 / 27.0 * t347 * t107 + 80.0 / 9.0 * t99 * t655 * t658 + 40.0 / 9.0 * t150 * t104 - 40.0 / 3.0 * t150 * t234 - 10.0 / 27.0 * t225 * t181 * t100 + 20.0 / 9.0 * t99 * t357 * t49 + 10.0 / 9.0 * t352 * t107 + t577);
        let t700 = piecewise3(t23, 0.0, 40.0 / 81.0 * t477 * t186 * t113 + 80.0 / 27.0 * t312 * t53 * t9 * t104 - 10.0 / 27.0 * t362 * t116 + 80.0 / 9.0 * t112 * t655 * t658 - 40.0 / 9.0 * t158 * t104 + 40.0 / 3.0 * t158 * t234 - 10.0 / 27.0 * t242 * t189 * t113 + 20.0 / 9.0 * t112 * t370 * t53 + 10.0 / 9.0 * t367 * t116 - t604);
        let t708 = t45 * t377 * t70;
        let t711 = t45 * t196 * t141;
        let t713 = 4.795851851851852 * t543 + 7.193777777777778 * t545 + 3.596888888888889 * t547 + 14.387555555555556 * t549 + 10.790666666666667 * t551 + 2.1581333333333332 * t614 + 4.3162666666666665 * t617 + 2.1581333333333332 * t620 + 1.198962962962963 * t640 + 3.596888888888889 * t642 + 3.596888888888889 * t644 + 1.0790666666666666 * t45 * t7 * (t674 / 2.0 + t700 / 2.0) * t39 + 2.1581333333333332 * t708 + 1.0790666666666666 * t711;
        let tv4rho42 = t631 + t713;
        v4rho4[ip * 5 + 2] += tv4rho42;

        let t720 = -0.3996543209876543 * t424 + t429 - 1.198962962962963 * t533 + 3.596888888888889 * t640 + 3.596888888888889 * t438 + 5.395333333333333 * t441 - t445 + t448 + t451 + 1.0790666666666666 * t507 + t528 + t544;
        let t746 = 12.0 * t233 + 24.0 * t468;
        let t750 = piecewise3(t14, 0.0, 40.0 / 81.0 * t454 * t389 * t49 - 20.0 / 9.0 * t347 * t105 - 10.0 / 9.0 * t297 * t181 * t49 + 20.0 / 3.0 * t300 * t104 * t181 + 10.0 / 3.0 * t150 * t357 + 10.0 / 9.0 * t99 * t395 * t49 + 5.0 / 3.0 * t19 * t746);
        let t771 = piecewise3(t23, 0.0, 40.0 / 81.0 * t477 * t400 * t53 + 20.0 / 9.0 * t362 * t105 - 10.0 / 9.0 * t312 * t189 * t53 - 20.0 / 3.0 * t315 * t104 * t189 + 10.0 / 3.0 * t158 * t370 + 10.0 / 9.0 * t112 * t405 * t53 - 5.0 / 3.0 * t25 * t746);
        let t779 = t45 * t412 * t70;
        let t781 = t147 * t413;
        let t783 = 10.790666666666667 * t545 + t550 + 16.186 * t551 + t618 + 3.2372 * t620 + 5.395333333333333 * t642 + 10.790666666666667 * t644 + 3.2372 * t708 + 3.2372 * t711 + 1.0790666666666666 * t45 * t7 * (t750 / 2.0 + t771 / 2.0) * t39 + 1.0790666666666666 * t779 + 1.7984444444444445 * t781;
        let tv4rho43 = t720 + t783;
        v4rho4[ip * 5 + 3] += tv4rho43;

        let t792 = t177 * t177;
        let t797 = t181 * t181;
        let t803 = 24.0 * t233 + 24.0 * t468;
        let t807 = piecewise3(t14, 0.0, 40.0 / 81.0 * t454 * t792 - 20.0 / 9.0 * t347 * t181 + 10.0 / 3.0 * t99 * t797 + 40.0 / 9.0 * t150 * t395 + 5.0 / 3.0 * t19 * t803);
        let t808 = t186 * t186;
        let t813 = t189 * t189;
        let t822 = piecewise3(t23, 0.0, 40.0 / 81.0 * t477 * t808 - 20.0 / 9.0 * t362 * t189 + 10.0 / 3.0 * t112 * t813 + 40.0 / 9.0 * t158 * t405 - 5.0 / 3.0 * t25 * t803);
        let tv4rho44 = t429 - 1.5986172839506172 * t533 + 7.193777777777778 * t640 - t445 + t448 + t451 + t528 + 14.387555555555556 * t545 + 21.581333333333333 * t551 + 4.3162666666666665 * t620 + 21.581333333333333 * t644 + 6.4744 * t711 + 4.3162666666666665 * t779 + 1.0790666666666666 * t45 * t7 * (t807 / 2.0 + t822 / 2.0) * t39 + 7.193777777777778 * t781;
        v4rho4[ip * 5 + 4] += tv4rho44;

    }
}
