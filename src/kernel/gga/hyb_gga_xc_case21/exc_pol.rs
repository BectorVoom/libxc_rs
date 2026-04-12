//! HYB_GGA_XC_CASE21 exc pol kernel.
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
pub fn hyb_gga_xc_case21_exc_pol(
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
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = 1.0 - param_ax;
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t18 = t17 * t8;
        let t19 = piecewise5(t11, t12, t15, t16, t18);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = M_CBRT6;
        let t30 = param_gammax * t29;
        let t31 = M_PI * M_PI;
        let t32 = pow_1_3(t31);
        let t33 = t32 * t32;
        let t34 = 1.0 / t33;
        let t35 = t30 * t34;
        let t36 = rho0 * rho0;
        let t37 = pow_1_3(rho0);
        let t38 = t37 * t37;
        let t40 = 1.0 / t38 / t36;
        let t42 = t34 * sigma0;
        let t46 = 1.0 + t30 * t42 * t40 / 24.0;
        let t47 = 1.0 / t46;
        let t50 = t35 * sigma0 * t40 * t47 / 24.0;
        let t51 = case21_xbspline(t50, 0, param_cx_0, param_cx_1, param_cx_2, param_cx_3, param_cx_4, param_cx_5, param_cx_6, param_cx_7, param_cx_8, param_cx_9);
        let t55 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t51);
        let t56 = rho1 <= dens_threshold;
        let t57 = -t17;
        let t59 = piecewise5(t15, t12, t11, t16, t57 * t8);
        let t60 = 1.0 + t59;
        let t61 = t60 <= zeta_threshold;
        let t62 = pow_1_3(t60);
        let t64 = piecewise3(t61, t23, t62 * t60);
        let t65 = t64 * t27;
        let t66 = rho1 * rho1;
        let t67 = pow_1_3(rho1);
        let t68 = t67 * t67;
        let t70 = 1.0 / t68 / t66;
        let t72 = t34 * sigma2;
        let t76 = 1.0 + t30 * t72 * t70 / 24.0;
        let t77 = 1.0 / t76;
        let t80 = t35 * sigma2 * t70 * t77 / 24.0;
        let t81 = case21_xbspline(t80, 0, param_cx_0, param_cx_1, param_cx_2, param_cx_3, param_cx_4, param_cx_5, param_cx_6, param_cx_7, param_cx_8, param_cx_9);
        let t85 = piecewise3(t56, 0.0, -3.0 / 8.0 * t6 * t65 * t81);
        let t87 = t1 * (t55 + t85);
        let t88 = t18 + 1.0;
        let t89 = t88 <= zeta_threshold;
        let t90 = t22 * t22;
        let t91 = pow_1_3(t88);
        let t92 = t91 * t91;
        let t93 = piecewise3(t89, t90, t92);
        let t94 = 1.0 - t18;
        let t95 = t94 <= zeta_threshold;
        let t96 = pow_1_3(t94);
        let t97 = t96 * t96;
        let t98 = piecewise3(t95, t90, t97);
        let t100 = t93 / 2.0 + t98 / 2.0;
        let t101 = t3 * t3;
        let t102 = t100 * t101;
        let t103 = t102 * t4;
        let t104 = f64::sqrt(sigma0);
        let t105 = f64::sqrt(sigma2);
        let t106 = t104 + t105;
        let t107 = t106 * t106;
        let t108 = t7 * t7;
        let t110 = 1.0 / t27 / t108;
        let t111 = t107 * t110;
        let t112 = t4 * t107;
        let t113 = t112 * t110;
        let t116 = 1.0 / M_PI;
        let t117 = pow_1_3(t116);
        let t118 = t3 * t117;
        let t119 = M_CBRT4;
        let t120 = t119 * t119;
        let t123 = t118 * t120 / t27;
        let t125 = 1.0 + 0.53425e-1 * t123;
        let t126 = f64::sqrt(t123);
        let t129 = pow_3_2(t123);
        let t131 = t117 * t117;
        let t132 = t101 * t131;
        let t133 = t27 * t27;
        let t134 = 1.0 / t133;
        let t136 = t132 * t119 * t134;
        let t138 = 0.379785e1 * t126 + 0.8969e0 * t123 + 0.204775e0 * t129 + 0.123235e0 * t136;
        let t141 = 1.0 + 0.16081979498692535067e2 / t138;
        let t142 = f64::ln(t141);
        let t144 = 0.621814e-1 * t125 * t142;
        let t145 = t17 * t17;
        let t146 = t145 * t145;
        let t147 = t108 * t108;
        let t148 = 1.0 / t147;
        let t149 = t146 * t148;
        let t150 = t91 * t88;
        let t151 = piecewise3(t89, t23, t150);
        let t152 = t96 * t94;
        let t153 = piecewise3(t95, t23, t152);
        let t154 = t151 + t153 - 2.0;
        let t155 = M_CBRT2;
        let t158 = 1.0 / (2.0 * t155 - 2.0);
        let t159 = t154 * t158;
        let t161 = 1.0 + 0.5137e-1 * t123;
        let t166 = 0.705945e1 * t126 + 0.1549425e1 * t123 + 0.420775e0 * t129 + 0.1562925e0 * t136;
        let t169 = 1.0 + 0.32163958997385070134e2 / t166;
        let t170 = f64::ln(t169);
        let t174 = 1.0 + 0.278125e-1 * t123;
        let t179 = 0.51785e1 * t126 + 0.905775e0 * t123 + 0.1100325e0 * t129 + 0.1241775e0 * t136;
        let t182 = 1.0 + 0.29608749977793437516e2 / t179;
        let t183 = f64::ln(t182);
        let t184 = t174 * t183;
        let t186 = -0.310907e-1 * t161 * t170 + t144 - 0.19751673498613801407e-1 * t184;
        let t187 = t159 * t186;
        let t191 = -t144 + t149 * t187 + 0.19751673498613801407e-1 * t159 * t184;
        let t193 = -t102 * t113 / 48.0 + param_gammac * t191;
        let t194 = 1.0 / t193;
        let t195 = t111 * t194;
        let t197 = t103 * t195 / 48.0;
        let t198 = case21_cbspline(-t197, 0, param_cc_0, param_cc_1, param_cc_2, param_cc_3, param_cc_4, param_cc_5, param_cc_6, param_cc_7, param_cc_8, param_cc_9);
        let t199 = t198 * t191;
        let tzk0 = t87 + t199;
        zk[ip] += tzk0;
    }
}
