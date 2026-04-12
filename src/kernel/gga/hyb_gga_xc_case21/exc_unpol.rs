//! HYB_GGA_XC_CASE21 exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/hyb_gga_xc_case21.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::bspline::{case21_cbspline, case21_xbspline};
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn hyb_gga_xc_case21_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_ax: f64,
    param_gammac: f64,
    param_gammax: f64,
    param_cx_0: f64,
    param_cx_1: f64,
    param_cx_2: f64,
    param_cx_3: f64,
    param_cx_4: f64,
    param_cx_5: f64,
    param_cx_6: f64,
    param_cx_7: f64,
    param_cx_8: f64,
    param_cx_9: f64,
    param_cc_0: f64,
    param_cc_1: f64,
    param_cc_2: f64,
    param_cc_3: f64,
    param_cc_4: f64,
    param_cc_5: f64,
    param_cc_6: f64,
    param_cc_7: f64,
    param_cc_8: f64,
    param_cc_9: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = 1.0 - param_ax;
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * zeta_threshold;
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t15, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT6;
        let t23 = M_PI * M_PI;
        let t24 = pow_1_3(t23);
        let t25 = t24 * t24;
        let t26 = 1.0 / t25;
        let t27 = param_gammax * t21 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = sigma[ip] * t29;
        let t31 = rho[ip] * rho[ip];
        let t32 = t19 * t19;
        let t34 = 1.0 / t32 / t31;
        let t38 = 1.0 + t27 * t30 * t34 / 24.0;
        let t39 = 1.0 / t38;
        let t43 = t27 * t30 * t34 * t39 / 24.0;
        let t44 = case21_xbspline(t43, 0, param_cx_0, param_cx_1, param_cx_2, param_cx_3, param_cx_4, param_cx_5, param_cx_6, param_cx_7, param_cx_8, param_cx_9);
        let t48 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t44);
        let t50 = 2.0 * t1 * t48;
        let t51 = t14 * t14;
        let t52 = piecewise3(t8, t51, 1.0);
        let t53 = t4 * t4;
        let t54 = t52 * t53;
        let t55 = t54 * t5;
        let t57 = 1.0 / t19 / t31;
        let t58 = sigma[ip] * t57;
        let t59 = t5 * sigma[ip];
        let t63 = 1.0 / M_PI;
        let t64 = pow_1_3(t63);
        let t65 = t4 * t64;
        let t66 = M_CBRT4;
        let t67 = t66 * t66;
        let t70 = t65 * t67 / t19;
        let t72 = 1.0 + 0.53425e-1 * t70;
        let t73 = f64::sqrt(t70);
        let t76 = pow_3_2(t70);
        let t78 = t64 * t64;
        let t79 = t53 * t78;
        let t80 = 1.0 / t32;
        let t82 = t79 * t66 * t80;
        let t84 = 0.379785e1 * t73 + 0.8969e0 * t70 + 0.204775e0 * t76 + 0.123235e0 * t82;
        let t87 = 1.0 + 0.16081979498692535067e2 / t84;
        let t88 = f64::ln(t87);
        let t91 = piecewise3(t8, t15, 1.0);
        let t97 = (2.0 * t91 - 2.0) / (2.0 * t28 - 2.0);
        let t99 = 1.0 + 0.278125e-1 * t70;
        let t104 = 0.51785e1 * t73 + 0.905775e0 * t70 + 0.1100325e0 * t76 + 0.1241775e0 * t82;
        let t107 = 1.0 + 0.29608749977793437516e2 / t104;
        let t108 = f64::ln(t107);
        let t112 = -0.621814e-1 * t72 * t88 + 0.19751673498613801407e-1 * t97 * t99 * t108;
        let t114 = -t54 * t59 * t57 / 48.0 + param_gammac * t112;
        let t115 = 1.0 / t114;
        let t118 = t55 * t58 * t115 / 48.0;
        let t119 = case21_cbspline(-t118, 0, param_cc_0, param_cc_1, param_cc_2, param_cc_3, param_cc_4, param_cc_5, param_cc_6, param_cc_7, param_cc_8, param_cc_9);
        let t120 = t119 * t112;
        let tzk0 = t50 + t120;
        zk[ip] += tzk0;
    }
}
