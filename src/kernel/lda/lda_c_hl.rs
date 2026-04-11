//! LDA_C_HL kernel functions translated from libxc maple2c.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_hl.c`.
//! Translation preserves exact maple2c variable names and operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use crate::math::powers::{pow_1_3};
use crate::math::piecewise::{piecewise3};

// ============================================================================
// UNPOLARIZED FUNCTIONS
// ============================================================================

/// LDA_C_HL exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_hl_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_hl_c_0: f64,
    param_hl_c_1: f64,
    param_hl_r_0: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = param_hl_c_0;
        let t2 = 1.0 / M_PI;
        let t4 = t2 / rho[ip];
        let t5 = param_hl_r_0;
        let t6 = t5 * t5;
        let t7 = t6 * t5;
        let t8 = 1.0 / t7;
        let t11 = 1.0 + 3.0 / 4.0 * t4 * t8;
        let t12 = M_CBRT3;
        let t13 = t12 * t12;
        let t14 = pow_1_3(t2);
        let t15 = 1.0 / t14;
        let t16 = t13 * t15;
        let t17 = M_CBRT4;
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t23 = 1.0 + t16 * t19 * t5 / 3.0;
        let t24 = f64::ln(t23);
        let t26 = t14 * t14;
        let t27 = t13 * t26;
        let t28 = t18 * t18;
        let t30 = t17 / t28;
        let t31 = 1.0 / t6;
        let t35 = t12 * t14;
        let t36 = t17 * t17;
        let t38 = t36 / t18;
        let t39 = 1.0 / t5;
        let t44 = t1 * (t11 * t24 - t27 * t30 * t31 / 4.0 + t35 * t38 * t39 / 8.0 - 1.0 / 3.0);
        let t46 = pow_1_3(zeta_threshold);
        let t48 = piecewise3(1.0 <= zeta_threshold, t46 * zeta_threshold, 1.0);
        let t51 = M_CBRT2;
        let t55 = (2.0 * t48 - 2.0) / (2.0 * t51 - 2.0);
        let t56 = param_hl_c_1;
        let t57 = param_hl_r_1;
        let t58 = t57 * t57;
        let t59 = t58 * t57;
        let t60 = 1.0 / t59;
        let t63 = 1.0 + 3.0 / 4.0 * t4 * t60;
        let t67 = 1.0 + t16 * t19 * t57 / 3.0;
        let t68 = f64::ln(t67);
        let t70 = 1.0 / t58;
        let t74 = 1.0 / t57;
        let t81 = t55 * (-t56 * (t63 * t68 - t27 * t30 * t70 / 4.0 + t35 * t38 * t74 / 8.0 - 1.0 / 3.0) + t44);
        let tzk0 = -t44 + t81;
        zk[ip] += tzk0;
    }
}

/// LDA_C_HL vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_hl_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_hl_c_0: f64,
    param_hl_c_1: f64,
    param_hl_r_0: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = param_hl_c_0;
        let t2 = 1.0 / M_PI;
        let t4 = t2 / rho[ip];
        let t5 = param_hl_r_0;
        let t6 = t5 * t5;
        let t7 = t6 * t5;
        let t8 = 1.0 / t7;
        let t11 = 1.0 + 3.0 / 4.0 * t4 * t8;
        let t12 = M_CBRT3;
        let t13 = t12 * t12;
        let t14 = pow_1_3(t2);
        let t15 = 1.0 / t14;
        let t16 = t13 * t15;
        let t17 = M_CBRT4;
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t23 = 1.0 + t16 * t19 * t5 / 3.0;
        let t24 = f64::ln(t23);
        let t26 = t14 * t14;
        let t27 = t13 * t26;
        let t28 = t18 * t18;
        let t30 = t17 / t28;
        let t31 = 1.0 / t6;
        let t35 = t12 * t14;
        let t36 = t17 * t17;
        let t38 = t36 / t18;
        let t39 = 1.0 / t5;
        let t44 = t1 * (t11 * t24 - t27 * t30 * t31 / 4.0 + t35 * t38 * t39 / 8.0 - 1.0 / 3.0);
        let t46 = pow_1_3(zeta_threshold);
        let t48 = piecewise3(1.0 <= zeta_threshold, t46 * zeta_threshold, 1.0);
        let t51 = M_CBRT2;
        let t55 = (2.0 * t48 - 2.0) / (2.0 * t51 - 2.0);
        let t56 = param_hl_c_1;
        let t57 = param_hl_r_1;
        let t58 = t57 * t57;
        let t59 = t58 * t57;
        let t60 = 1.0 / t59;
        let t63 = 1.0 + 3.0 / 4.0 * t4 * t60;
        let t67 = 1.0 + t16 * t19 * t57 / 3.0;
        let t68 = f64::ln(t67);
        let t70 = 1.0 / t58;
        let t74 = 1.0 / t57;
        let t81 = t55 * (-t56 * (t63 * t68 - t27 * t30 * t70 / 4.0 + t35 * t38 * t74 / 8.0 - 1.0 / 3.0) + t44);
        let tzk0 = -t44 + t81;
        zk[ip] += tzk0;
        let t82 = rho[ip] * rho[ip];
        let t83 = 1.0 / t82;
        let t84 = t2 * t83;
        let t85 = t8 * t24;
        let t89 = t11 * t13 * t15;
        let t90 = 1.0 / t23;
        let t91 = t5 * t90;
        let t97 = t17 / t28 / rho[ip];
        let t103 = t36 / t18 / rho[ip];
        let t108 = t1 * (-3.0 / 4.0 * t84 * t85 + t89 * t30 * t91 / 9.0 + t27 * t97 * t31 / 6.0 - t35 * t103 * t39 / 24.0);
        let t109 = t60 * t68;
        let t113 = t63 * t13 * t15;
        let t114 = 1.0 / t67;
        let t115 = t57 * t114;
        let t128 = t55 * (-t56 * (-3.0 / 4.0 * t84 * t109 + t113 * t30 * t115 / 9.0 + t27 * t97 * t70 / 6.0 - t35 * t103 * t74 / 24.0) + t108);
        let tvrho0 = -t44 + t81 + rho[ip] * (-t108 + t128);
        vrho[ip] += tvrho0;
    }
}

/// LDA_C_HL fxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_hl_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_hl_c_0: f64,
    param_hl_c_1: f64,
    param_hl_r_0: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = param_hl_c_0;
        let t2 = 1.0 / M_PI;
        let t4 = t2 / rho[ip];
        let t5 = param_hl_r_0;
        let t6 = t5 * t5;
        let t7 = t6 * t5;
        let t8 = 1.0 / t7;
        let t11 = 1.0 + 3.0 / 4.0 * t4 * t8;
        let t12 = M_CBRT3;
        let t13 = t12 * t12;
        let t14 = pow_1_3(t2);
        let t15 = 1.0 / t14;
        let t16 = t13 * t15;
        let t17 = M_CBRT4;
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t23 = 1.0 + t16 * t19 * t5 / 3.0;
        let t24 = f64::ln(t23);
        let t26 = t14 * t14;
        let t27 = t13 * t26;
        let t28 = t18 * t18;
        let t30 = t17 / t28;
        let t31 = 1.0 / t6;
        let t35 = t12 * t14;
        let t36 = t17 * t17;
        let t38 = t36 / t18;
        let t39 = 1.0 / t5;
        let t44 = t1 * (t11 * t24 - t27 * t30 * t31 / 4.0 + t35 * t38 * t39 / 8.0 - 1.0 / 3.0);
        let t46 = pow_1_3(zeta_threshold);
        let t48 = piecewise3(1.0 <= zeta_threshold, t46 * zeta_threshold, 1.0);
        let t51 = M_CBRT2;
        let t55 = (2.0 * t48 - 2.0) / (2.0 * t51 - 2.0);
        let t56 = param_hl_c_1;
        let t57 = param_hl_r_1;
        let t58 = t57 * t57;
        let t59 = t58 * t57;
        let t60 = 1.0 / t59;
        let t63 = 1.0 + 3.0 / 4.0 * t4 * t60;
        let t67 = 1.0 + t16 * t19 * t57 / 3.0;
        let t68 = f64::ln(t67);
        let t70 = 1.0 / t58;
        let t74 = 1.0 / t57;
        let t81 = t55 * (-t56 * (t63 * t68 - t27 * t30 * t70 / 4.0 + t35 * t38 * t74 / 8.0 - 1.0 / 3.0) + t44);
        let tzk0 = -t44 + t81;
        zk[ip] += tzk0;
        let t82 = rho[ip] * rho[ip];
        let t83 = 1.0 / t82;
        let t84 = t2 * t83;
        let t85 = t8 * t24;
        let t89 = t11 * t13 * t15;
        let t90 = 1.0 / t23;
        let t91 = t5 * t90;
        let t97 = t17 / t28 / rho[ip];
        let t103 = t36 / t18 / rho[ip];
        let t108 = t1 * (-3.0 / 4.0 * t84 * t85 + t89 * t30 * t91 / 9.0 + t27 * t97 * t31 / 6.0 - t35 * t103 * t39 / 24.0);
        let t109 = t60 * t68;
        let t113 = t63 * t13 * t15;
        let t114 = 1.0 / t67;
        let t115 = t57 * t114;
        let t128 = t55 * (-t56 * (-3.0 / 4.0 * t84 * t109 + t113 * t30 * t115 / 9.0 + t27 * t97 * t70 / 6.0 - t35 * t103 * t74 / 24.0) + t108);
        let tvrho0 = -t44 + t81 + rho[ip] * (-t108 + t128);
        vrho[ip] += tvrho0;
        let t133 = t82 * rho[ip];
        let t134 = 1.0 / t133;
        let t135 = t2 * t134;
        let t139 = 1.0 / t28 / t82;
        let t140 = t2 * t139;
        let t143 = t16 * t17 * t90;
        let t150 = 1.0 / t26;
        let t151 = t11 * t12 * t150;
        let t152 = t23 * t23;
        let t153 = 1.0 / t152;
        let t154 = t6 * t153;
        let t158 = t17 * t139;
        let t164 = t36 / t18 / t82;
        let t169 = t1 * (3.0 / 2.0 * t135 * t85 - t140 * t31 * t143 / 6.0 - 2.0 / 27.0 * t89 * t97 * t91 - t151 * t103 * t154 / 27.0 - 5.0 / 18.0 * t27 * t158 * t31 + t35 * t164 * t39 / 18.0);
        let t174 = t16 * t17 * t114;
        let t181 = t63 * t12 * t150;
        let t182 = t67 * t67;
        let t183 = 1.0 / t182;
        let t184 = t58 * t183;
        let t197 = t55 * (-t56 * (3.0 / 2.0 * t135 * t109 - t140 * t70 * t174 / 6.0 - 2.0 / 27.0 * t113 * t97 * t115 - t181 * t103 * t184 / 27.0 - 5.0 / 18.0 * t27 * t158 * t70 + t35 * t164 * t74 / 18.0) + t169);
        let tv2rho20 = -2.0 * t108 + 2.0 * t128 + rho[ip] * (-t169 + t197);
        v2rho2[ip] += tv2rho20;
    }
}

/// LDA_C_HL kxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_hl_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_hl_c_0: f64,
    param_hl_c_1: f64,
    param_hl_r_0: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = param_hl_c_0;
        let t2 = 1.0 / M_PI;
        let t4 = t2 / rho[ip];
        let t5 = param_hl_r_0;
        let t6 = t5 * t5;
        let t7 = t6 * t5;
        let t8 = 1.0 / t7;
        let t11 = 1.0 + 3.0 / 4.0 * t4 * t8;
        let t12 = M_CBRT3;
        let t13 = t12 * t12;
        let t14 = pow_1_3(t2);
        let t15 = 1.0 / t14;
        let t16 = t13 * t15;
        let t17 = M_CBRT4;
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t23 = 1.0 + t16 * t19 * t5 / 3.0;
        let t24 = f64::ln(t23);
        let t26 = t14 * t14;
        let t27 = t13 * t26;
        let t28 = t18 * t18;
        let t30 = t17 / t28;
        let t31 = 1.0 / t6;
        let t35 = t12 * t14;
        let t36 = t17 * t17;
        let t38 = t36 / t18;
        let t39 = 1.0 / t5;
        let t44 = t1 * (t11 * t24 - t27 * t30 * t31 / 4.0 + t35 * t38 * t39 / 8.0 - 1.0 / 3.0);
        let t46 = pow_1_3(zeta_threshold);
        let t48 = piecewise3(1.0 <= zeta_threshold, t46 * zeta_threshold, 1.0);
        let t51 = M_CBRT2;
        let t55 = (2.0 * t48 - 2.0) / (2.0 * t51 - 2.0);
        let t56 = param_hl_c_1;
        let t57 = param_hl_r_1;
        let t58 = t57 * t57;
        let t59 = t58 * t57;
        let t60 = 1.0 / t59;
        let t63 = 1.0 + 3.0 / 4.0 * t4 * t60;
        let t67 = 1.0 + t16 * t19 * t57 / 3.0;
        let t68 = f64::ln(t67);
        let t70 = 1.0 / t58;
        let t74 = 1.0 / t57;
        let t81 = t55 * (-t56 * (t63 * t68 - t27 * t30 * t70 / 4.0 + t35 * t38 * t74 / 8.0 - 1.0 / 3.0) + t44);
        let tzk0 = -t44 + t81;
        zk[ip] += tzk0;
        let t82 = rho[ip] * rho[ip];
        let t83 = 1.0 / t82;
        let t84 = t2 * t83;
        let t85 = t8 * t24;
        let t89 = t11 * t13 * t15;
        let t90 = 1.0 / t23;
        let t91 = t5 * t90;
        let t97 = t17 / t28 / rho[ip];
        let t103 = t36 / t18 / rho[ip];
        let t108 = t1 * (-3.0 / 4.0 * t84 * t85 + t89 * t30 * t91 / 9.0 + t27 * t97 * t31 / 6.0 - t35 * t103 * t39 / 24.0);
        let t109 = t60 * t68;
        let t113 = t63 * t13 * t15;
        let t114 = 1.0 / t67;
        let t115 = t57 * t114;
        let t128 = t55 * (-t56 * (-3.0 / 4.0 * t84 * t109 + t113 * t30 * t115 / 9.0 + t27 * t97 * t70 / 6.0 - t35 * t103 * t74 / 24.0) + t108);
        let tvrho0 = -t44 + t81 + rho[ip] * (-t108 + t128);
        vrho[ip] += tvrho0;
        let t133 = t82 * rho[ip];
        let t134 = 1.0 / t133;
        let t135 = t2 * t134;
        let t139 = 1.0 / t28 / t82;
        let t140 = t2 * t139;
        let t143 = t16 * t17 * t90;
        let t150 = 1.0 / t26;
        let t151 = t11 * t12 * t150;
        let t152 = t23 * t23;
        let t153 = 1.0 / t152;
        let t154 = t6 * t153;
        let t158 = t17 * t139;
        let t164 = t36 / t18 / t82;
        let t169 = t1 * (3.0 / 2.0 * t135 * t85 - t140 * t31 * t143 / 6.0 - 2.0 / 27.0 * t89 * t97 * t91 - t151 * t103 * t154 / 27.0 - 5.0 / 18.0 * t27 * t158 * t31 + t35 * t164 * t39 / 18.0);
        let t174 = t16 * t17 * t114;
        let t181 = t63 * t12 * t150;
        let t182 = t67 * t67;
        let t183 = 1.0 / t182;
        let t184 = t58 * t183;
        let t197 = t55 * (-t56 * (3.0 / 2.0 * t135 * t109 - t140 * t70 * t174 / 6.0 - 2.0 / 27.0 * t113 * t97 * t115 - t181 * t103 * t184 / 27.0 - 5.0 / 18.0 * t27 * t158 * t70 + t35 * t164 * t74 / 18.0) + t169);
        let tv2rho20 = -2.0 * t108 + 2.0 * t128 + rho[ip] * (-t169 + t197);
        v2rho2[ip] += tv2rho20;
        let t202 = t82 * t82;
        let t203 = 1.0 / t202;
        let t204 = t2 * t203;
        let t208 = 1.0 / t28 / t133;
        let t209 = t2 * t208;
        let t214 = 1.0 / t18 / t133;
        let t215 = t2 * t214;
        let t217 = t12 * t150;
        let t219 = t217 * t36 * t153;
        let t228 = t11 * M_PI;
        let t231 = 1.0 / t152 / t23;
        let t235 = t17 * t208;
        let t239 = t36 * t214;
        let t244 = t1 * (-9.0 / 2.0 * t204 * t85 + 2.0 / 3.0 * t209 * t31 * t143 + t215 * t39 * t219 / 12.0 + 10.0 / 81.0 * t89 * t158 * t91 + 2.0 / 27.0 * t151 * t164 * t154 + 8.0 / 81.0 * t228 * t83 * t7 * t231 + 20.0 / 27.0 * t27 * t235 * t31 - 7.0 / 54.0 * t35 * t239 * t39);
        let t252 = t217 * t36 * t183;
        let t261 = t63 * M_PI;
        let t264 = 1.0 / t182 / t67;
        let t277 = t55 * (-t56 * (-9.0 / 2.0 * t204 * t109 + 2.0 / 3.0 * t209 * t70 * t174 + t215 * t74 * t252 / 12.0 + 10.0 / 81.0 * t113 * t158 * t115 + 2.0 / 27.0 * t181 * t164 * t184 + 8.0 / 81.0 * t261 * t83 * t59 * t264 + 20.0 / 27.0 * t27 * t235 * t70 - 7.0 / 54.0 * t35 * t239 * t74) + t244);
        let tv3rho30 = -3.0 * t169 + 3.0 * t197 + rho[ip] * (-t244 + t277);
        v3rho3[ip] += tv3rho30;
    }
}

/// LDA_C_HL lxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_hl_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_hl_c_0: f64,
    param_hl_c_1: f64,
    param_hl_r_0: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = param_hl_c_0;
        let t2 = 1.0 / M_PI;
        let t4 = t2 / rho[ip];
        let t5 = param_hl_r_0;
        let t6 = t5 * t5;
        let t7 = t6 * t5;
        let t8 = 1.0 / t7;
        let t11 = 1.0 + 3.0 / 4.0 * t4 * t8;
        let t12 = M_CBRT3;
        let t13 = t12 * t12;
        let t14 = pow_1_3(t2);
        let t15 = 1.0 / t14;
        let t16 = t13 * t15;
        let t17 = M_CBRT4;
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t23 = 1.0 + t16 * t19 * t5 / 3.0;
        let t24 = f64::ln(t23);
        let t26 = t14 * t14;
        let t27 = t13 * t26;
        let t28 = t18 * t18;
        let t30 = t17 / t28;
        let t31 = 1.0 / t6;
        let t35 = t12 * t14;
        let t36 = t17 * t17;
        let t38 = t36 / t18;
        let t39 = 1.0 / t5;
        let t44 = t1 * (t11 * t24 - t27 * t30 * t31 / 4.0 + t35 * t38 * t39 / 8.0 - 1.0 / 3.0);
        let t46 = pow_1_3(zeta_threshold);
        let t48 = piecewise3(1.0 <= zeta_threshold, t46 * zeta_threshold, 1.0);
        let t51 = M_CBRT2;
        let t55 = (2.0 * t48 - 2.0) / (2.0 * t51 - 2.0);
        let t56 = param_hl_c_1;
        let t57 = param_hl_r_1;
        let t58 = t57 * t57;
        let t59 = t58 * t57;
        let t60 = 1.0 / t59;
        let t63 = 1.0 + 3.0 / 4.0 * t4 * t60;
        let t67 = 1.0 + t16 * t19 * t57 / 3.0;
        let t68 = f64::ln(t67);
        let t70 = 1.0 / t58;
        let t74 = 1.0 / t57;
        let t81 = t55 * (-t56 * (t63 * t68 - t27 * t30 * t70 / 4.0 + t35 * t38 * t74 / 8.0 - 1.0 / 3.0) + t44);
        let tzk0 = -t44 + t81;
        zk[ip] += tzk0;
        let t82 = rho[ip] * rho[ip];
        let t83 = 1.0 / t82;
        let t84 = t2 * t83;
        let t85 = t8 * t24;
        let t89 = t11 * t13 * t15;
        let t90 = 1.0 / t23;
        let t91 = t5 * t90;
        let t97 = t17 / t28 / rho[ip];
        let t103 = t36 / t18 / rho[ip];
        let t108 = t1 * (-3.0 / 4.0 * t84 * t85 + t89 * t30 * t91 / 9.0 + t27 * t97 * t31 / 6.0 - t35 * t103 * t39 / 24.0);
        let t109 = t60 * t68;
        let t113 = t63 * t13 * t15;
        let t114 = 1.0 / t67;
        let t115 = t57 * t114;
        let t128 = t55 * (-t56 * (-3.0 / 4.0 * t84 * t109 + t113 * t30 * t115 / 9.0 + t27 * t97 * t70 / 6.0 - t35 * t103 * t74 / 24.0) + t108);
        let tvrho0 = -t44 + t81 + rho[ip] * (-t108 + t128);
        vrho[ip] += tvrho0;
        let t133 = t82 * rho[ip];
        let t134 = 1.0 / t133;
        let t135 = t2 * t134;
        let t139 = 1.0 / t28 / t82;
        let t140 = t2 * t139;
        let t143 = t16 * t17 * t90;
        let t150 = 1.0 / t26;
        let t151 = t11 * t12 * t150;
        let t152 = t23 * t23;
        let t153 = 1.0 / t152;
        let t154 = t6 * t153;
        let t158 = t17 * t139;
        let t164 = t36 / t18 / t82;
        let t169 = t1 * (3.0 / 2.0 * t135 * t85 - t140 * t31 * t143 / 6.0 - 2.0 / 27.0 * t89 * t97 * t91 - t151 * t103 * t154 / 27.0 - 5.0 / 18.0 * t27 * t158 * t31 + t35 * t164 * t39 / 18.0);
        let t174 = t16 * t17 * t114;
        let t181 = t63 * t12 * t150;
        let t182 = t67 * t67;
        let t183 = 1.0 / t182;
        let t184 = t58 * t183;
        let t197 = t55 * (-t56 * (3.0 / 2.0 * t135 * t109 - t140 * t70 * t174 / 6.0 - 2.0 / 27.0 * t113 * t97 * t115 - t181 * t103 * t184 / 27.0 - 5.0 / 18.0 * t27 * t158 * t70 + t35 * t164 * t74 / 18.0) + t169);
        let tv2rho20 = -2.0 * t108 + 2.0 * t128 + rho[ip] * (-t169 + t197);
        v2rho2[ip] += tv2rho20;
        let t202 = t82 * t82;
        let t203 = 1.0 / t202;
        let t204 = t2 * t203;
        let t208 = 1.0 / t28 / t133;
        let t209 = t2 * t208;
        let t214 = 1.0 / t18 / t133;
        let t215 = t2 * t214;
        let t217 = t12 * t150;
        let t219 = t217 * t36 * t153;
        let t228 = t11 * M_PI;
        let t231 = 1.0 / t152 / t23;
        let t235 = t17 * t208;
        let t239 = t36 * t214;
        let t244 = t1 * (-9.0 / 2.0 * t204 * t85 + 2.0 / 3.0 * t209 * t31 * t143 + t215 * t39 * t219 / 12.0 + 10.0 / 81.0 * t89 * t158 * t91 + 2.0 / 27.0 * t151 * t164 * t154 + 8.0 / 81.0 * t228 * t83 * t7 * t231 + 20.0 / 27.0 * t27 * t235 * t31 - 7.0 / 54.0 * t35 * t239 * t39);
        let t252 = t217 * t36 * t183;
        let t261 = t63 * M_PI;
        let t264 = 1.0 / t182 / t67;
        let t277 = t55 * (-t56 * (-9.0 / 2.0 * t204 * t109 + 2.0 / 3.0 * t209 * t70 * t174 + t215 * t74 * t252 / 12.0 + 10.0 / 81.0 * t113 * t158 * t115 + 2.0 / 27.0 * t181 * t164 * t184 + 8.0 / 81.0 * t261 * t83 * t59 * t264 + 20.0 / 27.0 * t27 * t235 * t70 - 7.0 / 54.0 * t35 * t239 * t74) + t244);
        let tv3rho30 = -3.0 * t169 + 3.0 * t197 + rho[ip] * (-t244 + t277);
        v3rho3[ip] += tv3rho30;
        let t284 = t2 / t202 / rho[ip];
        let t288 = 1.0 / t28 / t202;
        let t289 = t2 * t288;
        let t294 = 1.0 / t18 / t202;
        let t295 = t2 * t294;
        let t311 = t6 * t6;
        let t314 = t152 * t152;
        let t317 = t15 * t17;
        let t321 = t17 * t288;
        let t325 = t36 * t294;
        let t330 = t1 * (18.0 * t284 * t85 - 82.0 / 27.0 * t289 * t31 * t143 - 5.0 / 9.0 * t295 * t39 * t219 - 8.0 / 27.0 * t203 * t231 - 80.0 / 243.0 * t89 * t235 * t91 - 52.0 / 243.0 * t151 * t239 * t154 - 32.0 / 81.0 * t228 * t134 * t7 * t231 - 8.0 / 243.0 * t228 * t139 * t311 / t314 * t13 * t317 - 220.0 / 81.0 * t27 * t321 * t31 + 35.0 / 81.0 * t35 * t325 * t39);
        let t351 = t58 * t58;
        let t354 = t182 * t182;
        let tv4rho40 = -4.0 * t244 + 4.0 * t277 + rho[ip] * (-t330 + t55 * (-t56 * (18.0 * t284 * t109 - 82.0 / 27.0 * t289 * t70 * t174 - 5.0 / 9.0 * t295 * t74 * t252 - 8.0 / 27.0 * t203 * t264 - 80.0 / 243.0 * t113 * t235 * t115 - 52.0 / 243.0 * t181 * t239 * t184 - 32.0 / 81.0 * t261 * t134 * t59 * t264 - 8.0 / 243.0 * t261 * t139 * t351 / t354 * t13 * t317 - 220.0 / 81.0 * t27 * t321 * t70 + 35.0 / 81.0 * t35 * t325 * t74) + t330));
        v4rho4[ip] += tv4rho40;
    }
}

// ============================================================================
// POLARIZED FUNCTIONS
// ============================================================================

/// LDA_C_HL exc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_hl_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_hl_c_0: f64,
    param_hl_c_1: f64,
    param_hl_r_0: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = param_hl_c_0;
        let t2 = 1.0 / M_PI;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = param_hl_r_0;
        let t7 = t6 * t6;
        let t8 = t7 * t6;
        let t9 = 1.0 / t8;
        let t12 = 1.0 + 3.0 / 4.0 * t5 * t9;
        let t13 = M_CBRT3;
        let t14 = t13 * t13;
        let t15 = pow_1_3(t2);
        let t16 = 1.0 / t15;
        let t17 = t14 * t16;
        let t18 = M_CBRT4;
        let t19 = pow_1_3(t3);
        let t20 = t18 * t19;
        let t24 = 1.0 + t17 * t20 * t6 / 3.0;
        let t25 = f64::ln(t24);
        let t27 = t15 * t15;
        let t28 = t14 * t27;
        let t29 = t19 * t19;
        let t31 = t18 / t29;
        let t32 = 1.0 / t7;
        let t36 = t13 * t15;
        let t37 = t18 * t18;
        let t39 = t37 / t19;
        let t40 = 1.0 / t6;
        let t45 = t1 * (t12 * t25 - t28 * t31 * t32 / 4.0 + t36 * t39 * t40 / 8.0 - 1.0 / 3.0);
        let t46 = rho0 - rho1;
        let t47 = t46 * t4;
        let t48 = 1.0 + t47;
        let t49 = t48 <= zeta_threshold;
        let t50 = pow_1_3(zeta_threshold);
        let t51 = t50 * zeta_threshold;
        let t52 = pow_1_3(t48);
        let t54 = piecewise3(t49, t51, t52 * t48);
        let t55 = 1.0 - t47;
        let t56 = t55 <= zeta_threshold;
        let t57 = pow_1_3(t55);
        let t59 = piecewise3(t56, t51, t57 * t55);
        let t61 = M_CBRT2;
        let t64 = 1.0 / (2.0 * t61 - 2.0);
        let t65 = (t54 + t59 - 2.0) * t64;
        let t66 = param_hl_c_1;
        let t67 = param_hl_r_1;
        let t68 = t67 * t67;
        let t69 = t68 * t67;
        let t70 = 1.0 / t69;
        let t73 = 1.0 + 3.0 / 4.0 * t5 * t70;
        let t77 = 1.0 + t17 * t20 * t67 / 3.0;
        let t78 = f64::ln(t77);
        let t80 = 1.0 / t68;
        let t84 = 1.0 / t67;
        let t90 = -t66 * (t73 * t78 - t28 * t31 * t80 / 4.0 + t36 * t39 * t84 / 8.0 - 1.0 / 3.0) + t45;
        let t91 = t65 * t90;
        let tzk0 = -t45 + t91;
        zk[ip] += tzk0;
    }
}

/// LDA_C_HL vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_hl_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_hl_c_0: f64,
    param_hl_c_1: f64,
    param_hl_r_0: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = param_hl_c_0;
        let t2 = 1.0 / M_PI;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = param_hl_r_0;
        let t7 = t6 * t6;
        let t8 = t7 * t6;
        let t9 = 1.0 / t8;
        let t12 = 1.0 + 3.0 / 4.0 * t5 * t9;
        let t13 = M_CBRT3;
        let t14 = t13 * t13;
        let t15 = pow_1_3(t2);
        let t16 = 1.0 / t15;
        let t17 = t14 * t16;
        let t18 = M_CBRT4;
        let t19 = pow_1_3(t3);
        let t20 = t18 * t19;
        let t24 = 1.0 + t17 * t20 * t6 / 3.0;
        let t25 = f64::ln(t24);
        let t27 = t15 * t15;
        let t28 = t14 * t27;
        let t29 = t19 * t19;
        let t31 = t18 / t29;
        let t32 = 1.0 / t7;
        let t36 = t13 * t15;
        let t37 = t18 * t18;
        let t39 = t37 / t19;
        let t40 = 1.0 / t6;
        let t45 = t1 * (t12 * t25 - t28 * t31 * t32 / 4.0 + t36 * t39 * t40 / 8.0 - 1.0 / 3.0);
        let t46 = rho0 - rho1;
        let t47 = t46 * t4;
        let t48 = 1.0 + t47;
        let t49 = t48 <= zeta_threshold;
        let t50 = pow_1_3(zeta_threshold);
        let t51 = t50 * zeta_threshold;
        let t52 = pow_1_3(t48);
        let t54 = piecewise3(t49, t51, t52 * t48);
        let t55 = 1.0 - t47;
        let t56 = t55 <= zeta_threshold;
        let t57 = pow_1_3(t55);
        let t59 = piecewise3(t56, t51, t57 * t55);
        let t61 = M_CBRT2;
        let t64 = 1.0 / (2.0 * t61 - 2.0);
        let t65 = (t54 + t59 - 2.0) * t64;
        let t66 = param_hl_c_1;
        let t67 = param_hl_r_1;
        let t68 = t67 * t67;
        let t69 = t68 * t67;
        let t70 = 1.0 / t69;
        let t73 = 1.0 + 3.0 / 4.0 * t5 * t70;
        let t77 = 1.0 + t17 * t20 * t67 / 3.0;
        let t78 = f64::ln(t77);
        let t80 = 1.0 / t68;
        let t84 = 1.0 / t67;
        let t90 = -t66 * (t73 * t78 - t28 * t31 * t80 / 4.0 + t36 * t39 * t84 / 8.0 - 1.0 / 3.0) + t45;
        let t91 = t65 * t90;
        let tzk0 = -t45 + t91;
        zk[ip] += tzk0;
        let t92 = t3 * t3;
        let t93 = 1.0 / t92;
        let t94 = t2 * t93;
        let t95 = t9 * t25;
        let t99 = t12 * t14 * t16;
        let t100 = 1.0 / t24;
        let t101 = t6 * t100;
        let t107 = t18 / t29 / t3;
        let t113 = t37 / t19 / t3;
        let t118 = t1 * (-3.0 / 4.0 * t94 * t95 + t99 * t31 * t101 / 9.0 + t28 * t107 * t32 / 6.0 - t36 * t113 * t40 / 24.0);
        let t119 = t46 * t93;
        let t120 = t4 - t119;
        let t123 = piecewise3(t49, 0.0, 4.0 / 3.0 * t52 * t120);
        let t124 = -t120;
        let t127 = piecewise3(t56, 0.0, 4.0 / 3.0 * t57 * t124);
        let t129 = (t123 + t127) * t64;
        let t130 = t129 * t90;
        let t131 = t70 * t78;
        let t135 = t73 * t14 * t16;
        let t136 = 1.0 / t77;
        let t137 = t67 * t136;
        let t149 = -t66 * (-3.0 / 4.0 * t94 * t131 + t135 * t31 * t137 / 9.0 + t28 * t107 * t80 / 6.0 - t36 * t113 * t84 / 24.0) + t118;
        let t150 = t65 * t149;
        let tvrho0 = -t45 + t91 + t3 * (-t118 + t130 + t150);
        vrho[ip * 2] += tvrho0;
        let t153 = -t4 - t119;
        let t156 = piecewise3(t49, 0.0, 4.0 / 3.0 * t52 * t153);
        let t157 = -t153;
        let t160 = piecewise3(t56, 0.0, 4.0 / 3.0 * t57 * t157);
        let t162 = (t156 + t160) * t64;
        let t163 = t162 * t90;
        let tvrho1 = -t45 + t91 + t3 * (-t118 + t163 + t150);
        vrho[ip * 2 + 1] += tvrho1;
    }
}

/// LDA_C_HL fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_hl_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_hl_c_0: f64,
    param_hl_c_1: f64,
    param_hl_r_0: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = param_hl_c_0;
        let t2 = 1.0 / M_PI;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = param_hl_r_0;
        let t7 = t6 * t6;
        let t8 = t7 * t6;
        let t9 = 1.0 / t8;
        let t12 = 1.0 + 3.0 / 4.0 * t5 * t9;
        let t13 = M_CBRT3;
        let t14 = t13 * t13;
        let t15 = pow_1_3(t2);
        let t16 = 1.0 / t15;
        let t17 = t14 * t16;
        let t18 = M_CBRT4;
        let t19 = pow_1_3(t3);
        let t20 = t18 * t19;
        let t24 = 1.0 + t17 * t20 * t6 / 3.0;
        let t25 = f64::ln(t24);
        let t27 = t15 * t15;
        let t28 = t14 * t27;
        let t29 = t19 * t19;
        let t31 = t18 / t29;
        let t32 = 1.0 / t7;
        let t36 = t13 * t15;
        let t37 = t18 * t18;
        let t39 = t37 / t19;
        let t40 = 1.0 / t6;
        let t45 = t1 * (t12 * t25 - t28 * t31 * t32 / 4.0 + t36 * t39 * t40 / 8.0 - 1.0 / 3.0);
        let t46 = rho0 - rho1;
        let t47 = t46 * t4;
        let t48 = 1.0 + t47;
        let t49 = t48 <= zeta_threshold;
        let t50 = pow_1_3(zeta_threshold);
        let t51 = t50 * zeta_threshold;
        let t52 = pow_1_3(t48);
        let t54 = piecewise3(t49, t51, t52 * t48);
        let t55 = 1.0 - t47;
        let t56 = t55 <= zeta_threshold;
        let t57 = pow_1_3(t55);
        let t59 = piecewise3(t56, t51, t57 * t55);
        let t61 = M_CBRT2;
        let t64 = 1.0 / (2.0 * t61 - 2.0);
        let t65 = (t54 + t59 - 2.0) * t64;
        let t66 = param_hl_c_1;
        let t67 = param_hl_r_1;
        let t68 = t67 * t67;
        let t69 = t68 * t67;
        let t70 = 1.0 / t69;
        let t73 = 1.0 + 3.0 / 4.0 * t5 * t70;
        let t77 = 1.0 + t17 * t20 * t67 / 3.0;
        let t78 = f64::ln(t77);
        let t80 = 1.0 / t68;
        let t84 = 1.0 / t67;
        let t90 = -t66 * (t73 * t78 - t28 * t31 * t80 / 4.0 + t36 * t39 * t84 / 8.0 - 1.0 / 3.0) + t45;
        let t91 = t65 * t90;
        let tzk0 = -t45 + t91;
        zk[ip] += tzk0;
        let t92 = t3 * t3;
        let t93 = 1.0 / t92;
        let t94 = t2 * t93;
        let t95 = t9 * t25;
        let t99 = t12 * t14 * t16;
        let t100 = 1.0 / t24;
        let t101 = t6 * t100;
        let t107 = t18 / t29 / t3;
        let t113 = t37 / t19 / t3;
        let t118 = t1 * (-3.0 / 4.0 * t94 * t95 + t99 * t31 * t101 / 9.0 + t28 * t107 * t32 / 6.0 - t36 * t113 * t40 / 24.0);
        let t119 = t46 * t93;
        let t120 = t4 - t119;
        let t123 = piecewise3(t49, 0.0, 4.0 / 3.0 * t52 * t120);
        let t124 = -t120;
        let t127 = piecewise3(t56, 0.0, 4.0 / 3.0 * t57 * t124);
        let t129 = (t123 + t127) * t64;
        let t130 = t129 * t90;
        let t131 = t70 * t78;
        let t135 = t73 * t14 * t16;
        let t136 = 1.0 / t77;
        let t137 = t67 * t136;
        let t149 = -t66 * (-3.0 / 4.0 * t94 * t131 + t135 * t31 * t137 / 9.0 + t28 * t107 * t80 / 6.0 - t36 * t113 * t84 / 24.0) + t118;
        let t150 = t65 * t149;
        let tvrho0 = -t45 + t91 + t3 * (-t118 + t130 + t150);
        vrho[ip * 2] += tvrho0;
        let t153 = -t4 - t119;
        let t156 = piecewise3(t49, 0.0, 4.0 / 3.0 * t52 * t153);
        let t157 = -t153;
        let t160 = piecewise3(t56, 0.0, 4.0 / 3.0 * t57 * t157);
        let t162 = (t156 + t160) * t64;
        let t163 = t162 * t90;
        let tvrho1 = -t45 + t91 + t3 * (-t118 + t163 + t150);
        vrho[ip * 2 + 1] += tvrho1;
        let t166 = 2.0 * t118;
        let t168 = 2.0 * t150;
        let t169 = t92 * t3;
        let t170 = 1.0 / t169;
        let t171 = t2 * t170;
        let t175 = 1.0 / t29 / t92;
        let t176 = t2 * t175;
        let t179 = t17 * t18 * t100;
        let t186 = 1.0 / t27;
        let t187 = t12 * t13 * t186;
        let t188 = t24 * t24;
        let t189 = 1.0 / t188;
        let t190 = t7 * t189;
        let t194 = t18 * t175;
        let t200 = t37 / t19 / t92;
        let t205 = t1 * (3.0 / 2.0 * t171 * t95 - t176 * t32 * t179 / 6.0 - 2.0 / 27.0 * t99 * t107 * t101 - t187 * t113 * t190 / 27.0 - 5.0 / 18.0 * t28 * t194 * t32 + t36 * t200 * t40 / 18.0);
        let t206 = t52 * t52;
        let t207 = 1.0 / t206;
        let t208 = t120 * t120;
        let t211 = t46 * t170;
        let t213 = -2.0 * t93 + 2.0 * t211;
        let t217 = piecewise3(t49, 0.0, 4.0 / 9.0 * t207 * t208 + 4.0 / 3.0 * t52 * t213);
        let t218 = t57 * t57;
        let t219 = 1.0 / t218;
        let t220 = t124 * t124;
        let t223 = -t213;
        let t227 = piecewise3(t56, 0.0, 4.0 / 9.0 * t219 * t220 + 4.0 / 3.0 * t57 * t223);
        let t229 = (t217 + t227) * t64;
        let t230 = t229 * t90;
        let t231 = t129 * t149;
        let t232 = 2.0 * t231;
        let t237 = t17 * t18 * t136;
        let t244 = t73 * t13 * t186;
        let t245 = t77 * t77;
        let t246 = 1.0 / t245;
        let t247 = t68 * t246;
        let t259 = -t66 * (3.0 / 2.0 * t171 * t131 - t176 * t80 * t237 / 6.0 - 2.0 / 27.0 * t135 * t107 * t137 - t244 * t113 * t247 / 27.0 - 5.0 / 18.0 * t28 * t194 * t80 + t36 * t200 * t84 / 18.0) + t205;
        let t260 = t65 * t259;
        let tv2rho20 = -t166 + 2.0 * t130 + t168 + t3 * (-t205 + t230 + t232 + t260);
        v2rho2[ip * 3] += tv2rho20;
        let t263 = t207 * t153;
        let t266 = t52 * t46;
        let t270 = piecewise3(t49, 0.0, 4.0 / 9.0 * t263 * t120 + 8.0 / 3.0 * t266 * t170);
        let t271 = t219 * t157;
        let t274 = t57 * t46;
        let t278 = piecewise3(t56, 0.0, 4.0 / 9.0 * t271 * t124 - 8.0 / 3.0 * t274 * t170);
        let t280 = (t270 + t278) * t64;
        let t281 = t280 * t90;
        let t282 = t162 * t149;
        let tv2rho21 = -t166 + t130 + t168 + t163 + t3 * (-t205 + t281 + t282 + t231 + t260);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t286 = t153 * t153;
        let t290 = 2.0 * t93 + 2.0 * t211;
        let t294 = piecewise3(t49, 0.0, 4.0 / 9.0 * t207 * t286 + 4.0 / 3.0 * t52 * t290);
        let t295 = t157 * t157;
        let t298 = -t290;
        let t302 = piecewise3(t56, 0.0, 4.0 / 9.0 * t219 * t295 + 4.0 / 3.0 * t57 * t298);
        let t304 = (t294 + t302) * t64;
        let t305 = t304 * t90;
        let t306 = 2.0 * t282;
        let tv2rho22 = -t166 + 2.0 * t163 + t168 + t3 * (-t205 + t305 + t306 + t260);
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}

/// LDA_C_HL kxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_hl_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_hl_c_0: f64,
    param_hl_c_1: f64,
    param_hl_r_0: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = param_hl_c_0;
        let t2 = 1.0 / M_PI;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = param_hl_r_0;
        let t7 = t6 * t6;
        let t8 = t7 * t6;
        let t9 = 1.0 / t8;
        let t12 = 1.0 + 3.0 / 4.0 * t5 * t9;
        let t13 = M_CBRT3;
        let t14 = t13 * t13;
        let t15 = pow_1_3(t2);
        let t16 = 1.0 / t15;
        let t17 = t14 * t16;
        let t18 = M_CBRT4;
        let t19 = pow_1_3(t3);
        let t20 = t18 * t19;
        let t24 = 1.0 + t17 * t20 * t6 / 3.0;
        let t25 = f64::ln(t24);
        let t27 = t15 * t15;
        let t28 = t14 * t27;
        let t29 = t19 * t19;
        let t31 = t18 / t29;
        let t32 = 1.0 / t7;
        let t36 = t13 * t15;
        let t37 = t18 * t18;
        let t39 = t37 / t19;
        let t40 = 1.0 / t6;
        let t45 = t1 * (t12 * t25 - t28 * t31 * t32 / 4.0 + t36 * t39 * t40 / 8.0 - 1.0 / 3.0);
        let t46 = rho0 - rho1;
        let t47 = t46 * t4;
        let t48 = 1.0 + t47;
        let t49 = t48 <= zeta_threshold;
        let t50 = pow_1_3(zeta_threshold);
        let t51 = t50 * zeta_threshold;
        let t52 = pow_1_3(t48);
        let t54 = piecewise3(t49, t51, t52 * t48);
        let t55 = 1.0 - t47;
        let t56 = t55 <= zeta_threshold;
        let t57 = pow_1_3(t55);
        let t59 = piecewise3(t56, t51, t57 * t55);
        let t61 = M_CBRT2;
        let t64 = 1.0 / (2.0 * t61 - 2.0);
        let t65 = (t54 + t59 - 2.0) * t64;
        let t66 = param_hl_c_1;
        let t67 = param_hl_r_1;
        let t68 = t67 * t67;
        let t69 = t68 * t67;
        let t70 = 1.0 / t69;
        let t73 = 1.0 + 3.0 / 4.0 * t5 * t70;
        let t77 = 1.0 + t17 * t20 * t67 / 3.0;
        let t78 = f64::ln(t77);
        let t80 = 1.0 / t68;
        let t84 = 1.0 / t67;
        let t90 = -t66 * (t73 * t78 - t28 * t31 * t80 / 4.0 + t36 * t39 * t84 / 8.0 - 1.0 / 3.0) + t45;
        let t91 = t65 * t90;
        let tzk0 = -t45 + t91;
        zk[ip] += tzk0;
        let t92 = t3 * t3;
        let t93 = 1.0 / t92;
        let t94 = t2 * t93;
        let t95 = t9 * t25;
        let t99 = t12 * t14 * t16;
        let t100 = 1.0 / t24;
        let t101 = t6 * t100;
        let t107 = t18 / t29 / t3;
        let t113 = t37 / t19 / t3;
        let t118 = t1 * (-3.0 / 4.0 * t94 * t95 + t99 * t31 * t101 / 9.0 + t28 * t107 * t32 / 6.0 - t36 * t113 * t40 / 24.0);
        let t119 = t46 * t93;
        let t120 = t4 - t119;
        let t123 = piecewise3(t49, 0.0, 4.0 / 3.0 * t52 * t120);
        let t124 = -t120;
        let t127 = piecewise3(t56, 0.0, 4.0 / 3.0 * t57 * t124);
        let t129 = (t123 + t127) * t64;
        let t130 = t129 * t90;
        let t131 = t70 * t78;
        let t135 = t73 * t14 * t16;
        let t136 = 1.0 / t77;
        let t137 = t67 * t136;
        let t149 = -t66 * (-3.0 / 4.0 * t94 * t131 + t135 * t31 * t137 / 9.0 + t28 * t107 * t80 / 6.0 - t36 * t113 * t84 / 24.0) + t118;
        let t150 = t65 * t149;
        let tvrho0 = -t45 + t91 + t3 * (-t118 + t130 + t150);
        vrho[ip * 2] += tvrho0;
        let t153 = -t4 - t119;
        let t156 = piecewise3(t49, 0.0, 4.0 / 3.0 * t52 * t153);
        let t157 = -t153;
        let t160 = piecewise3(t56, 0.0, 4.0 / 3.0 * t57 * t157);
        let t162 = (t156 + t160) * t64;
        let t163 = t162 * t90;
        let tvrho1 = -t45 + t91 + t3 * (-t118 + t163 + t150);
        vrho[ip * 2 + 1] += tvrho1;
        let t166 = 2.0 * t118;
        let t168 = 2.0 * t150;
        let t169 = t92 * t3;
        let t170 = 1.0 / t169;
        let t171 = t2 * t170;
        let t175 = 1.0 / t29 / t92;
        let t176 = t2 * t175;
        let t179 = t17 * t18 * t100;
        let t186 = 1.0 / t27;
        let t187 = t12 * t13 * t186;
        let t188 = t24 * t24;
        let t189 = 1.0 / t188;
        let t190 = t7 * t189;
        let t194 = t18 * t175;
        let t200 = t37 / t19 / t92;
        let t205 = t1 * (3.0 / 2.0 * t171 * t95 - t176 * t32 * t179 / 6.0 - 2.0 / 27.0 * t99 * t107 * t101 - t187 * t113 * t190 / 27.0 - 5.0 / 18.0 * t28 * t194 * t32 + t36 * t200 * t40 / 18.0);
        let t206 = t52 * t52;
        let t207 = 1.0 / t206;
        let t208 = t120 * t120;
        let t211 = t46 * t170;
        let t213 = -2.0 * t93 + 2.0 * t211;
        let t217 = piecewise3(t49, 0.0, 4.0 / 9.0 * t207 * t208 + 4.0 / 3.0 * t52 * t213);
        let t218 = t57 * t57;
        let t219 = 1.0 / t218;
        let t220 = t124 * t124;
        let t223 = -t213;
        let t227 = piecewise3(t56, 0.0, 4.0 / 9.0 * t219 * t220 + 4.0 / 3.0 * t57 * t223);
        let t229 = (t217 + t227) * t64;
        let t230 = t229 * t90;
        let t231 = t129 * t149;
        let t232 = 2.0 * t231;
        let t237 = t17 * t18 * t136;
        let t244 = t73 * t13 * t186;
        let t245 = t77 * t77;
        let t246 = 1.0 / t245;
        let t247 = t68 * t246;
        let t259 = -t66 * (3.0 / 2.0 * t171 * t131 - t176 * t80 * t237 / 6.0 - 2.0 / 27.0 * t135 * t107 * t137 - t244 * t113 * t247 / 27.0 - 5.0 / 18.0 * t28 * t194 * t80 + t36 * t200 * t84 / 18.0) + t205;
        let t260 = t65 * t259;
        let tv2rho20 = -t166 + 2.0 * t130 + t168 + t3 * (-t205 + t230 + t232 + t260);
        v2rho2[ip * 3] += tv2rho20;
        let t263 = t207 * t153;
        let t266 = t52 * t46;
        let t270 = piecewise3(t49, 0.0, 4.0 / 9.0 * t263 * t120 + 8.0 / 3.0 * t266 * t170);
        let t271 = t219 * t157;
        let t274 = t57 * t46;
        let t278 = piecewise3(t56, 0.0, 4.0 / 9.0 * t271 * t124 - 8.0 / 3.0 * t274 * t170);
        let t280 = (t270 + t278) * t64;
        let t281 = t280 * t90;
        let t282 = t162 * t149;
        let tv2rho21 = -t166 + t130 + t168 + t163 + t3 * (-t205 + t281 + t282 + t231 + t260);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t286 = t153 * t153;
        let t290 = 2.0 * t93 + 2.0 * t211;
        let t294 = piecewise3(t49, 0.0, 4.0 / 9.0 * t207 * t286 + 4.0 / 3.0 * t52 * t290);
        let t295 = t157 * t157;
        let t298 = -t290;
        let t302 = piecewise3(t56, 0.0, 4.0 / 9.0 * t219 * t295 + 4.0 / 3.0 * t57 * t298);
        let t304 = (t294 + t302) * t64;
        let t305 = t304 * t90;
        let t306 = 2.0 * t282;
        let tv2rho22 = -t166 + 2.0 * t163 + t168 + t3 * (-t205 + t305 + t306 + t260);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t309 = 3.0 * t205;
        let t312 = 3.0 * t260;
        let t313 = t92 * t92;
        let t314 = 1.0 / t313;
        let t315 = t2 * t314;
        let t319 = 1.0 / t29 / t169;
        let t320 = t2 * t319;
        let t325 = 1.0 / t19 / t169;
        let t326 = t2 * t325;
        let t328 = t13 * t186;
        let t330 = t328 * t37 * t189;
        let t339 = t12 * M_PI;
        let t342 = 1.0 / t188 / t24;
        let t346 = t18 * t319;
        let t350 = t37 * t325;
        let t355 = t1 * (-9.0 / 2.0 * t315 * t95 + 2.0 / 3.0 * t320 * t32 * t179 + t326 * t40 * t330 / 12.0 + 10.0 / 81.0 * t99 * t194 * t101 + 2.0 / 27.0 * t187 * t200 * t190 + 8.0 / 81.0 * t339 * t93 * t8 * t342 + 20.0 / 27.0 * t28 * t346 * t32 - 7.0 / 54.0 * t36 * t350 * t40);
        let t357 = 1.0 / t206 / t48;
        let t358 = t208 * t120;
        let t361 = t207 * t120;
        let t364 = t46 * t314;
        let t366 = 6.0 * t170 - 6.0 * t364;
        let t370 = piecewise3(t49, 0.0, -8.0 / 27.0 * t357 * t358 + 4.0 / 3.0 * t361 * t213 + 4.0 / 3.0 * t52 * t366);
        let t372 = 1.0 / t218 / t55;
        let t373 = t220 * t124;
        let t376 = t219 * t124;
        let t379 = -t366;
        let t383 = piecewise3(t56, 0.0, -8.0 / 27.0 * t372 * t373 + 4.0 / 3.0 * t376 * t223 + 4.0 / 3.0 * t57 * t379);
        let t385 = (t370 + t383) * t64;
        let t386 = t385 * t90;
        let t387 = t229 * t149;
        let t389 = t129 * t259;
        let t390 = 3.0 * t389;
        let t398 = t328 * t37 * t246;
        let t407 = t73 * M_PI;
        let t410 = 1.0 / t245 / t77;
        let t422 = -t66 * (-9.0 / 2.0 * t315 * t131 + 2.0 / 3.0 * t320 * t80 * t237 + t326 * t84 * t398 / 12.0 + 10.0 / 81.0 * t135 * t194 * t137 + 2.0 / 27.0 * t244 * t200 * t247 + 8.0 / 81.0 * t407 * t93 * t69 * t410 + 20.0 / 27.0 * t28 * t346 * t80 - 7.0 / 54.0 * t36 * t350 * t84) + t355;
        let t423 = t65 * t422;
        let tv3rho30 = -t309 + 3.0 * t230 + 6.0 * t231 + t312 + t3 * (-t355 + t386 + 3.0 * t387 + t390 + t423);
        v3rho3[ip * 4] += tv3rho30;
        let t427 = 2.0 * t281;
        let t428 = t357 * t153;
        let t431 = t207 * t46;
        let t442 = piecewise3(t49, 0.0, -8.0 / 27.0 * t428 * t208 + 16.0 / 9.0 * t431 * t170 * t120 + 4.0 / 9.0 * t263 * t213 + 8.0 / 3.0 * t52 * t170 - 8.0 * t266 * t314);
        let t443 = t372 * t157;
        let t446 = t219 * t46;
        let t457 = piecewise3(t56, 0.0, -8.0 / 27.0 * t443 * t220 - 16.0 / 9.0 * t446 * t170 * t124 + 4.0 / 9.0 * t271 * t223 - 8.0 / 3.0 * t57 * t170 + 8.0 * t274 * t314);
        let t459 = (t442 + t457) * t64;
        let t460 = t459 * t90;
        let t461 = t280 * t149;
        let t462 = 2.0 * t461;
        let t463 = t162 * t259;
        let tv3rho31 = -t309 + t230 + 4.0 * t231 + t312 + t427 + t306 + t3 * (-t355 + t460 + t462 + t463 + t387 + 2.0 * t389 + t423);
        v3rho3[ip * 4 + 1] += tv3rho31;
        let t468 = t357 * t286;
        let t473 = t207 * t290;
        let t478 = -2.0 * t170 - 6.0 * t364;
        let t482 = piecewise3(t49, 0.0, -8.0 / 27.0 * t468 * t120 + 16.0 / 9.0 * t263 * t211 + 4.0 / 9.0 * t473 * t120 + 4.0 / 3.0 * t52 * t478);
        let t483 = t372 * t295;
        let t488 = t219 * t298;
        let t491 = -t478;
        let t495 = piecewise3(t56, 0.0, -8.0 / 27.0 * t483 * t124 - 16.0 / 9.0 * t271 * t211 + 4.0 / 9.0 * t488 * t124 + 4.0 / 3.0 * t57 * t491);
        let t497 = (t482 + t495) * t64;
        let t498 = t497 * t90;
        let t499 = t304 * t149;
        let tv3rho32 = -t309 + t427 + 4.0 * t282 + t232 + t312 + t305 + t3 * (-t355 + t498 + t499 + t462 + 2.0 * t463 + t389 + t423);
        v3rho3[ip * 4 + 2] += tv3rho32;
        let t505 = t286 * t153;
        let t511 = -6.0 * t170 - 6.0 * t364;
        let t515 = piecewise3(t49, 0.0, -8.0 / 27.0 * t357 * t505 + 4.0 / 3.0 * t263 * t290 + 4.0 / 3.0 * t52 * t511);
        let t516 = t295 * t157;
        let t521 = -t511;
        let t525 = piecewise3(t56, 0.0, -8.0 / 27.0 * t372 * t516 + 4.0 / 3.0 * t271 * t298 + 4.0 / 3.0 * t57 * t521);
        let t527 = (t515 + t525) * t64;
        let t528 = t527 * t90;
        let t530 = 3.0 * t463;
        let tv3rho33 = -t309 + 3.0 * t305 + 6.0 * t282 + t312 + t3 * (-t355 + t528 + 3.0 * t499 + t530 + t423);
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}

/// LDA_C_HL lxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_hl_lxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_hl_c_0: f64,
    param_hl_c_1: f64,
    param_hl_r_0: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = param_hl_c_0;
        let t2 = 1.0 / M_PI;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = param_hl_r_0;
        let t7 = t6 * t6;
        let t8 = t7 * t6;
        let t9 = 1.0 / t8;
        let t12 = 1.0 + 3.0 / 4.0 * t5 * t9;
        let t13 = M_CBRT3;
        let t14 = t13 * t13;
        let t15 = pow_1_3(t2);
        let t16 = 1.0 / t15;
        let t17 = t14 * t16;
        let t18 = M_CBRT4;
        let t19 = pow_1_3(t3);
        let t20 = t18 * t19;
        let t24 = 1.0 + t17 * t20 * t6 / 3.0;
        let t25 = f64::ln(t24);
        let t27 = t15 * t15;
        let t28 = t14 * t27;
        let t29 = t19 * t19;
        let t31 = t18 / t29;
        let t32 = 1.0 / t7;
        let t36 = t13 * t15;
        let t37 = t18 * t18;
        let t39 = t37 / t19;
        let t40 = 1.0 / t6;
        let t45 = t1 * (t12 * t25 - t28 * t31 * t32 / 4.0 + t36 * t39 * t40 / 8.0 - 1.0 / 3.0);
        let t46 = rho0 - rho1;
        let t47 = t46 * t4;
        let t48 = 1.0 + t47;
        let t49 = t48 <= zeta_threshold;
        let t50 = pow_1_3(zeta_threshold);
        let t51 = t50 * zeta_threshold;
        let t52 = pow_1_3(t48);
        let t54 = piecewise3(t49, t51, t52 * t48);
        let t55 = 1.0 - t47;
        let t56 = t55 <= zeta_threshold;
        let t57 = pow_1_3(t55);
        let t59 = piecewise3(t56, t51, t57 * t55);
        let t61 = M_CBRT2;
        let t64 = 1.0 / (2.0 * t61 - 2.0);
        let t65 = (t54 + t59 - 2.0) * t64;
        let t66 = param_hl_c_1;
        let t67 = param_hl_r_1;
        let t68 = t67 * t67;
        let t69 = t68 * t67;
        let t70 = 1.0 / t69;
        let t73 = 1.0 + 3.0 / 4.0 * t5 * t70;
        let t77 = 1.0 + t17 * t20 * t67 / 3.0;
        let t78 = f64::ln(t77);
        let t80 = 1.0 / t68;
        let t84 = 1.0 / t67;
        let t90 = -t66 * (t73 * t78 - t28 * t31 * t80 / 4.0 + t36 * t39 * t84 / 8.0 - 1.0 / 3.0) + t45;
        let t91 = t65 * t90;
        let tzk0 = -t45 + t91;
        zk[ip] += tzk0;
        let t92 = t3 * t3;
        let t93 = 1.0 / t92;
        let t94 = t2 * t93;
        let t95 = t9 * t25;
        let t99 = t12 * t14 * t16;
        let t100 = 1.0 / t24;
        let t101 = t6 * t100;
        let t107 = t18 / t29 / t3;
        let t113 = t37 / t19 / t3;
        let t118 = t1 * (-3.0 / 4.0 * t94 * t95 + t99 * t31 * t101 / 9.0 + t28 * t107 * t32 / 6.0 - t36 * t113 * t40 / 24.0);
        let t119 = t46 * t93;
        let t120 = t4 - t119;
        let t123 = piecewise3(t49, 0.0, 4.0 / 3.0 * t52 * t120);
        let t124 = -t120;
        let t127 = piecewise3(t56, 0.0, 4.0 / 3.0 * t57 * t124);
        let t129 = (t123 + t127) * t64;
        let t130 = t129 * t90;
        let t131 = t70 * t78;
        let t135 = t73 * t14 * t16;
        let t136 = 1.0 / t77;
        let t137 = t67 * t136;
        let t149 = -t66 * (-3.0 / 4.0 * t94 * t131 + t135 * t31 * t137 / 9.0 + t28 * t107 * t80 / 6.0 - t36 * t113 * t84 / 24.0) + t118;
        let t150 = t65 * t149;
        let tvrho0 = -t45 + t91 + t3 * (-t118 + t130 + t150);
        vrho[ip * 2] += tvrho0;
        let t153 = -t4 - t119;
        let t156 = piecewise3(t49, 0.0, 4.0 / 3.0 * t52 * t153);
        let t157 = -t153;
        let t160 = piecewise3(t56, 0.0, 4.0 / 3.0 * t57 * t157);
        let t162 = (t156 + t160) * t64;
        let t163 = t162 * t90;
        let tvrho1 = -t45 + t91 + t3 * (-t118 + t163 + t150);
        vrho[ip * 2 + 1] += tvrho1;
        let t166 = 2.0 * t118;
        let t168 = 2.0 * t150;
        let t169 = t92 * t3;
        let t170 = 1.0 / t169;
        let t171 = t2 * t170;
        let t175 = 1.0 / t29 / t92;
        let t176 = t2 * t175;
        let t179 = t17 * t18 * t100;
        let t186 = 1.0 / t27;
        let t187 = t12 * t13 * t186;
        let t188 = t24 * t24;
        let t189 = 1.0 / t188;
        let t190 = t7 * t189;
        let t194 = t18 * t175;
        let t200 = t37 / t19 / t92;
        let t205 = t1 * (3.0 / 2.0 * t171 * t95 - t176 * t32 * t179 / 6.0 - 2.0 / 27.0 * t99 * t107 * t101 - t187 * t113 * t190 / 27.0 - 5.0 / 18.0 * t28 * t194 * t32 + t36 * t200 * t40 / 18.0);
        let t206 = t52 * t52;
        let t207 = 1.0 / t206;
        let t208 = t120 * t120;
        let t211 = t46 * t170;
        let t213 = -2.0 * t93 + 2.0 * t211;
        let t217 = piecewise3(t49, 0.0, 4.0 / 9.0 * t207 * t208 + 4.0 / 3.0 * t52 * t213);
        let t218 = t57 * t57;
        let t219 = 1.0 / t218;
        let t220 = t124 * t124;
        let t223 = -t213;
        let t227 = piecewise3(t56, 0.0, 4.0 / 9.0 * t219 * t220 + 4.0 / 3.0 * t57 * t223);
        let t229 = (t217 + t227) * t64;
        let t230 = t229 * t90;
        let t231 = t129 * t149;
        let t232 = 2.0 * t231;
        let t237 = t17 * t18 * t136;
        let t244 = t73 * t13 * t186;
        let t245 = t77 * t77;
        let t246 = 1.0 / t245;
        let t247 = t68 * t246;
        let t259 = -t66 * (3.0 / 2.0 * t171 * t131 - t176 * t80 * t237 / 6.0 - 2.0 / 27.0 * t135 * t107 * t137 - t244 * t113 * t247 / 27.0 - 5.0 / 18.0 * t28 * t194 * t80 + t36 * t200 * t84 / 18.0) + t205;
        let t260 = t65 * t259;
        let tv2rho20 = -t166 + 2.0 * t130 + t168 + t3 * (-t205 + t230 + t232 + t260);
        v2rho2[ip * 3] += tv2rho20;
        let t263 = t207 * t153;
        let t266 = t52 * t46;
        let t270 = piecewise3(t49, 0.0, 4.0 / 9.0 * t263 * t120 + 8.0 / 3.0 * t266 * t170);
        let t271 = t219 * t157;
        let t274 = t57 * t46;
        let t278 = piecewise3(t56, 0.0, 4.0 / 9.0 * t271 * t124 - 8.0 / 3.0 * t274 * t170);
        let t280 = (t270 + t278) * t64;
        let t281 = t280 * t90;
        let t282 = t162 * t149;
        let tv2rho21 = -t166 + t130 + t168 + t163 + t3 * (-t205 + t281 + t282 + t231 + t260);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t286 = t153 * t153;
        let t290 = 2.0 * t93 + 2.0 * t211;
        let t294 = piecewise3(t49, 0.0, 4.0 / 9.0 * t207 * t286 + 4.0 / 3.0 * t52 * t290);
        let t295 = t157 * t157;
        let t298 = -t290;
        let t302 = piecewise3(t56, 0.0, 4.0 / 9.0 * t219 * t295 + 4.0 / 3.0 * t57 * t298);
        let t304 = (t294 + t302) * t64;
        let t305 = t304 * t90;
        let t306 = 2.0 * t282;
        let tv2rho22 = -t166 + 2.0 * t163 + t168 + t3 * (-t205 + t305 + t306 + t260);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t309 = 3.0 * t205;
        let t312 = 3.0 * t260;
        let t313 = t92 * t92;
        let t314 = 1.0 / t313;
        let t315 = t2 * t314;
        let t319 = 1.0 / t29 / t169;
        let t320 = t2 * t319;
        let t325 = 1.0 / t19 / t169;
        let t326 = t2 * t325;
        let t328 = t13 * t186;
        let t330 = t328 * t37 * t189;
        let t339 = t12 * M_PI;
        let t342 = 1.0 / t188 / t24;
        let t346 = t18 * t319;
        let t350 = t37 * t325;
        let t355 = t1 * (-9.0 / 2.0 * t315 * t95 + 2.0 / 3.0 * t320 * t32 * t179 + t326 * t40 * t330 / 12.0 + 10.0 / 81.0 * t99 * t194 * t101 + 2.0 / 27.0 * t187 * t200 * t190 + 8.0 / 81.0 * t339 * t93 * t8 * t342 + 20.0 / 27.0 * t28 * t346 * t32 - 7.0 / 54.0 * t36 * t350 * t40);
        let t357 = 1.0 / t206 / t48;
        let t358 = t208 * t120;
        let t361 = t207 * t120;
        let t364 = t46 * t314;
        let t366 = 6.0 * t170 - 6.0 * t364;
        let t370 = piecewise3(t49, 0.0, -8.0 / 27.0 * t357 * t358 + 4.0 / 3.0 * t361 * t213 + 4.0 / 3.0 * t52 * t366);
        let t372 = 1.0 / t218 / t55;
        let t373 = t220 * t124;
        let t376 = t219 * t124;
        let t379 = -t366;
        let t383 = piecewise3(t56, 0.0, -8.0 / 27.0 * t372 * t373 + 4.0 / 3.0 * t376 * t223 + 4.0 / 3.0 * t57 * t379);
        let t385 = (t370 + t383) * t64;
        let t386 = t385 * t90;
        let t387 = t229 * t149;
        let t389 = t129 * t259;
        let t390 = 3.0 * t389;
        let t398 = t328 * t37 * t246;
        let t407 = t73 * M_PI;
        let t410 = 1.0 / t245 / t77;
        let t422 = -t66 * (-9.0 / 2.0 * t315 * t131 + 2.0 / 3.0 * t320 * t80 * t237 + t326 * t84 * t398 / 12.0 + 10.0 / 81.0 * t135 * t194 * t137 + 2.0 / 27.0 * t244 * t200 * t247 + 8.0 / 81.0 * t407 * t93 * t69 * t410 + 20.0 / 27.0 * t28 * t346 * t80 - 7.0 / 54.0 * t36 * t350 * t84) + t355;
        let t423 = t65 * t422;
        let tv3rho30 = -t309 + 3.0 * t230 + 6.0 * t231 + t312 + t3 * (-t355 + t386 + 3.0 * t387 + t390 + t423);
        v3rho3[ip * 4] += tv3rho30;
        let t427 = 2.0 * t281;
        let t428 = t357 * t153;
        let t431 = t207 * t46;
        let t442 = piecewise3(t49, 0.0, -8.0 / 27.0 * t428 * t208 + 16.0 / 9.0 * t431 * t170 * t120 + 4.0 / 9.0 * t263 * t213 + 8.0 / 3.0 * t52 * t170 - 8.0 * t266 * t314);
        let t443 = t372 * t157;
        let t446 = t219 * t46;
        let t457 = piecewise3(t56, 0.0, -8.0 / 27.0 * t443 * t220 - 16.0 / 9.0 * t446 * t170 * t124 + 4.0 / 9.0 * t271 * t223 - 8.0 / 3.0 * t57 * t170 + 8.0 * t274 * t314);
        let t459 = (t442 + t457) * t64;
        let t460 = t459 * t90;
        let t461 = t280 * t149;
        let t462 = 2.0 * t461;
        let t463 = t162 * t259;
        let tv3rho31 = -t309 + t230 + 4.0 * t231 + t312 + t427 + t306 + t3 * (-t355 + t460 + t462 + t463 + t387 + 2.0 * t389 + t423);
        v3rho3[ip * 4 + 1] += tv3rho31;
        let t468 = t357 * t286;
        let t473 = t207 * t290;
        let t478 = -2.0 * t170 - 6.0 * t364;
        let t482 = piecewise3(t49, 0.0, -8.0 / 27.0 * t468 * t120 + 16.0 / 9.0 * t263 * t211 + 4.0 / 9.0 * t473 * t120 + 4.0 / 3.0 * t52 * t478);
        let t483 = t372 * t295;
        let t488 = t219 * t298;
        let t491 = -t478;
        let t495 = piecewise3(t56, 0.0, -8.0 / 27.0 * t483 * t124 - 16.0 / 9.0 * t271 * t211 + 4.0 / 9.0 * t488 * t124 + 4.0 / 3.0 * t57 * t491);
        let t497 = (t482 + t495) * t64;
        let t498 = t497 * t90;
        let t499 = t304 * t149;
        let tv3rho32 = -t309 + t427 + 4.0 * t282 + t232 + t312 + t305 + t3 * (-t355 + t498 + t499 + t462 + 2.0 * t463 + t389 + t423);
        v3rho3[ip * 4 + 2] += tv3rho32;
        let t505 = t286 * t153;
        let t511 = -6.0 * t170 - 6.0 * t364;
        let t515 = piecewise3(t49, 0.0, -8.0 / 27.0 * t357 * t505 + 4.0 / 3.0 * t263 * t290 + 4.0 / 3.0 * t52 * t511);
        let t516 = t295 * t157;
        let t521 = -t511;
        let t525 = piecewise3(t56, 0.0, -8.0 / 27.0 * t372 * t516 + 4.0 / 3.0 * t271 * t298 + 4.0 / 3.0 * t57 * t521);
        let t527 = (t515 + t525) * t64;
        let t528 = t527 * t90;
        let t530 = 3.0 * t463;
        let tv3rho33 = -t309 + 3.0 * t305 + 6.0 * t282 + t312 + t3 * (-t355 + t528 + 3.0 * t499 + t530 + t423);
        v3rho3[ip * 4 + 3] += tv3rho33;
        let t533 = 4.0 * t355;
        let t537 = 4.0 * t423;
        let t539 = 1.0 / t313 / t3;
        let t540 = t2 * t539;
        let t544 = 1.0 / t29 / t313;
        let t545 = t2 * t544;
        let t550 = 1.0 / t19 / t313;
        let t551 = t2 * t550;
        let t567 = t7 * t7;
        let t570 = t188 * t188;
        let t573 = t16 * t18;
        let t577 = t18 * t544;
        let t581 = t37 * t550;
        let t586 = t1 * (18.0 * t540 * t95 - 82.0 / 27.0 * t545 * t32 * t179 - 5.0 / 9.0 * t551 * t40 * t330 - 8.0 / 27.0 * t314 * t342 - 80.0 / 243.0 * t99 * t346 * t101 - 52.0 / 243.0 * t187 * t350 * t190 - 32.0 / 81.0 * t339 * t170 * t8 * t342 - 8.0 / 243.0 * t339 * t175 * t567 / t570 * t14 * t573 - 220.0 / 81.0 * t28 * t577 * t32 + 35.0 / 81.0 * t36 * t581 * t40);
        let t587 = t48 * t48;
        let t589 = 1.0 / t206 / t587;
        let t590 = t208 * t208;
        let t596 = t213 * t213;
        let t601 = t46 * t539;
        let t603 = -24.0 * t314 + 24.0 * t601;
        let t607 = piecewise3(t49, 0.0, 40.0 / 81.0 * t589 * t590 - 16.0 / 9.0 * t357 * t208 * t213 + 4.0 / 3.0 * t207 * t596 + 16.0 / 9.0 * t361 * t366 + 4.0 / 3.0 * t52 * t603);
        let t608 = t55 * t55;
        let t610 = 1.0 / t218 / t608;
        let t611 = t220 * t220;
        let t617 = t223 * t223;
        let t626 = piecewise3(t56, 0.0, 40.0 / 81.0 * t610 * t611 - 16.0 / 9.0 * t372 * t220 * t223 + 4.0 / 3.0 * t219 * t617 + 16.0 / 9.0 * t376 * t379 - 4.0 / 3.0 * t57 * t603);
        let t630 = t385 * t149;
        let t632 = t229 * t259;
        let t634 = t129 * t422;
        let t656 = t68 * t68;
        let t659 = t245 * t245;
        let t674 = t65 * (-t66 * (18.0 * t540 * t131 - 82.0 / 27.0 * t545 * t80 * t237 - 5.0 / 9.0 * t551 * t84 * t398 - 8.0 / 27.0 * t314 * t410 - 80.0 / 243.0 * t135 * t346 * t137 - 52.0 / 243.0 * t244 * t350 * t247 - 32.0 / 81.0 * t407 * t170 * t69 * t410 - 8.0 / 243.0 * t407 * t175 * t656 / t659 * t14 * t573 - 220.0 / 81.0 * t28 * t577 * t80 + 35.0 / 81.0 * t36 * t581 * t84) + t586);
        let tv4rho40 = -t533 + 4.0 * t386 + 12.0 * t387 + 12.0 * t389 + t537 + t3 * (-t586 + (t607 + t626) * t64 * t90 + 4.0 * t630 + 6.0 * t632 + 4.0 * t634 + t674);
        v4rho4[ip * 5] += tv4rho40;
        let t680 = 6.0 * t461;
        let t705 = 32.0 * t266 * t539;
        let t707 = piecewise3(t49, 0.0, 40.0 / 81.0 * t589 * t153 * t358 - 16.0 / 9.0 * t357 * t46 * t170 * t208 - 8.0 / 9.0 * t428 * t120 * t213 + 8.0 / 3.0 * t207 * t170 * t120 - 8.0 * t431 * t314 * t120 + 8.0 / 3.0 * t431 * t170 * t213 + 4.0 / 9.0 * t263 * t366 - 16.0 * t52 * t314 + t705);
        let t732 = 32.0 * t274 * t539;
        let t734 = piecewise3(t56, 0.0, 40.0 / 81.0 * t610 * t157 * t373 + 16.0 / 9.0 * t372 * t46 * t170 * t220 - 8.0 / 9.0 * t443 * t124 * t223 - 8.0 / 3.0 * t219 * t170 * t124 + 8.0 * t446 * t314 * t124 - 8.0 / 3.0 * t446 * t170 * t223 + 4.0 / 9.0 * t271 * t379 + 16.0 * t57 * t314 - t732);
        let t738 = t459 * t149;
        let t740 = t280 * t259;
        let t741 = 3.0 * t740;
        let t742 = t162 * t422;
        let tv4rho41 = -t533 + t386 + 6.0 * t387 + 9.0 * t389 + t537 + 3.0 * t460 + t680 + t530 + t3 * (-t586 + (t707 + t734) * t64 * t90 + 3.0 * t738 + t741 + t742 + t630 + 3.0 * t632 + 3.0 * t634 + t674);
        v4rho4[ip * 5 + 1] += tv4rho41;
        let t763 = t46 * t46;
        let t766 = 1.0 / t313 / t92;
        let t782 = piecewise3(t49, 0.0, 40.0 / 81.0 * t589 * t286 * t208 - 64.0 / 27.0 * t428 * t120 * t46 * t170 - 8.0 / 27.0 * t468 * t213 + 32.0 / 9.0 * t207 * t763 * t766 + 16.0 / 9.0 * t263 * t170 - 16.0 / 3.0 * t263 * t364 - 8.0 / 27.0 * t357 * t290 * t208 + 8.0 / 9.0 * t207 * t478 * t120 + 4.0 / 9.0 * t473 * t213 + t705);
        let t808 = piecewise3(t56, 0.0, 40.0 / 81.0 * t610 * t295 * t220 + 64.0 / 27.0 * t443 * t124 * t46 * t170 - 8.0 / 27.0 * t483 * t223 + 32.0 / 9.0 * t219 * t763 * t766 - 16.0 / 9.0 * t271 * t170 + 16.0 / 3.0 * t271 * t364 - 8.0 / 27.0 * t372 * t298 * t220 + 8.0 / 9.0 * t219 * t491 * t124 + 4.0 / 9.0 * t488 * t223 - t732);
        let t812 = t497 * t149;
        let t814 = t304 * t259;
        let tv4rho42 = -t533 + 2.0 * t460 + 8.0 * t461 + 6.0 * t463 + 2.0 * t387 + 6.0 * t389 + t537 + 2.0 * t498 + 2.0 * t499 + t3 * (-t586 + (t782 + t808) * t64 * t90 + 2.0 * t812 + t814 + 2.0 * t738 + 4.0 * t740 + 2.0 * t742 + t632 + 2.0 * t634 + t674);
        v4rho4[ip * 5 + 2] += tv4rho42;
        let t842 = 12.0 * t314 + 24.0 * t601;
        let t846 = piecewise3(t49, 0.0, 40.0 / 81.0 * t589 * t505 * t120 - 16.0 / 9.0 * t468 * t211 - 8.0 / 9.0 * t428 * t290 * t120 + 8.0 / 3.0 * t431 * t170 * t290 + 4.0 / 3.0 * t263 * t478 + 4.0 / 9.0 * t207 * t511 * t120 + 4.0 / 3.0 * t52 * t842);
        let t867 = piecewise3(t56, 0.0, 40.0 / 81.0 * t610 * t516 * t124 + 16.0 / 9.0 * t483 * t211 - 8.0 / 9.0 * t443 * t298 * t124 - 8.0 / 3.0 * t446 * t170 * t298 + 4.0 / 3.0 * t271 * t491 + 4.0 / 9.0 * t219 * t521 * t124 - 4.0 / 3.0 * t57 * t842);
        let t871 = t527 * t149;
        let tv4rho43 = -t533 + 3.0 * t498 + 6.0 * t499 + t680 + 9.0 * t463 + t390 + t537 + t528 + t3 * (-t586 + (t846 + t867) * t64 * t90 + t871 + 3.0 * t812 + 3.0 * t814 + t741 + 3.0 * t742 + t634 + t674);
        v4rho4[ip * 5 + 3] += tv4rho43;
        let t880 = t286 * t286;
        let t885 = t290 * t290;
        let t891 = 24.0 * t314 + 24.0 * t601;
        let t895 = piecewise3(t49, 0.0, 40.0 / 81.0 * t589 * t880 - 16.0 / 9.0 * t468 * t290 + 4.0 / 3.0 * t207 * t885 + 16.0 / 9.0 * t263 * t511 + 4.0 / 3.0 * t52 * t891);
        let t896 = t295 * t295;
        let t901 = t298 * t298;
        let t910 = piecewise3(t56, 0.0, 40.0 / 81.0 * t610 * t896 - 16.0 / 9.0 * t483 * t298 + 4.0 / 3.0 * t219 * t901 + 16.0 / 9.0 * t271 * t521 - 4.0 / 3.0 * t57 * t891);
        let tv4rho44 = -t533 + 4.0 * t528 + 12.0 * t499 + 12.0 * t463 + t537 + t3 * (-t586 + (t895 + t910) * t64 * t90 + 4.0 * t871 + 6.0 * t814 + 4.0 * t742 + t674);
        v4rho4[ip * 5 + 4] += tv4rho44;
    }
}
