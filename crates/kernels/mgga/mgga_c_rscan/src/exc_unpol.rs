//! MGGA_C_RSCAN exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rscan.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_rscan_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = M_CBRT3;
        let t3 = 1.0 / M_PI;
        let t4 = pow_1_3::<f64>(t3);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t8 = pow_1_3::<f64>(rho[ip]);
        let t11 = t5 * t7 / t8;
        let t13 = 1.0 + 0.53425e-1 * t11;
        let t14 = f64::sqrt(t11);
        let t17 = pow_3_2::<f64>(t11);
        let t19 = t2 * t2;
        let t20 = t4 * t4;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t23 = 1.0 / t22;
        let t25 = t21 * t6 * t23;
        let t27 = 0.379785e1 * t14 + 0.8969e0 * t11 + 0.204775e0 * t17 + 0.123235e0 * t25;
        let t30 = 1.0 + 0.16081979498692535067e2 / t27;
        let t31 = f64::ln(t30);
        let t33 = 0.621814e-1 * t13 * t31;
        let t34 = 1.0 <= zeta_threshold;
        let t35 = pow_1_3::<f64>(zeta_threshold);
        let t37 = piecewise3::<f64>(t34, t35 * zeta_threshold, 1.0);
        let t39 = 2.0 * t37 - 2.0;
        let t40 = M_CBRT2;
        let t41 = t40 - 1.0;
        let t43 = 1.0 / t41 / 2.0;
        let t44 = t39 * t43;
        let t46 = 1.0 + 0.278125e-1 * t11;
        let t51 = 0.51785e1 * t14 + 0.905775e0 * t11 + 0.1100325e0 * t17 + 0.1241775e0 * t25;
        let t54 = 1.0 + 0.29608749977793437516e2 / t51;
        let t55 = f64::ln(t54);
        let t58 = 0.19751673498613801407e-1 * t44 * t46 * t55;
        let t59 = f64::ln(2.0);
        let t60 = 1.0 - t59;
        let t61 = M_PI * M_PI;
        let t63 = t60 / t61;
        let t64 = t35 * t35;
        let t65 = piecewise3::<f64>(t34, t64, 1.0);
        let t66 = t65 * t65;
        let t67 = t66 * t65;
        let t69 = 1.0 + 0.25e-1 * t11;
        let t71 = 1.0 + 0.4445e-1 * t11;
        let t72 = 1.0 / t71;
        let t73 = t69 * t72;
        let t74 = 1.0 / t60;
        let t77 = 1.0 / t67;
        let t78 = t61 * t77;
        let t80 = f64::exp(-(-t33 + t58) * t74 * t78);
        let t81 = t80 - 1.0;
        let t82 = 1.0 / t81;
        let t83 = t74 * t82;
        let t84 = t83 * sigma[ip];
        let t85 = t73 * t84;
        let t86 = rho[ip] * rho[ip];
        let t87 = t8 * t86;
        let t88 = 1.0 / t87;
        let t89 = t88 * t40;
        let t90 = 1.0 / t66;
        let t92 = 1.0 / t4;
        let t94 = t19 * t92 * t6;
        let t98 = 1.0 + 0.27439371595564631661e-1 * t85 * t89 * t90 * t94;
        let t99 = pow_1_4::<f64>(t98);
        let t101 = 1.0 - 1.0 / t99;
        let t104 = 1.0 + 1.0 * t101 * t81;
        let t105 = f64::ln(t104);
        let t107 = t63 * t67 * t105;
        let t108 = t86 * t86;
        let t109 = t108 * rho[ip];
        let t110 = t22 * rho[ip];
        let t111 = 1.0 / t110;
        let t113 = t22 * t86;
        let t114 = 1.0 / t113;
        let t117 = tau[ip] * t111 - sigma[ip] * t114 / 8.0;
        let t118 = 0.0 < t117;
        let t119 = piecewise3::<f64>(t118, t117, 0.0);
        let t120 = t119 * t119;
        let t121 = t120 * t119;
        let t122 = t109 * t121;
        let t123 = M_CBRT6;
        let t124 = t123 * t123;
        let t125 = pow_1_3::<f64>(t61);
        let t126 = t125 * t125;
        let t127 = t124 * t126;
        let t130 = t40 * t40;
        let t132 = 3.0 / 10.0 * t127 * t110 + 0.1e-3 * t130;
        let t133 = t132 * t132;
        let t134 = t133 * t132;
        let t135 = 1.0 / t134;
        let t136 = t86 * rho[ip];
        let t137 = t8 * t136;
        let t140 = 1.0 / t133 * t40;
        let t143 = 2.0 * t137 * t120 * t140 + 0.1e-2;
        let t144 = 1.0 / t143;
        let t145 = t135 * t144;
        let t147 = 4.0 * t122 * t145;
        let t148 = t147 <= 0.25e1;
        let t149 = 0.25e1 < t147;
        let t150 = piecewise3::<f64>(t149, 0.25e1, t147);
        let t152 = t150 * t150;
        let t154 = t152 * t150;
        let t156 = t152 * t152;
        let t158 = t156 * t150;
        let t160 = t156 * t152;
        let t165 = piecewise3::<f64>(t149, t147, 0.25e1);
        let t166 = 1.0 - t165;
        let t169 = f64::exp(0.15e1 / t166);
        let t171 = piecewise3::<f64>(t148, 1.0 - 0.64e0 * t150 - 0.4352e0 * t152 - 0.1535685604549e1 * t154 + 0.3061560252175e1 * t156 - 0.1915710236206e1 * t158 + 0.516884468372e0 * t160 - 0.51848879792e-1 * t156 * t154, -0.7e0 * t169);
        let t174 = 1.0 + 0.4445e-1 * t14 + 0.3138525e-1 * t11;
        let t175 = 1.0 / t174;
        let t178 = f64::exp(1.0 * t175);
        let t179 = t178 - 1.0;
        let t180 = 1.0 / t126;
        let t181 = t123 * t180;
        let t182 = t130 * sigma[ip];
        let t186 = 1.0 + 0.21337642104376358333e-1 * t181 * t182 * t114;
        let t187 = pow_1_4::<f64>(t186);
        let t189 = 1.0 - 1.0 / t187;
        let t191 = t179 * t189 + 1.0;
        let t192 = f64::ln(t191);
        let t198 = 1.0 - 0.2363e1 * t41 * t39 * t43;
        let t200 = (-0.285764e-1 * t175 + 0.285764e-1 * t192) * t198 + t33 - t58 - t107;
        let t201 = t171 * t200;
        let tzk0 = -t33 + t58 + t107 + t201;
        zk[ip] += tzk0;
    }
}
