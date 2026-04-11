//! LDA_C_PZ kernel functions translated from libxc maple2c.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pz.c`.
//! Translation preserves exact maple2c variable names and operation order.

#[allow(unused_variables, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_PI, M_CBRT2, M_CBRT3, M_CBRT4};
use crate::math::powers::{pow_1_3};
use crate::math::piecewise::{piecewise3};

// ============================================================================
// UNPOLARIZED FUNCTIONS
// ============================================================================

/// LDA_C_PZ exc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_c_pz_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_c: f64,
    param_d: f64,
    param_gamma: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t1 * t3 * t9;
        let t11 = t10 / 4.0;
        let t12 = 1.0 <= t11;
        let t13 = param_gamma[0];
        let t14 = param_beta1[0];
        let t15 = f64::sqrt(t10);
        let t19 = param_beta2[0] * t1;
        let t20 = t3 * t6;
        let t21 = t20 * t8;
        let t24 = 1.0 + t14 * t15 / 2.0 + t19 * t21 / 4.0;
        let t27 = param_a[0];
        let t28 = f64::ln(t11);
        let t32 = param_c[0] * t1;
        let t33 = t32 * t3;
        let t34 = t9 * t28;
        let t38 = param_d[0] * t1;
        let t42 = piecewise3(t12, t13 / t24, t27 * t28 + param_b[0] + t33 * t34 / 4.0 + t38 * t21 / 4.0);
        let t43 = param_gamma[1];
        let t44 = param_beta1[1];
        let t48 = param_beta2[1] * t1;
        let t51 = 1.0 + t44 * t15 / 2.0 + t48 * t21 / 4.0;
        let t54 = param_a[1];
        let t58 = param_c[1] * t1;
        let t59 = t58 * t3;
        let t63 = param_d[1] * t1;
        let t67 = piecewise3(t12, t43 / t51, t54 * t28 + param_b[1] + t59 * t34 / 4.0 + t63 * t21 / 4.0);
        let t70 = pow_1_3(zeta_threshold);
        let t72 = piecewise3(1.0 <= zeta_threshold, t70 * zeta_threshold, 1.0);
        let t74 = 2.0 * t72 - 2.0;
        let t76 = M_CBRT2;
        let t79 = 1.0 / (2.0 * t76 - 2.0);
        let t80 = (t67 - t42) * t74 * t79;
        let tzk0 = t42 + t80;
        zk[ip] += tzk0;

    }
}

/// LDA_C_PZ vxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_c_pz_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_c: f64,
    param_d: f64,
    param_gamma: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t1 * t3 * t9;
        let t11 = t10 / 4.0;
        let t12 = 1.0 <= t11;
        let t13 = param_gamma[0];
        let t14 = param_beta1[0];
        let t15 = f64::sqrt(t10);
        let t19 = param_beta2[0] * t1;
        let t20 = t3 * t6;
        let t21 = t20 * t8;
        let t24 = 1.0 + t14 * t15 / 2.0 + t19 * t21 / 4.0;
        let t27 = param_a[0];
        let t28 = f64::ln(t11);
        let t32 = param_c[0] * t1;
        let t33 = t32 * t3;
        let t34 = t9 * t28;
        let t38 = param_d[0] * t1;
        let t42 = piecewise3(t12, t13 / t24, t27 * t28 + param_b[0] + t33 * t34 / 4.0 + t38 * t21 / 4.0);
        let t43 = param_gamma[1];
        let t44 = param_beta1[1];
        let t48 = param_beta2[1] * t1;
        let t51 = 1.0 + t44 * t15 / 2.0 + t48 * t21 / 4.0;
        let t54 = param_a[1];
        let t58 = param_c[1] * t1;
        let t59 = t58 * t3;
        let t63 = param_d[1] * t1;
        let t67 = piecewise3(t12, t43 / t51, t54 * t28 + param_b[1] + t59 * t34 / 4.0 + t63 * t21 / 4.0);
        let t70 = pow_1_3(zeta_threshold);
        let t72 = piecewise3(1.0 <= zeta_threshold, t70 * zeta_threshold, 1.0);
        let t74 = 2.0 * t72 - 2.0;
        let t76 = M_CBRT2;
        let t79 = 1.0 / (2.0 * t76 - 2.0);
        let t80 = (t67 - t42) * t74 * t79;
        let tzk0 = t42 + t80;
        zk[ip] += tzk0;

        let t81 = t24 * t24;
        let t83 = t13 / t81;
        let t84 = 1.0 / t15;
        let t86 = t14 * t84 * t1;
        let t88 = 1.0 / t7 / rho[ip];
        let t89 = t20 * t88;
        let t93 = -t19 * t89 / 12.0 - t86 * t89 / 12.0;
        let t95 = 1.0 / rho[ip];
        let t99 = t6 * t88 * t28;
        let t107 = piecewise3(t12, -t83 * t93, -t27 * t95 / 3.0 - t33 * t99 / 12.0 - t32 * t89 / 12.0 - t38 * t89 / 12.0);
        let t108 = t51 * t51;
        let t110 = t43 / t108;
        let t112 = t44 * t84 * t1;
        let t116 = -t112 * t89 / 12.0 - t48 * t89 / 12.0;
        let t127 = piecewise3(t12, -t110 * t116, -t54 * t95 / 3.0 - t59 * t99 / 12.0 - t58 * t89 / 12.0 - t63 * t89 / 12.0);
        let t130 = (t127 - t107) * t74 * t79;
        let tvrho0 = t42 + t80 + rho[ip] * (t107 + t130);
        vrho[ip] += tvrho0;

    }
}

/// LDA_C_PZ fxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_c_pz_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_c: f64,
    param_d: f64,
    param_gamma: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t1 * t3 * t9;
        let t11 = t10 / 4.0;
        let t12 = 1.0 <= t11;
        let t13 = param_gamma[0];
        let t14 = param_beta1[0];
        let t15 = f64::sqrt(t10);
        let t19 = param_beta2[0] * t1;
        let t20 = t3 * t6;
        let t21 = t20 * t8;
        let t24 = 1.0 + t14 * t15 / 2.0 + t19 * t21 / 4.0;
        let t27 = param_a[0];
        let t28 = f64::ln(t11);
        let t32 = param_c[0] * t1;
        let t33 = t32 * t3;
        let t34 = t9 * t28;
        let t38 = param_d[0] * t1;
        let t42 = piecewise3(t12, t13 / t24, t27 * t28 + param_b[0] + t33 * t34 / 4.0 + t38 * t21 / 4.0);
        let t43 = param_gamma[1];
        let t44 = param_beta1[1];
        let t48 = param_beta2[1] * t1;
        let t51 = 1.0 + t44 * t15 / 2.0 + t48 * t21 / 4.0;
        let t54 = param_a[1];
        let t58 = param_c[1] * t1;
        let t59 = t58 * t3;
        let t63 = param_d[1] * t1;
        let t67 = piecewise3(t12, t43 / t51, t54 * t28 + param_b[1] + t59 * t34 / 4.0 + t63 * t21 / 4.0);
        let t70 = pow_1_3(zeta_threshold);
        let t72 = piecewise3(1.0 <= zeta_threshold, t70 * zeta_threshold, 1.0);
        let t74 = 2.0 * t72 - 2.0;
        let t76 = M_CBRT2;
        let t79 = 1.0 / (2.0 * t76 - 2.0);
        let t80 = (t67 - t42) * t74 * t79;
        let tzk0 = t42 + t80;
        zk[ip] += tzk0;

        let t81 = t24 * t24;
        let t83 = t13 / t81;
        let t84 = 1.0 / t15;
        let t86 = t14 * t84 * t1;
        let t88 = 1.0 / t7 / rho[ip];
        let t89 = t20 * t88;
        let t93 = -t19 * t89 / 12.0 - t86 * t89 / 12.0;
        let t95 = 1.0 / rho[ip];
        let t99 = t6 * t88 * t28;
        let t107 = piecewise3(t12, -t83 * t93, -t27 * t95 / 3.0 - t33 * t99 / 12.0 - t32 * t89 / 12.0 - t38 * t89 / 12.0);
        let t108 = t51 * t51;
        let t110 = t43 / t108;
        let t112 = t44 * t84 * t1;
        let t116 = -t112 * t89 / 12.0 - t48 * t89 / 12.0;
        let t127 = piecewise3(t12, -t110 * t116, -t54 * t95 / 3.0 - t59 * t99 / 12.0 - t58 * t89 / 12.0 - t63 * t89 / 12.0);
        let t130 = (t127 - t107) * t74 * t79;
        let tvrho0 = t42 + t80 + rho[ip] * (t107 + t130);
        vrho[ip] += tvrho0;

        let t137 = t13 / t81 / t24;
        let t138 = t93 * t93;
        let t142 = 1.0 / t15 / t10;
        let t144 = t1 * t1;
        let t145 = t14 * t142 * t144;
        let t146 = t3 * t3;
        let t147 = t146 * t5;
        let t148 = rho[ip] * rho[ip];
        let t149 = t7 * t7;
        let t152 = t147 / t149 / t148;
        let t156 = 1.0 / t7 / t148;
        let t157 = t20 * t156;
        let t162 = -t145 * t152 / 18.0 + t86 * t157 / 9.0 + t19 * t157 / 9.0;
        let t165 = 1.0 / t148;
        let t169 = t6 * t156 * t28;
        let t177 = piecewise3(t12, 2.0 * t137 * t138 - t83 * t162, t27 * t165 / 3.0 + t33 * t169 / 9.0 + 5.0 / 36.0 * t32 * t157 + t38 * t157 / 9.0);
        let t180 = t43 / t108 / t51;
        let t181 = t116 * t116;
        let t185 = t44 * t142 * t144;
        let t192 = -t185 * t152 / 18.0 + t112 * t157 / 9.0 + t48 * t157 / 9.0;
        let t204 = piecewise3(t12, -t110 * t192 + 2.0 * t180 * t181, t54 * t165 / 3.0 + t59 * t169 / 9.0 + 5.0 / 36.0 * t58 * t157 + t63 * t157 / 9.0);
        let t207 = (t204 - t177) * t74 * t79;
        let tv2rho20 = 2.0 * t107 + 2.0 * t130 + rho[ip] * (t177 + t207);
        v2rho2[ip] += tv2rho20;

    }
}

/// LDA_C_PZ kxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_c_pz_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_c: f64,
    param_d: f64,
    param_gamma: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t1 * t3 * t9;
        let t11 = t10 / 4.0;
        let t12 = 1.0 <= t11;
        let t13 = param_gamma[0];
        let t14 = param_beta1[0];
        let t15 = f64::sqrt(t10);
        let t19 = param_beta2[0] * t1;
        let t20 = t3 * t6;
        let t21 = t20 * t8;
        let t24 = 1.0 + t14 * t15 / 2.0 + t19 * t21 / 4.0;
        let t27 = param_a[0];
        let t28 = f64::ln(t11);
        let t32 = param_c[0] * t1;
        let t33 = t32 * t3;
        let t34 = t9 * t28;
        let t38 = param_d[0] * t1;
        let t42 = piecewise3(t12, t13 / t24, t27 * t28 + param_b[0] + t33 * t34 / 4.0 + t38 * t21 / 4.0);
        let t43 = param_gamma[1];
        let t44 = param_beta1[1];
        let t48 = param_beta2[1] * t1;
        let t51 = 1.0 + t44 * t15 / 2.0 + t48 * t21 / 4.0;
        let t54 = param_a[1];
        let t58 = param_c[1] * t1;
        let t59 = t58 * t3;
        let t63 = param_d[1] * t1;
        let t67 = piecewise3(t12, t43 / t51, t54 * t28 + param_b[1] + t59 * t34 / 4.0 + t63 * t21 / 4.0);
        let t70 = pow_1_3(zeta_threshold);
        let t72 = piecewise3(1.0 <= zeta_threshold, t70 * zeta_threshold, 1.0);
        let t74 = 2.0 * t72 - 2.0;
        let t76 = M_CBRT2;
        let t79 = 1.0 / (2.0 * t76 - 2.0);
        let t80 = (t67 - t42) * t74 * t79;
        let tzk0 = t42 + t80;
        zk[ip] += tzk0;

        let t81 = t24 * t24;
        let t83 = t13 / t81;
        let t84 = 1.0 / t15;
        let t86 = t14 * t84 * t1;
        let t88 = 1.0 / t7 / rho[ip];
        let t89 = t20 * t88;
        let t93 = -t19 * t89 / 12.0 - t86 * t89 / 12.0;
        let t95 = 1.0 / rho[ip];
        let t99 = t6 * t88 * t28;
        let t107 = piecewise3(t12, -t83 * t93, -t27 * t95 / 3.0 - t33 * t99 / 12.0 - t32 * t89 / 12.0 - t38 * t89 / 12.0);
        let t108 = t51 * t51;
        let t110 = t43 / t108;
        let t112 = t44 * t84 * t1;
        let t116 = -t112 * t89 / 12.0 - t48 * t89 / 12.0;
        let t127 = piecewise3(t12, -t110 * t116, -t54 * t95 / 3.0 - t59 * t99 / 12.0 - t58 * t89 / 12.0 - t63 * t89 / 12.0);
        let t130 = (t127 - t107) * t74 * t79;
        let tvrho0 = t42 + t80 + rho[ip] * (t107 + t130);
        vrho[ip] += tvrho0;

        let t137 = t13 / t81 / t24;
        let t138 = t93 * t93;
        let t142 = 1.0 / t15 / t10;
        let t144 = t1 * t1;
        let t145 = t14 * t142 * t144;
        let t146 = t3 * t3;
        let t147 = t146 * t5;
        let t148 = rho[ip] * rho[ip];
        let t149 = t7 * t7;
        let t152 = t147 / t149 / t148;
        let t156 = 1.0 / t7 / t148;
        let t157 = t20 * t156;
        let t162 = -t145 * t152 / 18.0 + t86 * t157 / 9.0 + t19 * t157 / 9.0;
        let t165 = 1.0 / t148;
        let t169 = t6 * t156 * t28;
        let t177 = piecewise3(t12, 2.0 * t137 * t138 - t83 * t162, t27 * t165 / 3.0 + t33 * t169 / 9.0 + 5.0 / 36.0 * t32 * t157 + t38 * t157 / 9.0);
        let t180 = t43 / t108 / t51;
        let t181 = t116 * t116;
        let t185 = t44 * t142 * t144;
        let t192 = -t185 * t152 / 18.0 + t112 * t157 / 9.0 + t48 * t157 / 9.0;
        let t204 = piecewise3(t12, -t110 * t192 + 2.0 * t180 * t181, t54 * t165 / 3.0 + t59 * t169 / 9.0 + 5.0 / 36.0 * t58 * t157 + t63 * t157 / 9.0);
        let t207 = (t204 - t177) * t74 * t79;
        let tv2rho20 = 2.0 * t107 + 2.0 * t130 + rho[ip] * (t177 + t207);
        v2rho2[ip] += tv2rho20;

        let t212 = t81 * t81;
        let t214 = t13 / t212;
        let t227 = 1.0 / t15 / t144 / t146 / t5 * t149 / 4.0;
        let t228 = t14 * t227;
        let t229 = t148 * t148;
        let t230 = 1.0 / t229;
        let t231 = t2 * t230;
        let t234 = t148 * rho[ip];
        let t237 = t147 / t149 / t234;
        let t241 = 1.0 / t7 / t234;
        let t242 = t20 * t241;
        let t247 = -t228 * t231 / 3.0 + 2.0 / 9.0 * t145 * t237 - 7.0 / 27.0 * t86 * t242 - 7.0 / 27.0 * t19 * t242;
        let t250 = 1.0 / t234;
        let t254 = t6 * t241 * t28;
        let t262 = piecewise3(t12, 6.0 * t137 * t93 * t162 - 6.0 * t214 * t138 * t93 - t83 * t247, -2.0 / 3.0 * t27 * t250 - 7.0 / 27.0 * t33 * t254 - 13.0 / 36.0 * t32 * t242 - 7.0 / 27.0 * t38 * t242);
        let t263 = t108 * t108;
        let t265 = t43 / t263;
        let t272 = t44 * t227;
        let t281 = -t272 * t231 / 3.0 + 2.0 / 9.0 * t185 * t237 - 7.0 / 27.0 * t112 * t242 - 7.0 / 27.0 * t48 * t242;
        let t293 = piecewise3(t12, 6.0 * t180 * t116 * t192 - 6.0 * t265 * t181 * t116 - t110 * t281, -2.0 / 3.0 * t54 * t250 - 7.0 / 27.0 * t59 * t254 - 13.0 / 36.0 * t58 * t242 - 7.0 / 27.0 * t63 * t242);
        let t296 = (t293 - t262) * t74 * t79;
        let tv3rho30 = 3.0 * t177 + 3.0 * t207 + rho[ip] * (t262 + t296);
        v3rho3[ip] += tv3rho30;

    }
}

/// LDA_C_PZ lxc -- unpolarized.
#[cube(launch_unchecked)]
pub fn lda_c_pz_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_c: f64,
    param_d: f64,
    param_gamma: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t1 * t3 * t9;
        let t11 = t10 / 4.0;
        let t12 = 1.0 <= t11;
        let t13 = param_gamma[0];
        let t14 = param_beta1[0];
        let t15 = f64::sqrt(t10);
        let t19 = param_beta2[0] * t1;
        let t20 = t3 * t6;
        let t21 = t20 * t8;
        let t24 = 1.0 + t14 * t15 / 2.0 + t19 * t21 / 4.0;
        let t27 = param_a[0];
        let t28 = f64::ln(t11);
        let t32 = param_c[0] * t1;
        let t33 = t32 * t3;
        let t34 = t9 * t28;
        let t38 = param_d[0] * t1;
        let t42 = piecewise3(t12, t13 / t24, t27 * t28 + param_b[0] + t33 * t34 / 4.0 + t38 * t21 / 4.0);
        let t43 = param_gamma[1];
        let t44 = param_beta1[1];
        let t48 = param_beta2[1] * t1;
        let t51 = 1.0 + t44 * t15 / 2.0 + t48 * t21 / 4.0;
        let t54 = param_a[1];
        let t58 = param_c[1] * t1;
        let t59 = t58 * t3;
        let t63 = param_d[1] * t1;
        let t67 = piecewise3(t12, t43 / t51, t54 * t28 + param_b[1] + t59 * t34 / 4.0 + t63 * t21 / 4.0);
        let t70 = pow_1_3(zeta_threshold);
        let t72 = piecewise3(1.0 <= zeta_threshold, t70 * zeta_threshold, 1.0);
        let t74 = 2.0 * t72 - 2.0;
        let t76 = M_CBRT2;
        let t79 = 1.0 / (2.0 * t76 - 2.0);
        let t80 = (t67 - t42) * t74 * t79;
        let tzk0 = t42 + t80;
        zk[ip] += tzk0;

        let t81 = t24 * t24;
        let t83 = t13 / t81;
        let t84 = 1.0 / t15;
        let t86 = t14 * t84 * t1;
        let t88 = 1.0 / t7 / rho[ip];
        let t89 = t20 * t88;
        let t93 = -t19 * t89 / 12.0 - t86 * t89 / 12.0;
        let t95 = 1.0 / rho[ip];
        let t99 = t6 * t88 * t28;
        let t107 = piecewise3(t12, -t83 * t93, -t27 * t95 / 3.0 - t33 * t99 / 12.0 - t32 * t89 / 12.0 - t38 * t89 / 12.0);
        let t108 = t51 * t51;
        let t110 = t43 / t108;
        let t112 = t44 * t84 * t1;
        let t116 = -t112 * t89 / 12.0 - t48 * t89 / 12.0;
        let t127 = piecewise3(t12, -t110 * t116, -t54 * t95 / 3.0 - t59 * t99 / 12.0 - t58 * t89 / 12.0 - t63 * t89 / 12.0);
        let t130 = (t127 - t107) * t74 * t79;
        let tvrho0 = t42 + t80 + rho[ip] * (t107 + t130);
        vrho[ip] += tvrho0;

        let t137 = t13 / t81 / t24;
        let t138 = t93 * t93;
        let t142 = 1.0 / t15 / t10;
        let t144 = t1 * t1;
        let t145 = t14 * t142 * t144;
        let t146 = t3 * t3;
        let t147 = t146 * t5;
        let t148 = rho[ip] * rho[ip];
        let t149 = t7 * t7;
        let t152 = t147 / t149 / t148;
        let t156 = 1.0 / t7 / t148;
        let t157 = t20 * t156;
        let t162 = -t145 * t152 / 18.0 + t86 * t157 / 9.0 + t19 * t157 / 9.0;
        let t165 = 1.0 / t148;
        let t169 = t6 * t156 * t28;
        let t177 = piecewise3(t12, 2.0 * t137 * t138 - t83 * t162, t27 * t165 / 3.0 + t33 * t169 / 9.0 + 5.0 / 36.0 * t32 * t157 + t38 * t157 / 9.0);
        let t180 = t43 / t108 / t51;
        let t181 = t116 * t116;
        let t185 = t44 * t142 * t144;
        let t192 = -t185 * t152 / 18.0 + t112 * t157 / 9.0 + t48 * t157 / 9.0;
        let t204 = piecewise3(t12, -t110 * t192 + 2.0 * t180 * t181, t54 * t165 / 3.0 + t59 * t169 / 9.0 + 5.0 / 36.0 * t58 * t157 + t63 * t157 / 9.0);
        let t207 = (t204 - t177) * t74 * t79;
        let tv2rho20 = 2.0 * t107 + 2.0 * t130 + rho[ip] * (t177 + t207);
        v2rho2[ip] += tv2rho20;

        let t212 = t81 * t81;
        let t214 = t13 / t212;
        let t227 = 1.0 / t15 / t144 / t146 / t5 * t149 / 4.0;
        let t228 = t14 * t227;
        let t229 = t148 * t148;
        let t230 = 1.0 / t229;
        let t231 = t2 * t230;
        let t234 = t148 * rho[ip];
        let t237 = t147 / t149 / t234;
        let t241 = 1.0 / t7 / t234;
        let t242 = t20 * t241;
        let t247 = -t228 * t231 / 3.0 + 2.0 / 9.0 * t145 * t237 - 7.0 / 27.0 * t86 * t242 - 7.0 / 27.0 * t19 * t242;
        let t250 = 1.0 / t234;
        let t254 = t6 * t241 * t28;
        let t262 = piecewise3(t12, 6.0 * t137 * t93 * t162 - 6.0 * t214 * t138 * t93 - t83 * t247, -2.0 / 3.0 * t27 * t250 - 7.0 / 27.0 * t33 * t254 - 13.0 / 36.0 * t32 * t242 - 7.0 / 27.0 * t38 * t242);
        let t263 = t108 * t108;
        let t265 = t43 / t263;
        let t272 = t44 * t227;
        let t281 = -t272 * t231 / 3.0 + 2.0 / 9.0 * t185 * t237 - 7.0 / 27.0 * t112 * t242 - 7.0 / 27.0 * t48 * t242;
        let t293 = piecewise3(t12, 6.0 * t180 * t116 * t192 - 6.0 * t265 * t181 * t116 - t110 * t281, -2.0 / 3.0 * t54 * t250 - 7.0 / 27.0 * t59 * t254 - 13.0 / 36.0 * t58 * t242 - 7.0 / 27.0 * t63 * t242);
        let t296 = (t293 - t262) * t74 * t79;
        let tv3rho30 = 3.0 * t177 + 3.0 * t207 + rho[ip] * (t262 + t296);
        v3rho3[ip] += tv3rho30;

        let t304 = t138 * t138;
        let t310 = t162 * t162;
        let t319 = 1.0 / t15 / t2 / t95 / 48.0;
        let t322 = t229 * rho[ip];
        let t326 = 1.0 / t7 / t322 * t1 * t20;
        let t330 = t2 / t322;
        let t335 = t147 / t149 / t229;
        let t339 = 1.0 / t7 / t229;
        let t340 = t20 * t339;
        let t351 = t6 * t339 * t28;
        let t359 = piecewise3(t12, 24.0 * t13 / t212 / t24 * t304 - 36.0 * t214 * t138 * t162 + 6.0 * t137 * t310 + 8.0 * t137 * t93 * t247 - t83 * (-5.0 / 18.0 * t14 * t319 * t2 * t326 + 8.0 / 3.0 * t228 * t330 - 80.0 / 81.0 * t145 * t335 + 70.0 / 81.0 * t86 * t340 + 70.0 / 81.0 * t19 * t340), 2.0 * t27 * t230 + 70.0 / 81.0 * t33 * t351 + 209.0 / 162.0 * t32 * t340 + 70.0 / 81.0 * t38 * t340);
        let t363 = t181 * t181;
        let t369 = t192 * t192;
        let t399 = piecewise3(t12, 24.0 * t43 / t263 / t51 * t363 - 36.0 * t265 * t181 * t192 + 6.0 * t180 * t369 + 8.0 * t180 * t116 * t281 - t110 * (-5.0 / 18.0 * t44 * t319 * t2 * t326 + 8.0 / 3.0 * t272 * t330 - 80.0 / 81.0 * t185 * t335 + 70.0 / 81.0 * t112 * t340 + 70.0 / 81.0 * t48 * t340), 2.0 * t54 * t230 + 70.0 / 81.0 * t59 * t351 + 209.0 / 162.0 * t58 * t340 + 70.0 / 81.0 * t63 * t340);
        let tv4rho40 = 4.0 * t262 + 4.0 * t296 + rho[ip] * (t359 + (t399 - t359) * t74 * t79);
        v4rho4[ip] += tv4rho40;

    }
}

// ============================================================================
// POLARIZED FUNCTIONS
// ============================================================================

/// LDA_C_PZ exc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_c_pz_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_c: f64,
    param_d: f64,
    param_gamma: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho[ip * 2] + rho[ip * 2 + 1];
        let t8 = pow_1_3(t7);
        let t9 = 1.0 / t8;
        let t10 = t6 * t9;
        let t11 = t1 * t3 * t10;
        let t12 = t11 / 4.0;
        let t13 = 1.0 <= t12;
        let t14 = param_gamma[0];
        let t15 = param_beta1[0];
        let t16 = f64::sqrt(t11);
        let t20 = param_beta2[0] * t1;
        let t21 = t3 * t6;
        let t22 = t21 * t9;
        let t25 = 1.0 + t15 * t16 / 2.0 + t20 * t22 / 4.0;
        let t28 = param_a[0];
        let t29 = f64::ln(t12);
        let t33 = param_c[0] * t1;
        let t34 = t33 * t3;
        let t35 = t10 * t29;
        let t39 = param_d[0] * t1;
        let t43 = piecewise3(t13, t14 / t25, t28 * t29 + param_b[0] + t34 * t35 / 4.0 + t39 * t22 / 4.0);
        let t44 = param_gamma[1];
        let t45 = param_beta1[1];
        let t49 = param_beta2[1] * t1;
        let t52 = 1.0 + t45 * t16 / 2.0 + t49 * t22 / 4.0;
        let t55 = param_a[1];
        let t59 = param_c[1] * t1;
        let t60 = t59 * t3;
        let t64 = param_d[1] * t1;
        let t68 = piecewise3(t13, t44 / t52, t55 * t29 + param_b[1] + t60 * t35 / 4.0 + t64 * t22 / 4.0);
        let t69 = t68 - t43;
        let t70 = rho[ip * 2] - rho[ip * 2 + 1];
        let t71 = 1.0 / t7;
        let t72 = t70 * t71;
        let t73 = 1.0 + t72;
        let t74 = t73 <= zeta_threshold;
        let t75 = pow_1_3(zeta_threshold);
        let t76 = t75 * zeta_threshold;
        let t77 = pow_1_3(t73);
        let t79 = piecewise3(t74, t76, t77 * t73);
        let t80 = 1.0 - t72;
        let t81 = t80 <= zeta_threshold;
        let t82 = pow_1_3(t80);
        let t84 = piecewise3(t81, t76, t82 * t80);
        let t85 = t79 + t84 - 2.0;
        let t87 = M_CBRT2;
        let t90 = 1.0 / (2.0 * t87 - 2.0);
        let t91 = t69 * t85 * t90;
        let tzk0 = t43 + t91;
        zk[ip] += tzk0;

    }
}

/// LDA_C_PZ vxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_c_pz_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_c: f64,
    param_d: f64,
    param_gamma: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho[ip * 2] + rho[ip * 2 + 1];
        let t8 = pow_1_3(t7);
        let t9 = 1.0 / t8;
        let t10 = t6 * t9;
        let t11 = t1 * t3 * t10;
        let t12 = t11 / 4.0;
        let t13 = 1.0 <= t12;
        let t14 = param_gamma[0];
        let t15 = param_beta1[0];
        let t16 = f64::sqrt(t11);
        let t20 = param_beta2[0] * t1;
        let t21 = t3 * t6;
        let t22 = t21 * t9;
        let t25 = 1.0 + t15 * t16 / 2.0 + t20 * t22 / 4.0;
        let t28 = param_a[0];
        let t29 = f64::ln(t12);
        let t33 = param_c[0] * t1;
        let t34 = t33 * t3;
        let t35 = t10 * t29;
        let t39 = param_d[0] * t1;
        let t43 = piecewise3(t13, t14 / t25, t28 * t29 + param_b[0] + t34 * t35 / 4.0 + t39 * t22 / 4.0);
        let t44 = param_gamma[1];
        let t45 = param_beta1[1];
        let t49 = param_beta2[1] * t1;
        let t52 = 1.0 + t45 * t16 / 2.0 + t49 * t22 / 4.0;
        let t55 = param_a[1];
        let t59 = param_c[1] * t1;
        let t60 = t59 * t3;
        let t64 = param_d[1] * t1;
        let t68 = piecewise3(t13, t44 / t52, t55 * t29 + param_b[1] + t60 * t35 / 4.0 + t64 * t22 / 4.0);
        let t69 = t68 - t43;
        let t70 = rho[ip * 2] - rho[ip * 2 + 1];
        let t71 = 1.0 / t7;
        let t72 = t70 * t71;
        let t73 = 1.0 + t72;
        let t74 = t73 <= zeta_threshold;
        let t75 = pow_1_3(zeta_threshold);
        let t76 = t75 * zeta_threshold;
        let t77 = pow_1_3(t73);
        let t79 = piecewise3(t74, t76, t77 * t73);
        let t80 = 1.0 - t72;
        let t81 = t80 <= zeta_threshold;
        let t82 = pow_1_3(t80);
        let t84 = piecewise3(t81, t76, t82 * t80);
        let t85 = t79 + t84 - 2.0;
        let t87 = M_CBRT2;
        let t90 = 1.0 / (2.0 * t87 - 2.0);
        let t91 = t69 * t85 * t90;
        let tzk0 = t43 + t91;
        zk[ip] += tzk0;

        let t92 = t25 * t25;
        let t94 = t14 / t92;
        let t95 = 1.0 / t16;
        let t97 = t15 * t95 * t1;
        let t99 = 1.0 / t8 / t7;
        let t100 = t21 * t99;
        let t104 = -t20 * t100 / 12.0 - t97 * t100 / 12.0;
        let t109 = t6 * t99 * t29;
        let t117 = piecewise3(t13, -t94 * t104, -t28 * t71 / 3.0 - t34 * t109 / 12.0 - t33 * t100 / 12.0 - t39 * t100 / 12.0);
        let t118 = t52 * t52;
        let t120 = t44 / t118;
        let t122 = t45 * t95 * t1;
        let t126 = -t122 * t100 / 12.0 - t49 * t100 / 12.0;
        let t137 = piecewise3(t13, -t120 * t126, -t55 * t71 / 3.0 - t60 * t109 / 12.0 - t59 * t100 / 12.0 - t64 * t100 / 12.0);
        let t138 = t137 - t117;
        let t140 = t138 * t85 * t90;
        let t141 = t7 * t7;
        let t142 = 1.0 / t141;
        let t143 = t70 * t142;
        let t144 = t71 - t143;
        let t147 = piecewise3(t74, 0.0, 4.0 / 3.0 * t77 * t144);
        let t148 = -t144;
        let t151 = piecewise3(t81, 0.0, 4.0 / 3.0 * t82 * t148);
        let t152 = t147 + t151;
        let t154 = t69 * t152 * t90;
        let tvrho0 = t43 + t91 + t7 * (t117 + t140 + t154);
        vrho[ip * 2] += tvrho0;

        let t157 = -t71 - t143;
        let t160 = piecewise3(t74, 0.0, 4.0 / 3.0 * t77 * t157);
        let t161 = -t157;
        let t164 = piecewise3(t81, 0.0, 4.0 / 3.0 * t82 * t161);
        let t165 = t160 + t164;
        let t167 = t69 * t165 * t90;
        let tvrho1 = t43 + t91 + t7 * (t117 + t140 + t167);
        vrho[ip * 2 + 1] += tvrho1;

    }
}

/// LDA_C_PZ fxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_c_pz_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_c: f64,
    param_d: f64,
    param_gamma: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho[ip * 2] + rho[ip * 2 + 1];
        let t8 = pow_1_3(t7);
        let t9 = 1.0 / t8;
        let t10 = t6 * t9;
        let t11 = t1 * t3 * t10;
        let t12 = t11 / 4.0;
        let t13 = 1.0 <= t12;
        let t14 = param_gamma[0];
        let t15 = param_beta1[0];
        let t16 = f64::sqrt(t11);
        let t20 = param_beta2[0] * t1;
        let t21 = t3 * t6;
        let t22 = t21 * t9;
        let t25 = 1.0 + t15 * t16 / 2.0 + t20 * t22 / 4.0;
        let t28 = param_a[0];
        let t29 = f64::ln(t12);
        let t33 = param_c[0] * t1;
        let t34 = t33 * t3;
        let t35 = t10 * t29;
        let t39 = param_d[0] * t1;
        let t43 = piecewise3(t13, t14 / t25, t28 * t29 + param_b[0] + t34 * t35 / 4.0 + t39 * t22 / 4.0);
        let t44 = param_gamma[1];
        let t45 = param_beta1[1];
        let t49 = param_beta2[1] * t1;
        let t52 = 1.0 + t45 * t16 / 2.0 + t49 * t22 / 4.0;
        let t55 = param_a[1];
        let t59 = param_c[1] * t1;
        let t60 = t59 * t3;
        let t64 = param_d[1] * t1;
        let t68 = piecewise3(t13, t44 / t52, t55 * t29 + param_b[1] + t60 * t35 / 4.0 + t64 * t22 / 4.0);
        let t69 = t68 - t43;
        let t70 = rho[ip * 2] - rho[ip * 2 + 1];
        let t71 = 1.0 / t7;
        let t72 = t70 * t71;
        let t73 = 1.0 + t72;
        let t74 = t73 <= zeta_threshold;
        let t75 = pow_1_3(zeta_threshold);
        let t76 = t75 * zeta_threshold;
        let t77 = pow_1_3(t73);
        let t79 = piecewise3(t74, t76, t77 * t73);
        let t80 = 1.0 - t72;
        let t81 = t80 <= zeta_threshold;
        let t82 = pow_1_3(t80);
        let t84 = piecewise3(t81, t76, t82 * t80);
        let t85 = t79 + t84 - 2.0;
        let t87 = M_CBRT2;
        let t90 = 1.0 / (2.0 * t87 - 2.0);
        let t91 = t69 * t85 * t90;
        let tzk0 = t43 + t91;
        zk[ip] += tzk0;

        let t92 = t25 * t25;
        let t94 = t14 / t92;
        let t95 = 1.0 / t16;
        let t97 = t15 * t95 * t1;
        let t99 = 1.0 / t8 / t7;
        let t100 = t21 * t99;
        let t104 = -t20 * t100 / 12.0 - t97 * t100 / 12.0;
        let t109 = t6 * t99 * t29;
        let t117 = piecewise3(t13, -t94 * t104, -t28 * t71 / 3.0 - t34 * t109 / 12.0 - t33 * t100 / 12.0 - t39 * t100 / 12.0);
        let t118 = t52 * t52;
        let t120 = t44 / t118;
        let t122 = t45 * t95 * t1;
        let t126 = -t122 * t100 / 12.0 - t49 * t100 / 12.0;
        let t137 = piecewise3(t13, -t120 * t126, -t55 * t71 / 3.0 - t60 * t109 / 12.0 - t59 * t100 / 12.0 - t64 * t100 / 12.0);
        let t138 = t137 - t117;
        let t140 = t138 * t85 * t90;
        let t141 = t7 * t7;
        let t142 = 1.0 / t141;
        let t143 = t70 * t142;
        let t144 = t71 - t143;
        let t147 = piecewise3(t74, 0.0, 4.0 / 3.0 * t77 * t144);
        let t148 = -t144;
        let t151 = piecewise3(t81, 0.0, 4.0 / 3.0 * t82 * t148);
        let t152 = t147 + t151;
        let t154 = t69 * t152 * t90;
        let tvrho0 = t43 + t91 + t7 * (t117 + t140 + t154);
        vrho[ip * 2] += tvrho0;

        let t157 = -t71 - t143;
        let t160 = piecewise3(t74, 0.0, 4.0 / 3.0 * t77 * t157);
        let t161 = -t157;
        let t164 = piecewise3(t81, 0.0, 4.0 / 3.0 * t82 * t161);
        let t165 = t160 + t164;
        let t167 = t69 * t165 * t90;
        let tvrho1 = t43 + t91 + t7 * (t117 + t140 + t167);
        vrho[ip * 2 + 1] += tvrho1;

        let t170 = 2.0 * t117;
        let t171 = 2.0 * t140;
        let t175 = t14 / t92 / t25;
        let t176 = t104 * t104;
        let t180 = 1.0 / t16 / t11;
        let t182 = t1 * t1;
        let t183 = t15 * t180 * t182;
        let t184 = t3 * t3;
        let t185 = t184 * t5;
        let t186 = t8 * t8;
        let t189 = t185 / t186 / t141;
        let t193 = 1.0 / t8 / t141;
        let t194 = t21 * t193;
        let t199 = -t183 * t189 / 18.0 + t97 * t194 / 9.0 + t20 * t194 / 9.0;
        let t205 = t6 * t193 * t29;
        let t213 = piecewise3(t13, 2.0 * t175 * t176 - t94 * t199, t28 * t142 / 3.0 + t34 * t205 / 9.0 + 5.0 / 36.0 * t33 * t194 + t39 * t194 / 9.0);
        let t216 = t44 / t118 / t52;
        let t217 = t126 * t126;
        let t221 = t45 * t180 * t182;
        let t228 = -t221 * t189 / 18.0 + t122 * t194 / 9.0 + t49 * t194 / 9.0;
        let t240 = piecewise3(t13, -t120 * t228 + 2.0 * t216 * t217, t55 * t142 / 3.0 + t60 * t205 / 9.0 + 5.0 / 36.0 * t59 * t194 + t64 * t194 / 9.0);
        let t241 = t240 - t213;
        let t243 = t241 * t85 * t90;
        let t245 = t138 * t152 * t90;
        let t246 = 2.0 * t245;
        let t247 = t77 * t77;
        let t248 = 1.0 / t247;
        let t249 = t144 * t144;
        let t252 = t141 * t7;
        let t253 = 1.0 / t252;
        let t254 = t70 * t253;
        let t256 = -2.0 * t142 + 2.0 * t254;
        let t260 = piecewise3(t74, 0.0, 4.0 / 9.0 * t248 * t249 + 4.0 / 3.0 * t77 * t256);
        let t261 = t82 * t82;
        let t262 = 1.0 / t261;
        let t263 = t148 * t148;
        let t266 = -t256;
        let t270 = piecewise3(t81, 0.0, 4.0 / 9.0 * t262 * t263 + 4.0 / 3.0 * t82 * t266);
        let t271 = t260 + t270;
        let t273 = t69 * t271 * t90;
        let tv2rho20 = t170 + t171 + 2.0 * t154 + t7 * (t213 + t243 + t246 + t273);
        v2rho2[ip * 3] += tv2rho20;

        let t277 = t138 * t165 * t90;
        let t278 = t248 * t157;
        let t281 = t77 * t70;
        let t285 = piecewise3(t74, 0.0, 4.0 / 9.0 * t278 * t144 + 8.0 / 3.0 * t281 * t253);
        let t286 = t262 * t161;
        let t289 = t82 * t70;
        let t293 = piecewise3(t81, 0.0, 4.0 / 9.0 * t286 * t148 - 8.0 / 3.0 * t289 * t253);
        let t294 = t285 + t293;
        let t296 = t69 * t294 * t90;
        let tv2rho21 = t170 + t171 + t154 + t167 + t7 * (t213 + t243 + t245 + t277 + t296);
        v2rho2[ip * 3 + 1] += tv2rho21;

        let t300 = 2.0 * t277;
        let t301 = t157 * t157;
        let t305 = 2.0 * t142 + 2.0 * t254;
        let t309 = piecewise3(t74, 0.0, 4.0 / 9.0 * t248 * t301 + 4.0 / 3.0 * t77 * t305);
        let t310 = t161 * t161;
        let t313 = -t305;
        let t317 = piecewise3(t81, 0.0, 4.0 / 9.0 * t262 * t310 + 4.0 / 3.0 * t82 * t313);
        let t318 = t309 + t317;
        let t320 = t69 * t318 * t90;
        let tv2rho22 = t170 + t171 + 2.0 * t167 + t7 * (t213 + t243 + t300 + t320);
        v2rho2[ip * 3 + 2] += tv2rho22;

    }
}

/// LDA_C_PZ kxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_c_pz_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_c: f64,
    param_d: f64,
    param_gamma: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho[ip * 2] + rho[ip * 2 + 1];
        let t8 = pow_1_3(t7);
        let t9 = 1.0 / t8;
        let t10 = t6 * t9;
        let t11 = t1 * t3 * t10;
        let t12 = t11 / 4.0;
        let t13 = 1.0 <= t12;
        let t14 = param_gamma[0];
        let t15 = param_beta1[0];
        let t16 = f64::sqrt(t11);
        let t20 = param_beta2[0] * t1;
        let t21 = t3 * t6;
        let t22 = t21 * t9;
        let t25 = 1.0 + t15 * t16 / 2.0 + t20 * t22 / 4.0;
        let t28 = param_a[0];
        let t29 = f64::ln(t12);
        let t33 = param_c[0] * t1;
        let t34 = t33 * t3;
        let t35 = t10 * t29;
        let t39 = param_d[0] * t1;
        let t43 = piecewise3(t13, t14 / t25, t28 * t29 + param_b[0] + t34 * t35 / 4.0 + t39 * t22 / 4.0);
        let t44 = param_gamma[1];
        let t45 = param_beta1[1];
        let t49 = param_beta2[1] * t1;
        let t52 = 1.0 + t45 * t16 / 2.0 + t49 * t22 / 4.0;
        let t55 = param_a[1];
        let t59 = param_c[1] * t1;
        let t60 = t59 * t3;
        let t64 = param_d[1] * t1;
        let t68 = piecewise3(t13, t44 / t52, t55 * t29 + param_b[1] + t60 * t35 / 4.0 + t64 * t22 / 4.0);
        let t69 = t68 - t43;
        let t70 = rho[ip * 2] - rho[ip * 2 + 1];
        let t71 = 1.0 / t7;
        let t72 = t70 * t71;
        let t73 = 1.0 + t72;
        let t74 = t73 <= zeta_threshold;
        let t75 = pow_1_3(zeta_threshold);
        let t76 = t75 * zeta_threshold;
        let t77 = pow_1_3(t73);
        let t79 = piecewise3(t74, t76, t77 * t73);
        let t80 = 1.0 - t72;
        let t81 = t80 <= zeta_threshold;
        let t82 = pow_1_3(t80);
        let t84 = piecewise3(t81, t76, t82 * t80);
        let t85 = t79 + t84 - 2.0;
        let t87 = M_CBRT2;
        let t90 = 1.0 / (2.0 * t87 - 2.0);
        let t91 = t69 * t85 * t90;
        let tzk0 = t43 + t91;
        zk[ip] += tzk0;

        let t92 = t25 * t25;
        let t94 = t14 / t92;
        let t95 = 1.0 / t16;
        let t97 = t15 * t95 * t1;
        let t99 = 1.0 / t8 / t7;
        let t100 = t21 * t99;
        let t104 = -t20 * t100 / 12.0 - t97 * t100 / 12.0;
        let t109 = t6 * t99 * t29;
        let t117 = piecewise3(t13, -t94 * t104, -t28 * t71 / 3.0 - t34 * t109 / 12.0 - t33 * t100 / 12.0 - t39 * t100 / 12.0);
        let t118 = t52 * t52;
        let t120 = t44 / t118;
        let t122 = t45 * t95 * t1;
        let t126 = -t122 * t100 / 12.0 - t49 * t100 / 12.0;
        let t137 = piecewise3(t13, -t120 * t126, -t55 * t71 / 3.0 - t60 * t109 / 12.0 - t59 * t100 / 12.0 - t64 * t100 / 12.0);
        let t138 = t137 - t117;
        let t140 = t138 * t85 * t90;
        let t141 = t7 * t7;
        let t142 = 1.0 / t141;
        let t143 = t70 * t142;
        let t144 = t71 - t143;
        let t147 = piecewise3(t74, 0.0, 4.0 / 3.0 * t77 * t144);
        let t148 = -t144;
        let t151 = piecewise3(t81, 0.0, 4.0 / 3.0 * t82 * t148);
        let t152 = t147 + t151;
        let t154 = t69 * t152 * t90;
        let tvrho0 = t43 + t91 + t7 * (t117 + t140 + t154);
        vrho[ip * 2] += tvrho0;

        let t157 = -t71 - t143;
        let t160 = piecewise3(t74, 0.0, 4.0 / 3.0 * t77 * t157);
        let t161 = -t157;
        let t164 = piecewise3(t81, 0.0, 4.0 / 3.0 * t82 * t161);
        let t165 = t160 + t164;
        let t167 = t69 * t165 * t90;
        let tvrho1 = t43 + t91 + t7 * (t117 + t140 + t167);
        vrho[ip * 2 + 1] += tvrho1;

        let t170 = 2.0 * t117;
        let t171 = 2.0 * t140;
        let t175 = t14 / t92 / t25;
        let t176 = t104 * t104;
        let t180 = 1.0 / t16 / t11;
        let t182 = t1 * t1;
        let t183 = t15 * t180 * t182;
        let t184 = t3 * t3;
        let t185 = t184 * t5;
        let t186 = t8 * t8;
        let t189 = t185 / t186 / t141;
        let t193 = 1.0 / t8 / t141;
        let t194 = t21 * t193;
        let t199 = -t183 * t189 / 18.0 + t97 * t194 / 9.0 + t20 * t194 / 9.0;
        let t205 = t6 * t193 * t29;
        let t213 = piecewise3(t13, 2.0 * t175 * t176 - t94 * t199, t28 * t142 / 3.0 + t34 * t205 / 9.0 + 5.0 / 36.0 * t33 * t194 + t39 * t194 / 9.0);
        let t216 = t44 / t118 / t52;
        let t217 = t126 * t126;
        let t221 = t45 * t180 * t182;
        let t228 = -t221 * t189 / 18.0 + t122 * t194 / 9.0 + t49 * t194 / 9.0;
        let t240 = piecewise3(t13, -t120 * t228 + 2.0 * t216 * t217, t55 * t142 / 3.0 + t60 * t205 / 9.0 + 5.0 / 36.0 * t59 * t194 + t64 * t194 / 9.0);
        let t241 = t240 - t213;
        let t243 = t241 * t85 * t90;
        let t245 = t138 * t152 * t90;
        let t246 = 2.0 * t245;
        let t247 = t77 * t77;
        let t248 = 1.0 / t247;
        let t249 = t144 * t144;
        let t252 = t141 * t7;
        let t253 = 1.0 / t252;
        let t254 = t70 * t253;
        let t256 = -2.0 * t142 + 2.0 * t254;
        let t260 = piecewise3(t74, 0.0, 4.0 / 9.0 * t248 * t249 + 4.0 / 3.0 * t77 * t256);
        let t261 = t82 * t82;
        let t262 = 1.0 / t261;
        let t263 = t148 * t148;
        let t266 = -t256;
        let t270 = piecewise3(t81, 0.0, 4.0 / 9.0 * t262 * t263 + 4.0 / 3.0 * t82 * t266);
        let t271 = t260 + t270;
        let t273 = t69 * t271 * t90;
        let tv2rho20 = t170 + t171 + 2.0 * t154 + t7 * (t213 + t243 + t246 + t273);
        v2rho2[ip * 3] += tv2rho20;

        let t277 = t138 * t165 * t90;
        let t278 = t248 * t157;
        let t281 = t77 * t70;
        let t285 = piecewise3(t74, 0.0, 4.0 / 9.0 * t278 * t144 + 8.0 / 3.0 * t281 * t253);
        let t286 = t262 * t161;
        let t289 = t82 * t70;
        let t293 = piecewise3(t81, 0.0, 4.0 / 9.0 * t286 * t148 - 8.0 / 3.0 * t289 * t253);
        let t294 = t285 + t293;
        let t296 = t69 * t294 * t90;
        let tv2rho21 = t170 + t171 + t154 + t167 + t7 * (t213 + t243 + t245 + t277 + t296);
        v2rho2[ip * 3 + 1] += tv2rho21;

        let t300 = 2.0 * t277;
        let t301 = t157 * t157;
        let t305 = 2.0 * t142 + 2.0 * t254;
        let t309 = piecewise3(t74, 0.0, 4.0 / 9.0 * t248 * t301 + 4.0 / 3.0 * t77 * t305);
        let t310 = t161 * t161;
        let t313 = -t305;
        let t317 = piecewise3(t81, 0.0, 4.0 / 9.0 * t262 * t310 + 4.0 / 3.0 * t82 * t313);
        let t318 = t309 + t317;
        let t320 = t69 * t318 * t90;
        let tv2rho22 = t170 + t171 + 2.0 * t167 + t7 * (t213 + t243 + t300 + t320);
        v2rho2[ip * 3 + 2] += tv2rho22;

        let t323 = 3.0 * t213;
        let t324 = 3.0 * t243;
        let t327 = t92 * t92;
        let t329 = t14 / t327;
        let t342 = 1.0 / t16 / t182 / t184 / t5 * t186 / 4.0;
        let t343 = t15 * t342;
        let t344 = t141 * t141;
        let t345 = 1.0 / t344;
        let t346 = t2 * t345;
        let t351 = t185 / t186 / t252;
        let t355 = 1.0 / t8 / t252;
        let t356 = t21 * t355;
        let t361 = -t343 * t346 / 3.0 + 2.0 / 9.0 * t183 * t351 - 7.0 / 27.0 * t97 * t356 - 7.0 / 27.0 * t20 * t356;
        let t367 = t6 * t355 * t29;
        let t375 = piecewise3(t13, 6.0 * t175 * t104 * t199 - 6.0 * t329 * t176 * t104 - t94 * t361, -2.0 / 3.0 * t28 * t253 - 7.0 / 27.0 * t34 * t367 - 13.0 / 36.0 * t33 * t356 - 7.0 / 27.0 * t39 * t356);
        let t376 = t118 * t118;
        let t378 = t44 / t376;
        let t385 = t45 * t342;
        let t394 = -t385 * t346 / 3.0 + 2.0 / 9.0 * t221 * t351 - 7.0 / 27.0 * t122 * t356 - 7.0 / 27.0 * t49 * t356;
        let t406 = piecewise3(t13, 6.0 * t216 * t126 * t228 - 6.0 * t378 * t217 * t126 - t120 * t394, -2.0 / 3.0 * t55 * t253 - 7.0 / 27.0 * t60 * t367 - 13.0 / 36.0 * t59 * t356 - 7.0 / 27.0 * t64 * t356);
        let t407 = t406 - t375;
        let t409 = t407 * t85 * t90;
        let t411 = t241 * t152 * t90;
        let t412 = 3.0 * t411;
        let t414 = t138 * t271 * t90;
        let t417 = 1.0 / t247 / t73;
        let t418 = t249 * t144;
        let t421 = t248 * t144;
        let t424 = t70 * t345;
        let t426 = 6.0 * t253 - 6.0 * t424;
        let t430 = piecewise3(t74, 0.0, -8.0 / 27.0 * t417 * t418 + 4.0 / 3.0 * t421 * t256 + 4.0 / 3.0 * t77 * t426);
        let t432 = 1.0 / t261 / t80;
        let t433 = t263 * t148;
        let t436 = t262 * t148;
        let t439 = -t426;
        let t443 = piecewise3(t81, 0.0, -8.0 / 27.0 * t432 * t433 + 4.0 / 3.0 * t436 * t266 + 4.0 / 3.0 * t82 * t439);
        let t444 = t430 + t443;
        let t446 = t69 * t444 * t90;
        let tv3rho30 = t323 + t324 + 6.0 * t245 + 3.0 * t273 + t7 * (t375 + t409 + t412 + 3.0 * t414 + t446);
        v3rho3[ip * 4] += tv3rho30;

        let t450 = 2.0 * t296;
        let t453 = t241 * t165 * t90;
        let t455 = t138 * t294 * t90;
        let t456 = 2.0 * t455;
        let t457 = t417 * t157;
        let t460 = t248 * t70;
        let t471 = piecewise3(t74, 0.0, -8.0 / 27.0 * t457 * t249 + 16.0 / 9.0 * t460 * t253 * t144 + 4.0 / 9.0 * t278 * t256 + 8.0 / 3.0 * t77 * t253 - 8.0 * t281 * t345);
        let t472 = t432 * t161;
        let t475 = t262 * t70;
        let t486 = piecewise3(t81, 0.0, -8.0 / 27.0 * t472 * t263 - 16.0 / 9.0 * t475 * t253 * t148 + 4.0 / 9.0 * t286 * t266 - 8.0 / 3.0 * t82 * t253 + 8.0 * t289 * t345);
        let t487 = t471 + t486;
        let t489 = t69 * t487 * t90;
        let tv3rho31 = t323 + t324 + 4.0 * t245 + t273 + t300 + t450 + t7 * (t375 + t409 + 2.0 * t411 + t414 + t453 + t456 + t489);
        v3rho3[ip * 4 + 1] += tv3rho31;

        let t495 = t138 * t318 * t90;
        let t496 = t417 * t301;
        let t501 = t248 * t305;
        let t506 = -2.0 * t253 - 6.0 * t424;
        let t510 = piecewise3(t74, 0.0, -8.0 / 27.0 * t496 * t144 + 16.0 / 9.0 * t278 * t254 + 4.0 / 9.0 * t501 * t144 + 4.0 / 3.0 * t77 * t506);
        let t511 = t432 * t310;
        let t516 = t262 * t313;
        let t519 = -t506;
        let t523 = piecewise3(t81, 0.0, -8.0 / 27.0 * t511 * t148 - 16.0 / 9.0 * t286 * t254 + 4.0 / 9.0 * t516 * t148 + 4.0 / 3.0 * t82 * t519);
        let t524 = t510 + t523;
        let t526 = t69 * t524 * t90;
        let tv3rho32 = t323 + t324 + t246 + 4.0 * t277 + t450 + t320 + t7 * (t375 + t409 + t411 + 2.0 * t453 + t456 + t495 + t526);
        v3rho3[ip * 4 + 2] += tv3rho32;

        let t531 = 3.0 * t453;
        let t533 = t301 * t157;
        let t539 = -6.0 * t253 - 6.0 * t424;
        let t543 = piecewise3(t74, 0.0, -8.0 / 27.0 * t417 * t533 + 4.0 / 3.0 * t278 * t305 + 4.0 / 3.0 * t77 * t539);
        let t544 = t310 * t161;
        let t549 = -t539;
        let t553 = piecewise3(t81, 0.0, -8.0 / 27.0 * t432 * t544 + 4.0 / 3.0 * t286 * t313 + 4.0 / 3.0 * t82 * t549);
        let t554 = t543 + t553;
        let t556 = t69 * t554 * t90;
        let tv3rho33 = t323 + t324 + 6.0 * t277 + 3.0 * t320 + t7 * (t375 + t409 + t531 + 3.0 * t495 + t556);
        v3rho3[ip * 4 + 3] += tv3rho33;

    }
}

/// LDA_C_PZ lxc -- polarized.
#[cube(launch_unchecked)]
pub fn lda_c_pz_lxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_c: f64,
    param_d: f64,
    param_gamma: f64,
    dens_threshold: f64,
    #[allow(unused_variables)] zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];

        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho[ip * 2] + rho[ip * 2 + 1];
        let t8 = pow_1_3(t7);
        let t9 = 1.0 / t8;
        let t10 = t6 * t9;
        let t11 = t1 * t3 * t10;
        let t12 = t11 / 4.0;
        let t13 = 1.0 <= t12;
        let t14 = param_gamma[0];
        let t15 = param_beta1[0];
        let t16 = f64::sqrt(t11);
        let t20 = param_beta2[0] * t1;
        let t21 = t3 * t6;
        let t22 = t21 * t9;
        let t25 = 1.0 + t15 * t16 / 2.0 + t20 * t22 / 4.0;
        let t28 = param_a[0];
        let t29 = f64::ln(t12);
        let t33 = param_c[0] * t1;
        let t34 = t33 * t3;
        let t35 = t10 * t29;
        let t39 = param_d[0] * t1;
        let t43 = piecewise3(t13, t14 / t25, t28 * t29 + param_b[0] + t34 * t35 / 4.0 + t39 * t22 / 4.0);
        let t44 = param_gamma[1];
        let t45 = param_beta1[1];
        let t49 = param_beta2[1] * t1;
        let t52 = 1.0 + t45 * t16 / 2.0 + t49 * t22 / 4.0;
        let t55 = param_a[1];
        let t59 = param_c[1] * t1;
        let t60 = t59 * t3;
        let t64 = param_d[1] * t1;
        let t68 = piecewise3(t13, t44 / t52, t55 * t29 + param_b[1] + t60 * t35 / 4.0 + t64 * t22 / 4.0);
        let t69 = t68 - t43;
        let t70 = rho[ip * 2] - rho[ip * 2 + 1];
        let t71 = 1.0 / t7;
        let t72 = t70 * t71;
        let t73 = 1.0 + t72;
        let t74 = t73 <= zeta_threshold;
        let t75 = pow_1_3(zeta_threshold);
        let t76 = t75 * zeta_threshold;
        let t77 = pow_1_3(t73);
        let t79 = piecewise3(t74, t76, t77 * t73);
        let t80 = 1.0 - t72;
        let t81 = t80 <= zeta_threshold;
        let t82 = pow_1_3(t80);
        let t84 = piecewise3(t81, t76, t82 * t80);
        let t85 = t79 + t84 - 2.0;
        let t87 = M_CBRT2;
        let t90 = 1.0 / (2.0 * t87 - 2.0);
        let t91 = t69 * t85 * t90;
        let tzk0 = t43 + t91;
        zk[ip] += tzk0;

        let t92 = t25 * t25;
        let t94 = t14 / t92;
        let t95 = 1.0 / t16;
        let t97 = t15 * t95 * t1;
        let t99 = 1.0 / t8 / t7;
        let t100 = t21 * t99;
        let t104 = -t20 * t100 / 12.0 - t97 * t100 / 12.0;
        let t109 = t6 * t99 * t29;
        let t117 = piecewise3(t13, -t94 * t104, -t28 * t71 / 3.0 - t34 * t109 / 12.0 - t33 * t100 / 12.0 - t39 * t100 / 12.0);
        let t118 = t52 * t52;
        let t120 = t44 / t118;
        let t122 = t45 * t95 * t1;
        let t126 = -t122 * t100 / 12.0 - t49 * t100 / 12.0;
        let t137 = piecewise3(t13, -t120 * t126, -t55 * t71 / 3.0 - t60 * t109 / 12.0 - t59 * t100 / 12.0 - t64 * t100 / 12.0);
        let t138 = t137 - t117;
        let t140 = t138 * t85 * t90;
        let t141 = t7 * t7;
        let t142 = 1.0 / t141;
        let t143 = t70 * t142;
        let t144 = t71 - t143;
        let t147 = piecewise3(t74, 0.0, 4.0 / 3.0 * t77 * t144);
        let t148 = -t144;
        let t151 = piecewise3(t81, 0.0, 4.0 / 3.0 * t82 * t148);
        let t152 = t147 + t151;
        let t154 = t69 * t152 * t90;
        let tvrho0 = t43 + t91 + t7 * (t117 + t140 + t154);
        vrho[ip * 2] += tvrho0;

        let t157 = -t71 - t143;
        let t160 = piecewise3(t74, 0.0, 4.0 / 3.0 * t77 * t157);
        let t161 = -t157;
        let t164 = piecewise3(t81, 0.0, 4.0 / 3.0 * t82 * t161);
        let t165 = t160 + t164;
        let t167 = t69 * t165 * t90;
        let tvrho1 = t43 + t91 + t7 * (t117 + t140 + t167);
        vrho[ip * 2 + 1] += tvrho1;

        let t170 = 2.0 * t117;
        let t171 = 2.0 * t140;
        let t175 = t14 / t92 / t25;
        let t176 = t104 * t104;
        let t180 = 1.0 / t16 / t11;
        let t182 = t1 * t1;
        let t183 = t15 * t180 * t182;
        let t184 = t3 * t3;
        let t185 = t184 * t5;
        let t186 = t8 * t8;
        let t189 = t185 / t186 / t141;
        let t193 = 1.0 / t8 / t141;
        let t194 = t21 * t193;
        let t199 = -t183 * t189 / 18.0 + t97 * t194 / 9.0 + t20 * t194 / 9.0;
        let t205 = t6 * t193 * t29;
        let t213 = piecewise3(t13, 2.0 * t175 * t176 - t94 * t199, t28 * t142 / 3.0 + t34 * t205 / 9.0 + 5.0 / 36.0 * t33 * t194 + t39 * t194 / 9.0);
        let t216 = t44 / t118 / t52;
        let t217 = t126 * t126;
        let t221 = t45 * t180 * t182;
        let t228 = -t221 * t189 / 18.0 + t122 * t194 / 9.0 + t49 * t194 / 9.0;
        let t240 = piecewise3(t13, -t120 * t228 + 2.0 * t216 * t217, t55 * t142 / 3.0 + t60 * t205 / 9.0 + 5.0 / 36.0 * t59 * t194 + t64 * t194 / 9.0);
        let t241 = t240 - t213;
        let t243 = t241 * t85 * t90;
        let t245 = t138 * t152 * t90;
        let t246 = 2.0 * t245;
        let t247 = t77 * t77;
        let t248 = 1.0 / t247;
        let t249 = t144 * t144;
        let t252 = t141 * t7;
        let t253 = 1.0 / t252;
        let t254 = t70 * t253;
        let t256 = -2.0 * t142 + 2.0 * t254;
        let t260 = piecewise3(t74, 0.0, 4.0 / 9.0 * t248 * t249 + 4.0 / 3.0 * t77 * t256);
        let t261 = t82 * t82;
        let t262 = 1.0 / t261;
        let t263 = t148 * t148;
        let t266 = -t256;
        let t270 = piecewise3(t81, 0.0, 4.0 / 9.0 * t262 * t263 + 4.0 / 3.0 * t82 * t266);
        let t271 = t260 + t270;
        let t273 = t69 * t271 * t90;
        let tv2rho20 = t170 + t171 + 2.0 * t154 + t7 * (t213 + t243 + t246 + t273);
        v2rho2[ip * 3] += tv2rho20;

        let t277 = t138 * t165 * t90;
        let t278 = t248 * t157;
        let t281 = t77 * t70;
        let t285 = piecewise3(t74, 0.0, 4.0 / 9.0 * t278 * t144 + 8.0 / 3.0 * t281 * t253);
        let t286 = t262 * t161;
        let t289 = t82 * t70;
        let t293 = piecewise3(t81, 0.0, 4.0 / 9.0 * t286 * t148 - 8.0 / 3.0 * t289 * t253);
        let t294 = t285 + t293;
        let t296 = t69 * t294 * t90;
        let tv2rho21 = t170 + t171 + t154 + t167 + t7 * (t213 + t243 + t245 + t277 + t296);
        v2rho2[ip * 3 + 1] += tv2rho21;

        let t300 = 2.0 * t277;
        let t301 = t157 * t157;
        let t305 = 2.0 * t142 + 2.0 * t254;
        let t309 = piecewise3(t74, 0.0, 4.0 / 9.0 * t248 * t301 + 4.0 / 3.0 * t77 * t305);
        let t310 = t161 * t161;
        let t313 = -t305;
        let t317 = piecewise3(t81, 0.0, 4.0 / 9.0 * t262 * t310 + 4.0 / 3.0 * t82 * t313);
        let t318 = t309 + t317;
        let t320 = t69 * t318 * t90;
        let tv2rho22 = t170 + t171 + 2.0 * t167 + t7 * (t213 + t243 + t300 + t320);
        v2rho2[ip * 3 + 2] += tv2rho22;

        let t323 = 3.0 * t213;
        let t324 = 3.0 * t243;
        let t327 = t92 * t92;
        let t329 = t14 / t327;
        let t342 = 1.0 / t16 / t182 / t184 / t5 * t186 / 4.0;
        let t343 = t15 * t342;
        let t344 = t141 * t141;
        let t345 = 1.0 / t344;
        let t346 = t2 * t345;
        let t351 = t185 / t186 / t252;
        let t355 = 1.0 / t8 / t252;
        let t356 = t21 * t355;
        let t361 = -t343 * t346 / 3.0 + 2.0 / 9.0 * t183 * t351 - 7.0 / 27.0 * t97 * t356 - 7.0 / 27.0 * t20 * t356;
        let t367 = t6 * t355 * t29;
        let t375 = piecewise3(t13, 6.0 * t175 * t104 * t199 - 6.0 * t329 * t176 * t104 - t94 * t361, -2.0 / 3.0 * t28 * t253 - 7.0 / 27.0 * t34 * t367 - 13.0 / 36.0 * t33 * t356 - 7.0 / 27.0 * t39 * t356);
        let t376 = t118 * t118;
        let t378 = t44 / t376;
        let t385 = t45 * t342;
        let t394 = -t385 * t346 / 3.0 + 2.0 / 9.0 * t221 * t351 - 7.0 / 27.0 * t122 * t356 - 7.0 / 27.0 * t49 * t356;
        let t406 = piecewise3(t13, 6.0 * t216 * t126 * t228 - 6.0 * t378 * t217 * t126 - t120 * t394, -2.0 / 3.0 * t55 * t253 - 7.0 / 27.0 * t60 * t367 - 13.0 / 36.0 * t59 * t356 - 7.0 / 27.0 * t64 * t356);
        let t407 = t406 - t375;
        let t409 = t407 * t85 * t90;
        let t411 = t241 * t152 * t90;
        let t412 = 3.0 * t411;
        let t414 = t138 * t271 * t90;
        let t417 = 1.0 / t247 / t73;
        let t418 = t249 * t144;
        let t421 = t248 * t144;
        let t424 = t70 * t345;
        let t426 = 6.0 * t253 - 6.0 * t424;
        let t430 = piecewise3(t74, 0.0, -8.0 / 27.0 * t417 * t418 + 4.0 / 3.0 * t421 * t256 + 4.0 / 3.0 * t77 * t426);
        let t432 = 1.0 / t261 / t80;
        let t433 = t263 * t148;
        let t436 = t262 * t148;
        let t439 = -t426;
        let t443 = piecewise3(t81, 0.0, -8.0 / 27.0 * t432 * t433 + 4.0 / 3.0 * t436 * t266 + 4.0 / 3.0 * t82 * t439);
        let t444 = t430 + t443;
        let t446 = t69 * t444 * t90;
        let tv3rho30 = t323 + t324 + 6.0 * t245 + 3.0 * t273 + t7 * (t375 + t409 + t412 + 3.0 * t414 + t446);
        v3rho3[ip * 4] += tv3rho30;

        let t450 = 2.0 * t296;
        let t453 = t241 * t165 * t90;
        let t455 = t138 * t294 * t90;
        let t456 = 2.0 * t455;
        let t457 = t417 * t157;
        let t460 = t248 * t70;
        let t471 = piecewise3(t74, 0.0, -8.0 / 27.0 * t457 * t249 + 16.0 / 9.0 * t460 * t253 * t144 + 4.0 / 9.0 * t278 * t256 + 8.0 / 3.0 * t77 * t253 - 8.0 * t281 * t345);
        let t472 = t432 * t161;
        let t475 = t262 * t70;
        let t486 = piecewise3(t81, 0.0, -8.0 / 27.0 * t472 * t263 - 16.0 / 9.0 * t475 * t253 * t148 + 4.0 / 9.0 * t286 * t266 - 8.0 / 3.0 * t82 * t253 + 8.0 * t289 * t345);
        let t487 = t471 + t486;
        let t489 = t69 * t487 * t90;
        let tv3rho31 = t323 + t324 + 4.0 * t245 + t273 + t300 + t450 + t7 * (t375 + t409 + 2.0 * t411 + t414 + t453 + t456 + t489);
        v3rho3[ip * 4 + 1] += tv3rho31;

        let t495 = t138 * t318 * t90;
        let t496 = t417 * t301;
        let t501 = t248 * t305;
        let t506 = -2.0 * t253 - 6.0 * t424;
        let t510 = piecewise3(t74, 0.0, -8.0 / 27.0 * t496 * t144 + 16.0 / 9.0 * t278 * t254 + 4.0 / 9.0 * t501 * t144 + 4.0 / 3.0 * t77 * t506);
        let t511 = t432 * t310;
        let t516 = t262 * t313;
        let t519 = -t506;
        let t523 = piecewise3(t81, 0.0, -8.0 / 27.0 * t511 * t148 - 16.0 / 9.0 * t286 * t254 + 4.0 / 9.0 * t516 * t148 + 4.0 / 3.0 * t82 * t519);
        let t524 = t510 + t523;
        let t526 = t69 * t524 * t90;
        let tv3rho32 = t323 + t324 + t246 + 4.0 * t277 + t450 + t320 + t7 * (t375 + t409 + t411 + 2.0 * t453 + t456 + t495 + t526);
        v3rho3[ip * 4 + 2] += tv3rho32;

        let t531 = 3.0 * t453;
        let t533 = t301 * t157;
        let t539 = -6.0 * t253 - 6.0 * t424;
        let t543 = piecewise3(t74, 0.0, -8.0 / 27.0 * t417 * t533 + 4.0 / 3.0 * t278 * t305 + 4.0 / 3.0 * t77 * t539);
        let t544 = t310 * t161;
        let t549 = -t539;
        let t553 = piecewise3(t81, 0.0, -8.0 / 27.0 * t432 * t544 + 4.0 / 3.0 * t286 * t313 + 4.0 / 3.0 * t82 * t549);
        let t554 = t543 + t553;
        let t556 = t69 * t554 * t90;
        let tv3rho33 = t323 + t324 + 6.0 * t277 + 3.0 * t320 + t7 * (t375 + t409 + t531 + 3.0 * t495 + t556);
        v3rho3[ip * 4 + 3] += tv3rho33;

        let t559 = 4.0 * t375;
        let t560 = 4.0 * t409;
        let t567 = t176 * t176;
        let t573 = t199 * t199;
        let t582 = 1.0 / t16 / t2 / t71 / 48.0;
        let t585 = t344 * t7;
        let t589 = 1.0 / t8 / t585 * t1 * t21;
        let t592 = 1.0 / t585;
        let t593 = t2 * t592;
        let t598 = t185 / t186 / t344;
        let t602 = 1.0 / t8 / t344;
        let t603 = t21 * t602;
        let t614 = t6 * t602 * t29;
        let t622 = piecewise3(t13, 24.0 * t14 / t327 / t25 * t567 - 36.0 * t329 * t176 * t199 + 6.0 * t175 * t573 + 8.0 * t175 * t104 * t361 - t94 * (-5.0 / 18.0 * t15 * t582 * t2 * t589 + 8.0 / 3.0 * t343 * t593 - 80.0 / 81.0 * t183 * t598 + 70.0 / 81.0 * t97 * t603 + 70.0 / 81.0 * t20 * t603), 2.0 * t28 * t345 + 70.0 / 81.0 * t34 * t614 + 209.0 / 162.0 * t33 * t603 + 70.0 / 81.0 * t39 * t603);
        let t626 = t217 * t217;
        let t632 = t228 * t228;
        let t662 = piecewise3(t13, 24.0 * t44 / t376 / t52 * t626 - 36.0 * t378 * t217 * t228 + 6.0 * t216 * t632 + 8.0 * t216 * t126 * t394 - t120 * (-5.0 / 18.0 * t45 * t582 * t2 * t589 + 8.0 / 3.0 * t385 * t593 - 80.0 / 81.0 * t221 * t598 + 70.0 / 81.0 * t122 * t603 + 70.0 / 81.0 * t49 * t603), 2.0 * t55 * t345 + 70.0 / 81.0 * t60 * t614 + 209.0 / 162.0 * t59 * t603 + 70.0 / 81.0 * t64 * t603);
        let t665 = (t662 - t622) * t85 * t90;
        let t667 = t407 * t152 * t90;
        let t670 = t241 * t271 * t90;
        let t673 = t138 * t444 * t90;
        let t675 = t73 * t73;
        let t677 = 1.0 / t247 / t675;
        let t678 = t249 * t249;
        let t684 = t256 * t256;
        let t689 = t70 * t592;
        let t691 = -24.0 * t345 + 24.0 * t689;
        let t695 = piecewise3(t74, 0.0, 40.0 / 81.0 * t677 * t678 - 16.0 / 9.0 * t417 * t249 * t256 + 4.0 / 3.0 * t248 * t684 + 16.0 / 9.0 * t421 * t426 + 4.0 / 3.0 * t77 * t691);
        let t696 = t80 * t80;
        let t698 = 1.0 / t261 / t696;
        let t699 = t263 * t263;
        let t705 = t266 * t266;
        let t714 = piecewise3(t81, 0.0, 40.0 / 81.0 * t698 * t699 - 16.0 / 9.0 * t432 * t263 * t266 + 4.0 / 3.0 * t262 * t705 + 16.0 / 9.0 * t436 * t439 - 4.0 / 3.0 * t82 * t691);
        let tv4rho40 = t559 + t560 + 12.0 * t411 + 12.0 * t414 + 4.0 * t446 + t7 * (t622 + t665 + 4.0 * t667 + 6.0 * t670 + 4.0 * t673 + t69 * (t695 + t714) * t90);
        v4rho4[ip * 5] += tv4rho40;

        let t722 = 6.0 * t455;
        let t727 = t407 * t165 * t90;
        let t729 = t241 * t294 * t90;
        let t730 = 3.0 * t729;
        let t732 = t138 * t487 * t90;
        let t758 = 32.0 * t281 * t592;
        let t760 = piecewise3(t74, 0.0, 40.0 / 81.0 * t677 * t157 * t418 - 16.0 / 9.0 * t417 * t70 * t253 * t249 - 8.0 / 9.0 * t457 * t144 * t256 + 8.0 / 3.0 * t248 * t253 * t144 - 8.0 * t460 * t345 * t144 + 8.0 / 3.0 * t460 * t253 * t256 + 4.0 / 9.0 * t278 * t426 - 16.0 * t77 * t345 + t758);
        let t785 = 32.0 * t289 * t592;
        let t787 = piecewise3(t81, 0.0, 40.0 / 81.0 * t698 * t161 * t433 + 16.0 / 9.0 * t432 * t70 * t253 * t263 - 8.0 / 9.0 * t472 * t148 * t266 - 8.0 / 3.0 * t262 * t253 * t148 + 8.0 * t475 * t345 * t148 - 8.0 / 3.0 * t475 * t253 * t266 + 4.0 / 9.0 * t286 * t439 + 16.0 * t82 * t345 - t785);
        let tv4rho41 = t559 + t560 + 9.0 * t411 + 6.0 * t414 + t446 + t531 + t722 + 3.0 * t489 + t7 * (t622 + t665 + 3.0 * t667 + 3.0 * t670 + t673 + t727 + t730 + 3.0 * t732 + t69 * (t760 + t787) * t90);
        v4rho4[ip * 5 + 1] += tv4rho41;

        let t805 = t241 * t318 * t90;
        let t807 = t138 * t524 * t90;
        let t818 = t70 * t70;
        let t821 = 1.0 / t344 / t141;
        let t837 = piecewise3(t74, 0.0, 40.0 / 81.0 * t677 * t301 * t249 - 64.0 / 27.0 * t457 * t144 * t70 * t253 - 8.0 / 27.0 * t496 * t256 + 32.0 / 9.0 * t248 * t818 * t821 + 16.0 / 9.0 * t278 * t253 - 16.0 / 3.0 * t278 * t424 - 8.0 / 27.0 * t417 * t305 * t249 + 8.0 / 9.0 * t248 * t506 * t144 + 4.0 / 9.0 * t501 * t256 + t758);
        let t863 = piecewise3(t81, 0.0, 40.0 / 81.0 * t698 * t310 * t263 + 64.0 / 27.0 * t472 * t148 * t70 * t253 - 8.0 / 27.0 * t511 * t266 + 32.0 / 9.0 * t262 * t818 * t821 - 16.0 / 9.0 * t286 * t253 + 16.0 / 3.0 * t286 * t424 - 8.0 / 27.0 * t432 * t313 * t263 + 8.0 / 9.0 * t262 * t519 * t148 + 4.0 / 9.0 * t516 * t266 - t785);
        let tv4rho42 = t559 + t560 + 6.0 * t411 + 2.0 * t414 + 6.0 * t453 + 8.0 * t455 + 2.0 * t489 + 2.0 * t495 + 2.0 * t526 + t7 * (t622 + t665 + 2.0 * t667 + t670 + 2.0 * t727 + 4.0 * t729 + 2.0 * t732 + t805 + 2.0 * t807 + t69 * (t837 + t863) * t90);
        v4rho4[ip * 5 + 2] += tv4rho42;

        let t876 = t138 * t554 * t90;
        let t895 = 12.0 * t345 + 24.0 * t689;
        let t899 = piecewise3(t74, 0.0, 40.0 / 81.0 * t677 * t533 * t144 - 16.0 / 9.0 * t496 * t254 - 8.0 / 9.0 * t457 * t305 * t144 + 8.0 / 3.0 * t460 * t253 * t305 + 4.0 / 3.0 * t278 * t506 + 4.0 / 9.0 * t248 * t539 * t144 + 4.0 / 3.0 * t77 * t895);
        let t920 = piecewise3(t81, 0.0, 40.0 / 81.0 * t698 * t544 * t148 + 16.0 / 9.0 * t511 * t254 - 8.0 / 9.0 * t472 * t313 * t148 - 8.0 / 3.0 * t475 * t253 * t313 + 4.0 / 3.0 * t286 * t519 + 4.0 / 9.0 * t262 * t549 * t148 - 4.0 / 3.0 * t82 * t895);
        let tv4rho43 = t559 + t560 + t412 + 9.0 * t453 + t722 + 6.0 * t495 + 3.0 * t526 + t556 + t7 * (t622 + t665 + t667 + 3.0 * t727 + t730 + 3.0 * t805 + 3.0 * t807 + t876 + t69 * (t899 + t920) * t90);
        v4rho4[ip * 5 + 3] += tv4rho43;

        let t932 = t301 * t301;
        let t937 = t305 * t305;
        let t943 = 24.0 * t345 + 24.0 * t689;
        let t947 = piecewise3(t74, 0.0, 40.0 / 81.0 * t677 * t932 - 16.0 / 9.0 * t496 * t305 + 4.0 / 3.0 * t248 * t937 + 16.0 / 9.0 * t278 * t539 + 4.0 / 3.0 * t77 * t943);
        let t948 = t310 * t310;
        let t953 = t313 * t313;
        let t962 = piecewise3(t81, 0.0, 40.0 / 81.0 * t698 * t948 - 16.0 / 9.0 * t511 * t313 + 4.0 / 3.0 * t262 * t953 + 16.0 / 9.0 * t286 * t549 - 4.0 / 3.0 * t82 * t943);
        let tv4rho44 = t559 + t560 + 12.0 * t453 + 12.0 * t495 + 4.0 * t556 + t7 * (t622 + t665 + 4.0 * t727 + 6.0 * t805 + 4.0 * t876 + t69 * (t947 + t962) * t90);
        v4rho4[ip * 5 + 4] += tv4rho44;

    }
}
