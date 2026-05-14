//! MGGA_XC_B98 exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_b98.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_xc_b98_exc_unpol(
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
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = 1.0 <= zeta_threshold;
        let t7 = pow_1_3(zeta_threshold);
        let t8 = t7 * zeta_threshold;
        let t9 = piecewise3(t6, t8, 1.0);
        let t10 = pow_1_3(rho[ip]);
        let t11 = t9 * t10;
        let t12 = M_CBRT2;
        let t13 = t12 * t12;
        let t14 = tau[ip] * t13;
        let t15 = t10 * t10;
        let t17 = 1.0 / t15 / rho[ip];
        let t19 = sigma[ip] * t13;
        let t20 = rho[ip] * rho[ip];
        let t22 = 1.0 / t15 / t20;
        let t25 = lapl[ip] * t13;
        let t29 = M_CBRT6;
        let t31 = M_PI * M_PI;
        let t32 = pow_1_3(t31);
        let t33 = t32 * t32;
        let t34 = 1.0 / t33;
        let t37 = 1.0 - 5.0 / 9.0 * (t14 * t17 - t19 * t22 / 8.0 - t25 * t17 / 4.0) * t29 * t34;
        let t38 = t37 * t37;
        let t40 = 1.0 + 0.121e-1 * t38;
        let t41 = f64::sqrt(t40);
        let t42 = 1.0 / t41;
        let t45 = 1.0 / t40;
        let t48 = 0.8085e0 + 0.73502e-1 * t37 * t42 + 0.17182e-2 * t38 * t45;
        let t51 = 3.0 / 4.0 * t5 * t11 * t48;
        let t54 = rho[ip] / 2.0 <= dens_threshold || t6;
        let t55 = piecewise3(t6, zeta_threshold, 1.0);
        let t56 = 1.0 / M_PI;
        let t57 = pow_1_3(t56);
        let t58 = t2 * t57;
        let t59 = M_CBRT4;
        let t60 = t59 * t59;
        let t61 = t58 * t60;
        let t62 = 1.0 / t10;
        let t65 = piecewise3(t6, 1.0 / t7, 1.0);
        let t67 = t61 * t62 * t12 * t65;
        let t69 = 1.0 + 0.53425e-1 * t67;
        let t70 = f64::sqrt(t67);
        let t73 = pow_3_2(t67);
        let t75 = t2 * t2;
        let t76 = t57 * t57;
        let t77 = t75 * t76;
        let t78 = t77 * t59;
        let t79 = 1.0 / t15;
        let t81 = t65 * t65;
        let t83 = t78 * t79 * t13 * t81;
        let t85 = 0.379785e1 * t70 + 0.8969e0 * t67 + 0.204775e0 * t73 + 0.123235e0 * t83;
        let t88 = 1.0 + 0.16081824322151104822e2 / t85;
        let t89 = f64::ln(t88);
        let t91 = 0.62182e-1 * t69 * t89;
        let t94 = piecewise3(2.0 <= zeta_threshold, t8, 2.0 * t12);
        let t96 = piecewise3(0.0 <= zeta_threshold, t8, 0.0);
        let t100 = 1.0 / (2.0 * t12 - 2.0);
        let t101 = (t94 + t96 - 2.0) * t100;
        let t103 = 1.0 + 0.5137e-1 * t67;
        let t108 = 0.705945e1 * t70 + 0.1549425e1 * t67 + 0.420775e0 * t73 + 0.1562925e0 * t83;
        let t111 = 1.0 + 0.32164683177870697974e2 / t108;
        let t112 = f64::ln(t111);
        let t116 = 1.0 + 0.278125e-1 * t67;
        let t121 = 0.51785e1 * t70 + 0.905775e0 * t67 + 0.1100325e0 * t73 + 0.1241775e0 * t83;
        let t124 = 1.0 + 0.29608574643216675549e2 / t121;
        let t125 = f64::ln(t124);
        let t126 = t116 * t125;
        let t135 = piecewise3(t54, 0.0, t55 * (-t91 + t101 * (-0.3109e-1 * t103 * t112 + t91 - 0.19751789702565206229e-1 * t126) + 0.19751789702565206229e-1 * t101 * t126) / 2.0);
        let t137 = 1.0 + 0.256e1 * t38;
        let t138 = f64::sqrt(t137);
        let t139 = 1.0 / t138;
        let t142 = 1.0 / t137;
        let t145 = 0.2606e0 - 0.153728e1 * t37 * t139 + 0.2309888e1 * t38 * t142;
        let t146 = t135 * t145;
        let t147 = 1.0 / rho[ip];
        let t148 = sigma[ip] * t147;
        let t149 = 1.0 / tau[ip];
        let t152 = 1.0 - t148 * t149 / 8.0;
        let t154 = 2.0 * t146 * t152;
        let t156 = t58 * t60 * t62;
        let t158 = 1.0 + 0.53425e-1 * t156;
        let t159 = f64::sqrt(t156);
        let t162 = pow_3_2(t156);
        let t165 = t77 * t59 * t79;
        let t167 = 0.379785e1 * t159 + 0.8969e0 * t156 + 0.204775e0 * t162 + 0.123235e0 * t165;
        let t170 = 1.0 + 0.16081824322151104822e2 / t167;
        let t171 = f64::ln(t170);
        let t176 = (2.0 * t9 - 2.0) * t100;
        let t178 = 1.0 + 0.278125e-1 * t156;
        let t183 = 0.51785e1 * t159 + 0.905775e0 * t156 + 0.1100325e0 * t162 + 0.1241775e0 * t165;
        let t186 = 1.0 + 0.29608574643216675549e2 / t183;
        let t187 = f64::ln(t186);
        let t192 = -0.62182e-1 * t158 * t171 + 0.19751789702565206229e-1 * t176 * t178 * t187 - 2.0 * t135;
        let t194 = 1.0 + 0.196e-1 * t38;
        let t195 = f64::sqrt(t194);
        let t196 = 1.0 / t195;
        let t199 = 1.0 / t194;
        let t202 = 0.12033e1 - 0.318038e0 * t37 * t196 + 0.1880816e-1 * t38 * t199;
        let t203 = t192 * t202;
        let tzk0 = -t51 + t154 + t203;
        zk[ip] += tzk0;
    }
}
